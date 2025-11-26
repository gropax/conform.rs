use regex::Regex;

use crate::validate::DocumentValidator;

#[derive(Debug, Clone, Copy, Default)]
pub enum Multiplicity {
    #[default]
    One,
    Many,
}

#[derive(Debug)]
pub enum FieldType {
    Bool,
    Number(Vec<NumberConstraint>),
    String(Vec<StringConstraint>),
}

#[derive(Debug)]
pub enum NumberConstraint {
    Above(f64),
    Below(f64),
}

#[derive(Debug)]
pub enum StringConstraint {
    Url,
    Enum(Vec<String>),
    Pattern(String, Regex),
    StartsWith(String),
    EndsWith(String),
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub multiplicity: Multiplicity,
    pub kind: FieldType,
}

#[derive(Debug)]
pub struct Schema {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Schema {
    pub fn compile(&self) -> DocumentValidator {
        DocumentValidator::from(self)
    }
}
