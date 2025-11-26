use crate::validation::{
    ValidationError, ValueError, FieldError, DocumentError,
    ValidateNumber, ValidateString,
};
use crate::document;
use crate::schema::Multiplicity;
use std::collections::HashSet;

pub enum ScalarValidator {
    Bool,
    Number(Vec<Box<dyn ValidateNumber>>),
    String(Vec<Box<dyn ValidateString>>),
}

impl ScalarValidator {
    fn validate(&self, index: usize, value: &document::Scalar) -> Option<ValueError> {
        let mut errors = vec![];

        match self {
            ScalarValidator::Bool => {
                if let document::Scalar::Bool { span: _, value: _ } = value {
                } else {
                    errors.push(ValidationError {
                        message: format!("{} is not a boolean", value),
                        span: value.span(),
                    })
                }
            }

            ScalarValidator::Number(validators) => {
                if let document::Scalar::Number { span, value } = value {
                    errors.extend(
                        validators
                            .iter()
                            .filter_map(|v| v.validate(span.clone(), *value))
                            .collect::<Vec<_>>(),
                    );
                } else {
                    errors.push(ValidationError {
                        message: format!("{} is not a number", value),
                        span: value.span(),
                    })
                }
            }

            ScalarValidator::String(validators) => {
                if let document::Scalar::String { span, value } = value {
                    errors.extend(
                        validators
                            .iter()
                            .filter_map(|v| v.validate(span.clone(), value))
                            .collect::<Vec<_>>(),
                    );
                } else {
                    errors.push(ValidationError {
                        message: format!("{} is not a string", value),
                        span: value.span(),
                    })
                }
            }
        };

        if errors.is_empty() {
            None
        } else {
            Some(ValueError {
                value_index: index,
                errors,
            })
        }
    }
}

pub struct FieldValidator {
    pub name: String,
    pub multiplicity: Multiplicity,
    pub rules: ScalarValidator,
}

impl FieldValidator {
    fn validate(&self, field: &document::Field) -> Option<FieldError> {
        let mut errors = vec![];
        let value = &field.value;

        if let Some(mult_error) = self.validate_multiplicity(value) {
            errors.push(mult_error);
        }

        let values = value
            .iter()
            .enumerate()
            .filter_map(|(i, s)| self.rules.validate(i, s))
            .collect::<Vec<_>>();

        if errors.is_empty() && values.is_empty() {
            None
        } else {
            Some(FieldError {
                field_name: self.name.to_string(),
                errors,
                values,
            })
        }
    }

    fn validate_multiplicity(&self, value: &document::Value) -> Option<ValidationError> {
        match self.multiplicity {
            Multiplicity::One => {
                if let document::Value::List { span, value: _ } = value {
                    Some(ValidationError {
                        message: "value is not a scalar".to_string(),
                        span: span.clone(),
                    })
                } else {
                    None
                }
            }
            Multiplicity::Many => {
                if let document::Value::Single { span, value: _ } = value {
                    Some(ValidationError {
                        message: "value is not a list".to_string(),
                        span: span.clone(),
                    })
                } else {
                    None
                }
            }
        }
    }
}

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
