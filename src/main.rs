mod cli;
mod commands;
mod schema;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use commands::check;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check {
            schema_file,
            verbose,
        } => {
            let args = check::Args {
                schema_file,
                verbose,
            };
            check::handle(&args)?;
            Ok(())
        }
    }
}
