use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum RawMultiplicity {
    #[default]
    One,
    Many,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum RawField {
    Integer {
        #[serde(default)]
        multiplicity: RawMultiplicity,
    },
    Number {
        #[serde(default)]
        multiplicity: RawMultiplicity,
    },
    String {
        #[serde(default)]
        pattern: Option<String>,

        #[serde(default)]
        multiplicity: RawMultiplicity,
    },
    Enum {
        values: Vec<String>,

        #[serde(default)]
        multiplicity: RawMultiplicity,
    },
    Url {
        #[serde(default)]
        starts_with: Option<String>,

        #[serde(default)]
        pattern: Option<String>,

        #[serde(default)]
        multiplicity: RawMultiplicity,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawSchema {
    pub name: String,
    pub fields: HashMap<String, RawField>,
}

#[derive(Error, Debug)]
pub enum TomlIOError {
    #[error("I/O error while accessing schema file: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML decoding error: {0}")]
    TomlDecode(#[from] toml::de::Error),

    #[error("TOML encoding error: {0}")]
    TomlEncode(#[from] toml::ser::Error),
}

pub fn load_schema(path: &Path) -> Result<RawSchema, TomlIOError> {
    let content = fs::read_to_string(path)?;
    let schema = toml::from_str(&content)?;
    Ok(schema)
}

pub fn save_schema(path: &Path, schema: &RawSchema) -> Result<(), TomlIOError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let toml_string = toml::to_string_pretty(schema)?;

    let mut file = fs::File::create(path)?;
    file.write_all(toml_string.as_bytes())?;
    file.sync_all()?;

    Ok(())
}
