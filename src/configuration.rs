use clap::{Parser};

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long, help = "Output file for stdout of program")]
    pub output_file: Option<String>,

    /// Arguments after `--` for another program
    #[arg(last = true, help = "Arguments after `--` will be passed to the loaded program")]
    pub loaded_prog_args: Vec<String>,
}