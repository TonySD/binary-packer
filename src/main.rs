use std::collections::HashMap;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::process::{Command, Stdio};
use tempfile::TempDir;
pub mod configuration;

// Include the generated code
include!("generated_assets.rs");

fn main() {
    let files: HashMap<&'static str, &'static [u8]> = get_embedded_files();
    let config = configuration::Cli::parse();

    let unpacked_program = match unpack_program(files) {
        Ok(unpacked_program) => unpacked_program,
        Err(e) => {
            eprintln!("Error unpacking program: {}", e);
            return;
        }
    };

    match run_program(unpacked_program, &config.loaded_prog_args, config.output_file.as_deref()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error running program: {}", e);
            return;
        }
    }
    
}

struct UnpackedProgram {
    executable_path: String,
    temp_dir: TempDir,
}

fn unpack_program(files: HashMap<&'static str, &'static [u8]>) -> Result<UnpackedProgram, io::Error> {
    let temp_dir = tempfile::Builder::new()
        .prefix("system-")
        .tempdir()?;

    let mut executable_counter = 0;
    let mut executable_path = String::new();
    for (filename, content) in &files {
        let file_path = temp_dir.path().join(filename);
        std::fs::write(&file_path, content)?;
        
        // Make executable files executable
        if filename.ends_with(EXECUTABLE_SUFFIX) {
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
    } else if executable_counter > 1 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Only one executable file is allowed"));
    }

    Ok(UnpackedProgram {
        executable_path,
        temp_dir,
    })
}

fn run_program(program: UnpackedProgram, args: &[String], stdout_file: Option<&str>) -> Result<(), io::Error> {
    let mut child = Command::new(program.executable_path)
        .args(args)
        .stdout(if stdout_file.is_some() { Stdio::piped() } else { Stdio::inherit() })
        .spawn()?;

    let status = child.wait()?;

    if stdout_file.is_some() && child.stdout.is_some() {
        let stdout = child.stdout.take().unwrap();
        let output_file = File::create(stdout_file.unwrap())?;
        let mut output_writer = BufWriter::new(output_file);
        let mut reader = BufReader::new(stdout);

        // Copy child stdout to file
        io::copy(&mut reader, &mut output_writer)?;
    }
    
    println!("Loaded program exited with status: {}", status);

    program.temp_dir.close()?;

    Ok(())
}
