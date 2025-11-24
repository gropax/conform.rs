use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use thiserror::Error;
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CollectionRegistry {
    pub collections: HashMap<String, Collection>,
}

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("I/O error while accessing registry file: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML decoding error: {0}")]
    TomlDecode(#[from] toml::de::Error),

    #[error("TOML encoding error: {0}")]
    TomlEncode(#[from] toml::ser::Error),
}

pub fn load_registry(path: &Path) -> Result<CollectionRegistry, RegistryError> {
    if !path.exists() {
        return Ok(CollectionRegistry::default());
    }

    let content = fs::read_to_string(path)?;
    let registry = toml::from_str(&content)?;

    Ok(registry)
}

pub fn save_registry(path: &Path, registry: &CollectionRegistry) -> Result<(), RegistryError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let toml_string = toml::to_string_pretty(registry)?;

    let mut file = fs::File::create(path)?;
    file.write_all(toml_string.as_bytes())?;
    file.sync_all()?;

    Ok(())
}
