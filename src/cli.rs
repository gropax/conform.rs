use clap::{ArgAction, Parser};
use std::path::{PathBuf};

#[derive(Debug, Parser)]
#[command(name = "conform")]
#[command(about = "A command line tool", long_about = None)]
pub struct Cli {
    pub schema_file: PathBuf,

    pub document_files: Vec<String>,

    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,
}
