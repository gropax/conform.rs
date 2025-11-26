use std::path::PathBuf;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Scalar {
    Bool(bool),
    Number(f64),
    String(String),
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scalar::Bool(v) => write!(f, "{v}"),
            Scalar::Number(v) => write!(f, "{v}"),
            Scalar::String(v) => write!(f, "\"{v}\""),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Single(Scalar),
    List(Vec<Scalar>),
    Invalid,
}

pub enum ValueIter<'a> {
    Single(std::option::IntoIter<&'a Scalar>),
    List(std::slice::Iter<'a, Scalar>),
    Invalid,
}

impl<'a> Iterator for ValueIter<'a> {
    type Item = &'a Scalar;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ValueIter::Single(iter) => iter.next(),
            ValueIter::List(iter) => iter.next(),
            ValueIter::Invalid => None,
        }
    }
}

impl Value {
    pub fn iter(&self) -> ValueIter<'_> {
        match self {
            Value::Single(s) => ValueIter::Single(Some(s).into_iter()),
            Value::List(v) => ValueIter::List(v.iter()),
            Value::Invalid => ValueIter::Invalid,
        }
    }
}

#[derive(Debug)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct FieldKey {
    pub name: String,
    pub span: Span,
}

#[derive(Debug)]
pub struct Field {
    pub key: FieldKey,
    pub value: Value,
}

#[derive(Debug)]
pub struct Document {
    pub file_path: PathBuf,
    pub fields: HashMap<String, Field>,
}
