use std::collections::HashMap;

#[derive(Debug, Default)]
pub enum Multiplicity {
    #[default]
    One,
    Many,
}

#[derive(Debug)]
pub enum FieldType {
    Bool,
    Number,
    String(Vec<StringContraint>),
}

#[derive(Debug)]
pub enum NumberContraint {
    Above(f64),
    Below(f64),
}

#[derive(Debug)]
pub enum StringContraint {
    Url,
    Enum(Vec<String>),
    Pattern(String),
    StartsWith(String),
    EndsWith(String),
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub multiplicity: Multiplicity,
    pub r#type: FieldType,
}

#[derive(Debug)]
pub struct Schema {
    pub name: String,
    pub fields: HashMap<String, Field>,
}
