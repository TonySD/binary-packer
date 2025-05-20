use std::collections::HashMap;
use clap::Parser;
use std::io::{self, BufReader, BufWriter};
use std::fs::File;
use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use zip::{ZipWriter, CompressionMethod, write::{FileOptions, SimpleFileOptions}, AesMode};
use walkdir::WalkDir;


pub mod configuration;

// Include the generated code
include!("generated_assets.rs");

const EXECUTABLE_SUFFIX: &[&str] = if cfg!(target_os = "windows") {
    &["exe", "bat"]
} else {
    &["elf", "sh"]
};

fn main() {

    #[cfg(feature = "only_unpacking")]
    let only_unpacking = true;
    #[cfg(not(feature = "only_unpacking"))]
    let only_unpacking = false;

    let files: HashMap<&'static str, &'static [u8]> = get_embedded_files();
    let config = configuration::Cli::parse();

    println!("Unpacking program...");
    let unpacked_program = match unpack_program(files) {
        Ok(unpacked_program) => unpacked_program,
        Err(e) => {
            println!("Error unpacking program: {}", e);
            return;}
    };

    // Just exit, if we need to only unpack the files
    if config.only_unpackage || only_unpacking {
        println!("Mode: only unpacking, temp dir was not deleted");
        return;
    }

    match run_program(&unpacked_program, &config.loaded_prog_args, &config) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error running program: {}", e);
            return;
        }
    }
}

struct UnpackedProgram {
    executable_path: String,
    temp_dir_path: std::path::PathBuf,
}

fn unpack_program(files: HashMap<&'static str, &'static [u8]>) -> Result<UnpackedProgram, io::Error> {
    let random_name = format!("{:x}", rand::random::<u64>());
    let temp_dir = std::env::temp_dir().join(random_name);
    std::fs::create_dir(&temp_dir)?;

    println!("Temp dir: {}", temp_dir.display());

    let mut executable_counter = 0;
    let mut executable_path = String::new();

    // Master is the executable, that will be launched after unpacking. 
    // If master is not found, the program will expect only one executable file in the assets dir.
    let mut master_found: bool = false;
    for (filename, content) in &files {
        println!("Unpacking file: {}", filename);
        let file_path = temp_dir.join(filename);
        std::fs::write(&file_path, content)?;
        
        // Make executable files executable
        if EXECUTABLE_SUFFIX.iter().any(|suffix| filename.ends_with(suffix)) && !master_found {
            if filename.starts_with("master") {
                master_found = true;
            }
            executable_counter += 1;
            executable_path = file_path.to_string_lossy().to_string();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&file_path, std::fs::Permissions::from_mode(0o755))?;
            }
        }
    }

    if executable_counter == 0 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No executable file found"));
    } else if executable_counter > 1 && !master_found {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Only one executable file is allowed. Name the executable file with prefix 'master', or use the --only-unpack flag"));
    } 


    Ok(UnpackedProgram {
        executable_path,
        temp_dir_path: temp_dir,
    })
}


fn run_program(program: &UnpackedProgram, args: &[String], config: &configuration::Cli) -> Result<(), io::Error> {
    let report_dir = match config.encrypt_report {
        Some(_) => {
            let report_dir = Path::new(&program.temp_dir_path).join("report");
            std::fs::create_dir_all(&report_dir)?;
            report_dir
        },
        None => Path::new(".").to_path_buf()
    };

    
    let mut child = Command::new(program.executable_path.clone())
        .args(args)
        .stdout(if config.binary_stdout_file.is_some() {Stdio::piped()} else {Stdio::inherit()})
        .current_dir(&report_dir)
        .spawn()?;

    if config.binary_stdout_file.is_some() && child.stdout.is_some() {
        let stdout = child.stdout.take().unwrap();
        let output_file = File::create(&report_dir.join(config.binary_stdout_file.clone().unwrap()))?;
        let mut output_writer = BufWriter::new(output_file);
        let mut reader = BufReader::new(stdout);

        // Copy child stdout to file
        io::copy(&mut reader, &mut output_writer)?;
    }

    let status = child.wait()?;
    
    println!("Loaded program finished, {}", status);

    if config.encrypt_report.is_some() {
        archive_dir(
            &program.temp_dir_path.join("report"), 
            &Path::new(&config.report_archive.clone().unwrap_or("./encrypted_report.zip".to_string())), 
            &config.encrypt_report.clone().unwrap()
        )?;
    }

    std::fs::remove_dir_all(&program.temp_dir_path)?;
    println!("Temp dir removed: {}", program.temp_dir_path.display());

    Ok(())
}

fn archive_dir(src_dir: &Path, dst_file: &Path, password: &str) -> Result<(), io::Error> {
    if !src_dir.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "source is not a directory"));
    }

    println!("Archiving directory: {}", src_dir.display());
    let writer = File::create(&dst_file)?;
    let mut zip = ZipWriter::new(writer);

    let base_components = src_dir.components().count();

    // Same options for every file/dir we add
    let opts = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .large_file(true)
        .with_aes_encryption(AesMode::Aes256, password); 

    for entry in WalkDir::new(src_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        let rel: PathBuf = path.components().skip(base_components).collect();

        if entry.file_type().is_dir() {
            zip.add_directory(rel.to_string_lossy(), opts)?;
        } else {
            zip.start_file(rel.to_string_lossy(), opts)?;
            io::copy(&mut File::open(path)?, &mut zip)?;
        }
    }

    zip.finish()?;
    Ok(())
}