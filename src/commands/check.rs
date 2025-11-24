use crate::schema;
use crate::utils;
use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct Args {
    pub schema_file: PathBuf,
    pub verbose: u8,
}

pub fn handle(args: &Args) -> Result<()> {
    utils::init_tracing(args.verbose);

    let _schema = schema::load_schema(&args.schema_file)?;
    info!("Loaded schema from file: {}", args.schema_file.display());

    Ok(())
}
