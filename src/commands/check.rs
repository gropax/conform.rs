use crate::schema;
use crate::utils;
use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct CheckArgs {
    pub schema_file: PathBuf,
    pub verbose: u8,
}

pub fn handle_check(args: &CheckArgs) -> Result<()> {
    utils::init_tracing(args.verbose);

    let _schema = schema::load_schema(&args.schema_file)?;
    Ok(())
}
