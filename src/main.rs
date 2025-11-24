mod cli;
mod commands;
mod schema;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use commands::{CheckArgs, handle_check};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check {
            schema_file,
            verbose,
        } => {
            let args = CheckArgs {
                schema_file,
                verbose,
            };
            handle_check(&args)?;
            Ok(())
        }
    }
}
