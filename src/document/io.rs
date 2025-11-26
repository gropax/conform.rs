use super::{Document, Field, FieldKey, Value, Scalar, Span};
use super::convert::{DocumentError};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;
use saphyr::{LoadableYamlNode, MarkedYaml, YamlData};
use std::collections::HashMap;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unsupported file extension: {0}")]
    UnsupportedExtension(String),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("YAML parse error: {0}")]
    Yaml2(#[from] saphyr::ScanError),

    #[error("Can't get YAML frontmatter")]
    Markdown,

    #[error("Document structure error: {0}")]
    Document(#[from] DocumentError),
}

//pub fn parse_json(input: &str, file_path: &PathBuf) -> Result<Document, LoadError> {
//    let json: JsonValue = serde_json::from_str(input)?;
//    let document = document(json, file_path)?;
//    Ok(document)
//}

//pub fn parse_yaml(input: &str, file_path: &PathBuf) -> Result<Document, LoadError> {
//    let yaml: YamlValue = serde_yaml::from_str(input)?;
//    let json: JsonValue = serde_json::to_value(yaml)?;
//    let document = document(json, file_path)?;
//    Ok(document)
//}

fn extract_frontmatter(input: &str) -> Option<&str> {
    let rest = input.strip_prefix("---\n")?;
    let end = rest.find("\n---\n")?;
    let (front, _) = rest.split_at(end);
    Some(front)
}

//pub fn parse_markdown(input: &str, file_path: &PathBuf) -> Result<Document, LoadError> {
//    let frontmatter = extract_frontmatter(input).ok_or(LoadError::Markdown)?;
//    parse_yaml(frontmatter, file_path)
//}

pub fn parse(file_path: impl Into<PathBuf>) -> Result<Document, LoadError> {
    let path = file_path.into();
    let input = fs::read_to_string(&path)?;

    match path.extension().and_then(|ext| ext.to_str()) {
        //Some("json") => parse_json(&input, &path),
        Some("yaml") | Some("yml") => parse_yaml(&input, &path),
        //Some("md") | Some("markdown") => parse_markdown(&input, &path),

        other => Err(LoadError::UnsupportedExtension(
            other.unwrap_or("").to_string(),
        )),
    }
}

// YAML

fn yaml_to_scalar(v: &MarkedYaml) -> Option<Scalar> {
    let span = yaml_to_span(v);

    match &v.data {
        YamlData::Value(s) => match s {
            saphyr::Scalar::Boolean(b) => Some(Scalar::Bool { span, value: *b }),
            saphyr::Scalar::Integer(i) => Some(Scalar::Number { span, value: *i as f64 }),
            saphyr::Scalar::FloatingPoint(f) => Some(Scalar::Number { span, value: f64::from(*f) }),
            saphyr::Scalar::String(s) => Some(Scalar::String { span, value: s.to_string() }),
            _ => None,
        },
        _ => None,
    }
}

fn yaml_to_value(v: &MarkedYaml) -> Value {
    let span = yaml_to_span(v);

    if let Some(scalar) = yaml_to_scalar(v) {
        Value::Single { span, value: scalar }
    } else {
        match &v.data {
            YamlData::Sequence(arr) => {
                let mut scalars = vec![];

                for item in arr {
                    match yaml_to_scalar(item) {
                        Some(s) => scalars.push(s),
                        None => return Value::Invalid { span },
                    }
                }

                Value::List { span, value: scalars }
            }
            _ => Value::Invalid { span },
        }
    }
}

fn yaml_to_key(k: &MarkedYaml) -> Result<FieldKey, DocumentError> {
    let name = match k.data.as_str() {
        Some(s) => s,
        None => return Err(DocumentError::NonStringKey),
    };

    let key = FieldKey {
        name: name.to_string(),
        span: yaml_to_span(k),
    };

    Ok(key)
}

fn yaml_to_span(v: &MarkedYaml) -> Span {
    Span {
        line: v.span.start.line(),
        column: v.span.start.col(),
    }
}

pub fn yaml_to_document(root: &MarkedYaml, file_path: &PathBuf) -> Result<Document, DocumentError> {
    let object = match root.data.as_mapping() {
        Some(obj) => obj,
        None => return Err(DocumentError::RootNotObject)
    };

    let mut fields = HashMap::new();

    for (k, v) in object.iter() {
        let key = yaml_to_key(k)?;
        let value = yaml_to_value(v);

        fields.insert(
            key.name.to_string(),
            Field {
                key,
                value,
            },
        );
    }

    Ok(Document {
        file_path: file_path.clone(),
        fields,
        span: yaml_to_span(root),
    })
}

pub fn parse_yaml(input: &str, file_path: &PathBuf) -> Result<Document, LoadError> {
    let yaml_docs = MarkedYaml::load_from_str(input)?;
    // Error in multiple documents
    let yaml_doc = &yaml_docs[0];
    let document = yaml_to_document(yaml_doc, file_path)?;
    Ok(document)
}
