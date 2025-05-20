# Executable loader
## Idea
Universal single-file loader, that unpacks all files from assets/ dir to %TEMP% and launches executable file

## Compilation
### Embed files
Add all files to assets dir and compile by cargo. Script build.rs while compilation will automatically detect all the files (except .gitkeep) and include it inside the binary.

### Linux platform
For compiling to linux, you have to change [this line](.cargo/config.toml#L8) to `target = "x86_64-unknown-linux-gnu"`

### Windows platform
Change [this line](.cargo/config.toml#L8) to `target = "x86_64-pc-windows-msvc"`

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

## Usage
### Plugin arguments
Loader have its own arguments. To access it, just pass it to command line. \
For passing arguments to loaded binary/script, pass args after `--`:
```bash
plugin --loader-arg -- --loaded-binary-arg
```

### Only unpack files via CLI flags (even if builded without feature)
```bash
plugin --only-unpackage
```

### Encrypting reports
By default, loaded binary is launched from your current directory. However some binary produces sensitive files, so loader can encrypt the whole report dir. \
To do this, you have to pass `--encrypt-report` flag. After that, loaded binary will be launched in %TEMPDIR%/report directory, so your responsibility - make loaded binary output all of the files to current directory, for example with passing `--output-dir .` \
If you want to capture output, pass the flag `--output-file`. Don't use absolute path here! Just name the file, it will be placed in %TEMPDIR%/report directory also. \
So the full command will be:
```bash
loader --output-file binary_stdout.txt --encrypt-report password -- --mode All
```
