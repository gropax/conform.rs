use super::{Document, Field, Scalar, Value};
use serde_yaml::Value as YamlValue;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;
use crate::document;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("YAML to JSON conversion error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Top-level JSON must be an object (mapping)")]
    RootNotObject,
}

pub fn parse_document(input: &str, file_path: &PathBuf) -> Result<Document, ParseError> {
    let yaml: YamlValue = serde_yaml::from_str(input)?;
    let json: JsonValue = serde_json::to_value(yaml)?;
    document::json::document(json, file_path)?
}
