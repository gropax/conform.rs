use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("YAML→JSON: {0}")]
    YamlToJson(serde_json::Error),

    #[error("Top-level YAML must be a mapping (object)")]
    TopLevelNotObject,

    #[error("Invalid value for field `{0}`")]
    InvalidValue(String),
}
