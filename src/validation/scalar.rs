use crate::document;
use crate::validation::{ValidateNumber, ValidateString, ValidationError, ValueError};

pub enum ScalarValidator {
    Bool,
    Number(Vec<Box<dyn ValidateNumber>>),
    String(Vec<Box<dyn ValidateString>>),
}

impl ScalarValidator {
    pub fn validate(&self, index: usize, value: &document::Scalar) -> Option<ValueError> {
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
