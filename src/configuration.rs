use clap::{Parser};

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long, help = "Output file for stdout of program (not implemented)")]
    pub output_file: Option<String>,

    #[arg(short = 'u', long, help = "Only unpackage the input files, don't execute anything and don't delete temporary folder")]
    pub only_unpackage: bool,

    /// Arguments after `--` for another program
    #[arg(last = true, help = "Arguments after `--` will be passed to the loaded program")]
    pub loaded_prog_args: Vec<String>,
}