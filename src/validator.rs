use crate::document;
use crate::schema::{Field, FieldType, Multiplicity, NumberConstraint, Schema, StringConstraint};
use regex::Regex;
use std::path::PathBuf;
use url::Url;

#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
}

#[derive(Debug)]
pub struct ValueError {
    pub value_index: usize,
    pub errors: Vec<ValidationError>,
}

#[derive(Debug)]
pub struct FieldError {
    pub field_name: String,
    pub errors: Vec<ValidationError>,
    pub values: Vec<ValueError>,
}

#[derive(Debug)]
pub struct DocumentError {
    pub file_path: PathBuf,
    pub errors: Vec<ValidationError>,
    pub fields: Vec<FieldError>,
}

trait ValidateNumber {
    fn validate(&self, value: f64) -> Option<ValidationError>;
}

trait ValidateString {
    fn validate(&self, value: &str) -> Option<ValidationError>;
}

struct AboveValidator {
    min: f64,
}

impl ValidateNumber for AboveValidator {
    fn validate(&self, value: f64) -> Option<ValidationError> {
        (value < self.min).then(|| ValidationError {
            message: format!("Expected number >= {}, found {}", self.min, value),
        })
    }
}

struct BelowValidator {
    max: f64,
}

impl ValidateNumber for BelowValidator {
    fn validate(&self, value: f64) -> Option<ValidationError> {
        (value > self.max).then(|| ValidationError {
            message: format!("Expected number <= {}, found {}", self.max, value),
        })
    }
}

struct UrlValidator {}

impl ValidateString for UrlValidator {
    fn validate(&self, value: &str) -> Option<ValidationError> {
        Url::parse(value).err().map(|_| ValidationError {
            message: format!("\"{}\" is not an URL", value),
        })
    }
}

struct EnumValidator {
    values: Vec<String>,
}

impl ValidateString for EnumValidator {
    fn validate(&self, value: &str) -> Option<ValidationError> {
        (!self.values.iter().any(|v| v == value)).then(|| ValidationError {
            message: format!(
                "\"{}\" don't match any of [{}]",
                value,
                self.values.join(", ")
            ),
        })
    }
}

struct StartsWithValidator {
    prefix: String,
}

impl ValidateString for StartsWithValidator {
    fn validate(&self, value: &str) -> Option<ValidationError> {
        (!value.starts_with(&self.prefix)).then(|| ValidationError {
            message: format!("\"{}\" don't start with \"{}\"", value, self.prefix),
        })
    }
}

struct EndsWithValidator {
    suffix: String,
}

impl ValidateString for EndsWithValidator {
    fn validate(&self, value: &str) -> Option<ValidationError> {
        (!value.ends_with(&self.suffix)).then(|| ValidationError {
            message: format!("\"{}\" don't ends with \"{}\"", value, self.suffix),
        })
    }
}

struct RegexValidator {
    pattern: String,
    regex: Regex,
}

impl ValidateString for RegexValidator {
    fn validate(&self, value: &str) -> Option<ValidationError> {
        (!self.regex.is_match(value)).then(|| ValidationError {
            message: format!("\"{}\" don't match pattern /{}/", value, self.pattern),
        })
    }
}

enum ValueValidator {
    Bool,
    Number(Vec<Box<dyn ValidateNumber>>),
    String(Vec<Box<dyn ValidateString>>),
}

impl ValueValidator {
    fn validate(&self, index: usize, value: &document::Scalar) -> Option<ValueError> {
        let mut errors = vec![];

        match self {
            ValueValidator::Bool => {
                if let document::Scalar::Bool(_) = value {
                } else {
                    errors.push(ValidationError {
                        message: format!("{} is not a boolean", value),
                    })
                }
            }

            ValueValidator::Number(validators) => {
                if let document::Scalar::Number(number) = value {
                    errors.extend(validators
                        .iter()
                        .filter_map(|v| v.validate(*number))
                        .collect::<Vec<_>>());
                } else {
                    errors.push(ValidationError {
                        message: format!("{} is not a number", value),
                    })
                }
            }

            ValueValidator::String(validators) => {
                if let document::Scalar::String(string) = value {
                    errors.extend(validators
                        .iter()
                        .filter_map(|v| v.validate(string))
                        .collect::<Vec<_>>());
                } else {
                    errors.push(ValidationError {
                        message: format!("{} is not a string", value),
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

struct FieldValidator {
    name: String,
    multiplicity: Multiplicity,
    rules: ValueValidator,
}

impl FieldValidator {
    fn validate(&self, value: &document::Value) -> Option<FieldError> {
        let mut errors = vec![];

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
                if let document::Value::List(_) = value {
                    Some(ValidationError {
                        message: "value is not a scalar".to_string(),
                    })
                } else {
                    None
                }
            }
            Multiplicity::Many => {
                if let document::Value::Single(_) = value {
                    Some(ValidationError {
                        message: "value is not a list".to_string(),
                    })
                } else {
                    None
                }
            }
        }
    }
}

fn number_validator(constraint: &NumberConstraint) -> Box<dyn ValidateNumber> {
    match *constraint {
        NumberConstraint::Above(min) => Box::new(AboveValidator { min }),
        NumberConstraint::Below(max) => Box::new(BelowValidator { max }),
    }
}

fn string_validator(constraint: &StringConstraint) -> Box<dyn ValidateString> {
    match constraint {
        StringConstraint::Url => Box::new(UrlValidator {}),

        StringConstraint::Enum(values) => Box::new(EnumValidator {
            values: values.iter().map(|s| s.to_string()).collect(),
        }),

        StringConstraint::StartsWith(prefix) => Box::new(StartsWithValidator {
            prefix: prefix.to_string(),
        }),

        StringConstraint::EndsWith(suffix) => Box::new(EndsWithValidator {
            suffix: suffix.to_string(),
        }),

        StringConstraint::Pattern(pattern, regex) => Box::new(RegexValidator {
            pattern: pattern.to_string(),
            regex: regex.clone(),
        }),
    }
}

fn value_validator(field_type: &FieldType) -> ValueValidator {
    match field_type {
        FieldType::Bool => ValueValidator::Bool,

        FieldType::Number(constraints) => {
            let validators = constraints.iter().map(|c| number_validator(c)).collect();

            ValueValidator::Number(validators)
        }

        FieldType::String(constraints) => {
            let validators = constraints.iter().map(|c| string_validator(c)).collect();

            ValueValidator::String(validators)
        }
    }
}

impl From<&Field> for FieldValidator {
    fn from(field: &Field) -> Self {
        let rules = value_validator(&field.kind);

        FieldValidator {
            name: field.name.to_string(),
            multiplicity: field.multiplicity,
            rules,
        }
    }
}

pub struct Validator {
    fields: Vec<FieldValidator>,
}

impl From<&Schema> for Validator {
    fn from(schema: &Schema) -> Self {
        let fields = schema
            .fields
            .iter()
            .map(|f| FieldValidator::from(f))
            .collect();

        Validator { fields }
    }
}
