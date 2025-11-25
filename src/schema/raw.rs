use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum RawMultiplicity {
    #[default]
    One,
    Many,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum RawField {
    Number {
        #[serde(default)]
        multiplicity: RawMultiplicity,
    },
    String {
        #[serde(default)]
        multiplicity: RawMultiplicity,

        #[serde(default)]
        starts_with: Option<String>,

        #[serde(default)]
        pattern: Option<String>,
    },
    Enum {
        values: Vec<String>,

        #[serde(default)]
        multiplicity: RawMultiplicity,
    },
    Url {
        #[serde(default)]
        multiplicity: RawMultiplicity,

        #[serde(default)]
        starts_with: Option<String>,

        #[serde(default)]
        pattern: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawSchema {
    pub name: String,
    pub fields: HashMap<String, RawField>,
}
