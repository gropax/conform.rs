use crate::document;
use crate::schema::Multiplicity;
use crate::validation::{FieldError, ScalarValidator, ValidationError};

pub struct FieldValidator {
    pub name: String,
    pub multiplicity: Multiplicity,
    pub rules: ScalarValidator,
}

impl FieldValidator {
    pub fn validate(&self, field: &document::Field) -> Option<FieldError> {
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
