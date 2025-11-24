mod cli;
//mod collections;
mod utils;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{CheckArgs, handle_check};

fn main() {
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

            handle_check(&args);
        }
    }
}
