use crate::document;
use crate::schema::{Field, FieldType, Multiplicity, NumberConstraint, Schema, StringConstraint};
use regex::Regex;
use std::collections::HashSet;
use url::Url;

pub trait FlattenErrors {
    fn flatten(&self) -> Vec<ValidationError>;
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub span: document::Span,
}

impl FlattenErrors for ValidationError {
    fn flatten(&self) -> Vec<ValidationError> {
        vec![self.clone()]
    }
}

#[derive(Debug)]
pub struct ValueError {
    pub value_index: usize,
    pub errors: Vec<ValidationError>,
}

impl FlattenErrors for ValueError {
    fn flatten(&self) -> Vec<ValidationError> {
        self.errors.iter().flat_map(|e| e.flatten()).collect()
    }
}

#[derive(Debug)]
pub struct FieldError {
    pub field_name: String,
    pub errors: Vec<ValidationError>,
    pub values: Vec<ValueError>,
}

impl FlattenErrors for FieldError {
    fn flatten(&self) -> Vec<ValidationError> {
        let mut out: Vec<ValidationError> = Vec::new();

        out.extend(self.errors.iter().flat_map(|e| e.flatten()));
        out.extend(self.values.iter().flat_map(|e| e.flatten()));

        out
    }
}

#[derive(Debug)]
pub struct DocumentError {
    pub file: String,
    pub errors: Vec<ValidationError>,
    pub fields: Vec<FieldError>,
}

impl FlattenErrors for DocumentError {
    fn flatten(&self) -> Vec<ValidationError> {
        let mut out: Vec<ValidationError> = Vec::new();

        out.extend(self.errors.iter().flat_map(|e| e.flatten()));
        out.extend(self.fields.iter().flat_map(|e| e.flatten()));

        out
    }
}

trait ValidateNumber {
    fn validate(&self, span: document::Span, value: f64) -> Option<ValidationError>;
}

trait ValidateString {
    fn validate(&self, span: document::Span, value: &str) -> Option<ValidationError>;
}

struct AboveValidator {
    min: f64,
}

impl ValidateNumber for AboveValidator {
    fn validate(&self, span: document::Span, value: f64) -> Option<ValidationError> {
        (value < self.min).then(|| ValidationError {
            message: format!("Expected number >= {}, found {}", self.min, value),
            span,
        })
    }
}

struct BelowValidator {
    max: f64,
}

impl ValidateNumber for BelowValidator {
    fn validate(&self, span: document::Span, value: f64) -> Option<ValidationError> {
        (value > self.max).then(|| ValidationError {
            message: format!("Expected number <= {}, found {}", self.max, value),
            span,
        })
    }
}

struct UrlValidator {}

impl ValidateString for UrlValidator {
    fn validate(&self, span: document::Span, value: &str) -> Option<ValidationError> {
        Url::parse(value).err().map(|_| ValidationError {
            message: format!("\"{}\" is not an URL", value),
            span,
        })
    }
}

struct EnumValidator {
    values: Vec<String>,
}

impl ValidateString for EnumValidator {
    fn validate(&self, span: document::Span, value: &str) -> Option<ValidationError> {
        (!self.values.iter().any(|v| v == value)).then(|| ValidationError {
            message: format!(
                "\"{}\" don't match any of [{}]",
                value,
                self.values.join(", ")
            ),
            span,
        })
    }
}

struct StartsWithValidator {
    prefix: String,
}

impl ValidateString for StartsWithValidator {
    fn validate(&self, span: document::Span, value: &str) -> Option<ValidationError> {
        (!value.starts_with(&self.prefix)).then(|| ValidationError {
            message: format!("\"{}\" don't start with \"{}\"", value, self.prefix),
            span,
        })
    }
}

struct EndsWithValidator {
    suffix: String,
}

impl ValidateString for EndsWithValidator {
    fn validate(&self, span: document::Span, value: &str) -> Option<ValidationError> {
        (!value.ends_with(&self.suffix)).then(|| ValidationError {
            message: format!("\"{}\" don't ends with \"{}\"", value, self.suffix),
            span,
        })
    }
}

struct RegexValidator {
    pattern: String,
    regex: Regex,
}

impl ValidateString for RegexValidator {
    fn validate(&self, span: document::Span, value: &str) -> Option<ValidationError> {
        (!self.regex.is_match(value)).then(|| ValidationError {
            message: format!("\"{}\" don't match pattern /{}/", value, self.pattern),
            span,
        })
    }
}

enum ScalarValidator {
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

struct FieldValidator {
    name: String,
    multiplicity: Multiplicity,
    rules: ScalarValidator,
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

fn value_validator(field_type: &FieldType) -> ScalarValidator {
    match field_type {
        FieldType::Bool => ScalarValidator::Bool,

        FieldType::Number(constraints) => {
            let validators = constraints.iter().map(|c| number_validator(c)).collect();

            ScalarValidator::Number(validators)
        }

        FieldType::String(constraints) => {
            let validators = constraints.iter().map(|c| string_validator(c)).collect();

            ScalarValidator::String(validators)
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

pub struct DocumentValidator {
    field_names: HashSet<String>,
    fields: Vec<FieldValidator>,
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

impl From<&Schema> for DocumentValidator {
    fn from(schema: &Schema) -> Self {
        let fields = schema.fields.iter().map(FieldValidator::from).collect();
        let field_names: HashSet<String> =
            schema.fields.iter().map(|f| f.name.to_string()).collect();

        DocumentValidator {
            field_names,
            fields,
        }
    }
}
