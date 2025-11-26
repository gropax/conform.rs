use crate::document;
use crate::schema;
use crate::schema::Multiplicity;
use crate::utils;
use crate::validation;
use crate::validation::FlattenErrors;
use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct Args {
    pub schema_file: PathBuf,
    pub document_file: PathBuf,
    pub verbose: u8,
}

#[derive(Debug)]
pub struct ValidationError {
    pub document: PathBuf,
    pub field_name: String,
    pub error: ValidationErrorType,
}

#[derive(Debug)]
pub enum ValidationErrorType {
    MissingField {},
    MultiplicityMismatch {
        expected: Multiplicity,
        found: Multiplicity,
    },
}

pub fn handle(args: &Args) -> Result<()> {
    utils::init_tracing(args.verbose);

    let schema = schema::load_schema(&args.schema_file)?;
    info!("Loaded schema from file: {}", args.schema_file.display());

    let validator = schema.compile();
    info!("Compiled schema into validator");

    let document = document::parse(&args.document_file)?;
    info!("Loaded document from file: {}", args.document_file.display());

    let document_errors = validator.validate(&document);
    info!("Validated document");

    let errors = document_errors.as_ref()
        .map_or(Vec::new(), |e| e.flatten());

    let quickfix = validation::errors_to_quickfix(errors);
    print!("{}", quickfix);

    Ok(())
}
