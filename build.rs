use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let dest_path = Path::new("src").join("generated_assets.rs");
    let mut file = File::create(&dest_path).unwrap();

    let asset_dir = "assets";

    writeln!(file, "pub fn get_embedded_files() -> HashMap<&'static str, &'static [u8]> {{").unwrap();
    writeln!(file, "    let mut map = std::collections::HashMap::new();").unwrap();

    for entry in fs::read_dir(asset_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap() != ".gitkeep" {
            let filename = path.file_name().unwrap().to_string_lossy();
            let rel_path = path.to_str().unwrap().replace("\\", "/");
            writeln!(file, "    map.insert(\"{}\", include_bytes!(\"../{}\") as &'static [u8]);", filename, rel_path).unwrap();
        }
    }

    writeln!(file, "    map").unwrap();
    writeln!(file, "}}").unwrap();
    println!("cargo:rerun-if-changed=assets");
}