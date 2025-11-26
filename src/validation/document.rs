use crate::document;
use crate::validation::{DocumentError, FieldError, FieldValidator, ValidationError};
use std::collections::HashSet;

pub struct DocumentValidator {
    pub field_names: HashSet<String>,
    pub fields: Vec<FieldValidator>,
}

impl DocumentValidator {
    pub fn validate(&self, document: &document::Document) -> Option<DocumentError> {
        let mut fields = vec![];

        for field_validator in &self.fields {
            if let Some(document_field) = document.fields.get(&field_validator.name) {
                if let Some(field_error) = field_validator.validate(document_field) {
                    fields.push(field_error)
                }
            } else {
                fields.push(FieldError {
                    field_name: field_validator.name.to_string(),
                    errors: vec![ValidationError {
                        message: format!("field [{}] is missing", field_validator.name),
                        span: document.span.clone(),
                    }],
                    values: vec![],
                })
            }
        }

        let mut errors = vec![];

        for field in document.fields.values() {
            if !self.field_names.contains(&field.key.name) {
                errors.push(ValidationError {
                    message: format!("field [{}] is unknown", field.key.name),
                    span: field.key.span.clone(),
                })
            }
        }

        if fields.is_empty() {
            None
        } else {
            Some(DocumentError {
                file: document.span.file.to_string(),
                errors,
                fields,
            })
        }
    }
}
