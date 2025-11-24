use std::path::{PathBuf};
use tracing::{debug, error, info, warn};
use crate::utils;

#[derive(Debug)]
pub struct CheckArgs {
    pub schema_file: PathBuf,
    pub verbose: u8,
}

pub fn handle_check(args: &CheckArgs) {
    utils::init_tracing(args.verbose);

    // TODO
}
