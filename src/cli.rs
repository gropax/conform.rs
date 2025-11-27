use clap::{ArgAction, Parser, ColorChoice};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "conform",
    version,
    author,
    about = "Validate YAML documents using data schemas.",
    long_about = None,
    color = ColorChoice::Auto,
)]
pub struct Cli {
    #[arg(
        value_name = "SCHEMA",
        help = "Path to the schema definition file in TOML format.",
    )]
    pub schema_file: PathBuf,

    #[arg(
        value_name = "DOCUMENTS",
        long_help = "Paths to the documents to validate. Glob patterns are supported.\n\
                     The following document types are accepted:\n\
                     - YAML\n\
                     - Markdown with YAML frontmatter",
    )]
    pub document_patterns: Vec<String>,

    #[arg(
        short,
        long,
        action = ArgAction::Count,
        long_help = "Control the amount of logging:\n\
                     - no flag: warnings\n\
                     - -v     : info\n\
                     - -vv    : debug"
    )]
    pub verbose: u8,

    #[arg(
        short,
        long,
        help = "Only show errors, remove warnings",
    )]
    pub quiet: bool,
}
