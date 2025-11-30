mod cli;
mod commands;
mod document;
mod schema;
mod utils;
mod validation;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use commands::conform;

fn main() -> Result<()> {
    let cli = Cli::parse();

    utils::init_logging(cli.quiet, cli.verbose);

    let args = conform::Args {
        schema_file: cli.schema_file,
        document_patterns: cli.document_patterns,
    };

    conform::handle(&args)?;

    Ok(())
}
