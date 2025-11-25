use std::path::{Path};
use std::fs;
use super::{Schema, SchemaError, raw::RawSchema};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchemaIOError {
    #[error("I/O error while accessing schema file: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML decoding error: {0}")]
    TomlDecode(#[from] toml::de::Error),

    #[error("TOML encoding error: {0}")]
    TomlEncode(#[from] toml::ser::Error),

    #[error("Invalid schema: {0}")]
    Schema(#[from] SchemaError),
}

pub fn load_schema(path: &Path) -> Result<Schema, SchemaIOError> {
    let content = fs::read_to_string(path)?;
    let raw_schema: RawSchema = toml::from_str(&content)?;
    let schema = Schema::try_from(raw_schema)?;
    Ok(schema)
}

//pub fn save_schema(path: &Path, schema: &Schema) -> Result<(), SchemaIOError> {
//    if let Some(parent) = path.parent() {
//        fs::create_dir_all(parent)?;
//    }
//
//    let toml_string = toml::to_string_pretty(schema)?;
//
//    let mut file = fs::File::create(path)?;
//    file.write_all(toml_string.as_bytes())?;
//    file.sync_all()?;
//
//    Ok(())
//}
