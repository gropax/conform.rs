mod cli;
mod commands;
mod schema;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use commands::apply;
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

        Commands::Apply {
            schema_file,
            document_file,
            verbose,
        } => {
            let args = apply::Args {
                schema_file,
                document_file,
                verbose,
            };
            apply::handle(&args)?;
            Ok(())
        }
    }
}
