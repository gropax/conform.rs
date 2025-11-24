use clap::{ArgAction, Parser, Subcommand};
use std::path::{PathBuf};

#[derive(Debug, Parser)]
#[command(name = "conform")]
#[command(about = "A command line tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Check {
        schema_file: PathBuf,

        #[arg(short, long, action = ArgAction::Count)]
        verbose: u8,
    },
}
