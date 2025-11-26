use std::path::PathBuf;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Scalar {
    Bool { span: Span, value: bool },
    Number { span: Span, value: f64 },
    String { span: Span, value: String },
}

impl Scalar {
    pub fn span(&self) -> Span {
        match *self {
            Scalar::Bool { span, value: _ } => span,
            Scalar::Number { span, value: _ } => span,
            Scalar::String { span, value: _ } => span,
        }
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scalar::Bool { span: _, value } => write!(f, "{value}"),
            Scalar::Number { span: _, value } => write!(f, "{value}"),
            Scalar::String { span: _, value } => write!(f, "\"{value}\""),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Single(Span, Scalar),
    List(Span, Vec<Scalar>),
    Invalid(Span),
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
            Value::Single(_, s) => ValueIter::Single(Some(s).into_iter()),
            Value::List(_, v) => ValueIter::List(v.iter()),
            Value::Invalid(_) => ValueIter::Invalid,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
    pub span: Span,
}
