# Executable loader
## Idea
Universal single-file loader, that unpacks all files from assets/ dir to %TEMP% and launches executable file

## Usage
Add all files to assets dir and compile by cargo. Script build.rs while compilation will automatically detect all the files (except .gitkeep) and include it inside the binary.

## Compilation
### Executable mode
```bash
cargo build --release
```
And find your binary in target/ folder. If you need to embed more than 1 executable file, rename 1 of them to "master.exe", and only it will be launched.
### Only unpacking mode
```bash
cargo build --features only_unpacking --release
```
All your files will be unpacked to temporary folder