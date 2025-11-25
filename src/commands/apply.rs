use crate::document;
use crate::schema;
use crate::schema::Multiplicity;
use crate::utils;
use anyhow::Result;
use serde_yaml::Value;
use std::path::{Path, PathBuf};
use thiserror::Error;
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

    let content = std::fs::read_to_string(&args.document_file)?;

    let parse_result = document::json::parse_document(&content, &args.document_file);
    match parse_result {
        Ok(document) => {
            let errors = validator.validate(&document);
            info!("Error computed !!!")
        }
        Err(error) => match error {
            document::json::ParseError::Json(e) => error!("Invalid JSON"),
            document::json::ParseError::RootNotObject => error!("Document root is not an object"),
        }
    }

    Ok(())
}
