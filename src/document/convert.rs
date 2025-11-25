use super::{Document, Field, Scalar, Value};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DocumentError {
    #[error("Top-level JSON must be an object (mapping)")]
    RootNotObject,
}

pub fn document(root: JsonValue, file_path: &PathBuf) -> Result<Document, DocumentError> {
    let object = root
        .as_object()
        .ok_or(DocumentError::RootNotObject)
        .unwrap();

    let mut fields = HashMap::new();

    for (key, val) in object.iter() {
        let value = json_to_value(val);

        fields.insert(
            key.clone(),
            Field {
                name: key.clone(),
                value,
            },
        );
    }

    Ok(Document {
        file_path: file_path.clone(),
        fields,
    })
}

fn json_to_value(v: &JsonValue) -> Value {
    if let Some(scalar) = json_to_scalar(v) {
        Value::Single(scalar)
    } else {
        match v {
            JsonValue::Array(arr) => {
                let mut scalars = vec![];

                for item in arr {
                    match json_to_scalar(item) {
                        Some(s) => scalars.push(s),
                        None => return Value::Invalid,
                    }
                }

                Value::List(scalars)
            }
            _ => Value::Invalid,
        }
    }
}

fn json_to_scalar(v: &JsonValue) -> Option<Scalar> {
    match v {
        JsonValue::Bool(b) => Some(Scalar::Bool(*b)),
        JsonValue::Number(n) => Some(Scalar::Number(n.as_f64()?)),
        JsonValue::String(s) => Some(Scalar::String(s.clone())),
        _ => None,
    }
}
