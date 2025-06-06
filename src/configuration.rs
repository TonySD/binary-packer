use clap::{Parser};

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short = 's', long="stdout-file", help = "Output file for stdout of program")]
    pub binary_stdout_file: Option<String>,

    #[arg(short = 'u', long = "only-unpack", help = "Only unpack the input files, don't execute anything and don't delete temporary folder", default_value_t = false)]
    pub only_unpackage: bool,

    #[arg(short = 'e', long, help = "Specify the key. Loaded binary will be launched from %TEMPDIR%, and you need to specify for loaded program the output directory ./report. After launch this directory will be archived and encrypted.")]
    pub encrypt_report: Option<String>,

    #[arg(short = 'o', long = "output-archive", help = "Specify the output archive path for encrypted report. Default is ./encrypted_report.zip")]
    pub report_archive: Option<String>,

    /// Arguments after `--` for another program
    #[arg(last = true, help = "Arguments after `--` will be passed to the loaded program")]
    pub loaded_prog_args: Vec<String>,
}