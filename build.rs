// build.rs
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() {
    let dest_path = Path::new("src").join("generated_assets.rs");
    let mut out = File::create(&dest_path)
        .expect("❌  could not create generated_assets.rs");

    let asset_root = Path::new("assets");

    // ---- file prelude -----------------------------------------------------
    writeln!(
        out,
        "// ---------------  AUTO-GENERATED — do not edit  ---------------\n\
         use include_bytes_zstd::include_bytes_zstd;\n\
         \n\
         pub fn get_embedded_files() -> std::collections::HashMap<&'static str, Vec<u8>> {{"
    )
    .unwrap();
    writeln!(out, "    let mut map = std::collections::HashMap::new();").unwrap();

    // ---- walk assets/ recursively ----------------------------------------
    walk_dir(asset_root, asset_root, &mut out);

    // ---- file postlude ----------------------------------------------------
    writeln!(out, "    map").unwrap();
    writeln!(out, "}}").unwrap();

    // Re-run if *anything* under assets/ changes.
    println!("cargo:rerun-if-changed=assets");
}

/// Recursively visit `dir`, emitting one map.insert(..) per file.
fn walk_dir(dir: &Path, base: &Path, out: &mut File) {
    for entry in fs::read_dir(dir).expect("❌  failed to read dir") {
        let entry = entry.expect("❌  failed to read entry");
        let path = entry.path();

        if path.is_dir() {
            walk_dir(&path, base, out);
        } else if path.is_file()
            && path
                .file_name()
                .and_then(|n| n.to_str())
                .map_or(true, |n| n != ".gitkeep")
        {
            let rel_key = path
                .strip_prefix(base)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            let inc_path = path.to_string_lossy().replace('\\', "/");

            //                key in HashMap               include_bytes_zstd! macro
            writeln!(
                out,
                "    map.insert(\"{}\", include_bytes_zstd!(\"./{}\", 19));",
                rel_key, inc_path
            )
            .unwrap();

            
        }
    }
    println!("cargo:rerun-if-changed=assets");
}
