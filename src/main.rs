mod cli;
mod commands;
mod schema;
mod document;
mod validation;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use commands::conform;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let files = utils::expand_globs(&cli.document_files)?;

    let args = conform::Args {
        schema_file: cli.schema_file,
        document_files: files,
        verbose: cli.verbose,
    };

    conform::handle(&args)?;

    Ok(())
}
