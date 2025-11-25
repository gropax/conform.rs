use crate::schema;
use crate::schema::Multiplicity;
use crate::utils;
use anyhow::Result;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info, warn};
use serde_yaml::Value;
use thiserror::Error;

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
    MissingField { },
    MultiplicityMismatch {
        expected: Multiplicity,
        found: Multiplicity,
    },
}

pub fn handle(args: &Args) -> Result<()> {
    utils::init_tracing(args.verbose);

    //let schema = schema::load_schema(&args.schema_file)?;
    //info!("Loaded schema from file: {}", args.schema_file.display());

    //let document = load_document(&args.document_file)?;
    //let mut validation_errors = Vec::<ValidationError>::new();

    //for (field_name, field_type) in &schema.fields {
    //    if let Some(field_value) = document.get(field_name) {
    //    } else {
    //        validation_errors.push(ValidationError {
    //            document: args.document_file.clone(),
    //            field_name: field_name.to_string(),
    //            error: ValidationErrorType::MissingField { },
    //        });
    //    }
    //}

    Ok(())
}

#[derive(Error, Debug)]
pub enum YamlIOError {
    #[error("I/O error while accessing schema file: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML decoding error: {0}")]
    YamlDecode(#[from] serde_yaml::Error),

    //#[error("TOML encoding error: {0}")]
    //YamlEncode(#[from] toml::ser::Error),
}

pub fn load_document(path: &Path) -> Result<Value, YamlIOError> {
    let content =  std::fs::read_to_string(path)?;
    let value: Value = serde_yaml::from_str(&content)?;
    Ok(value)
}
