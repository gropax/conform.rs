use crate::schema::{Field, FieldType, Multiplicity, NumberConstraint, Schema, StringConstraint};
use regex::Regex;
use url::Url;

#[derive(Debug)]
pub struct TypeValidationError {
    pub message: String,
}

trait ValidateNumber {
    fn validate(&self, value: f64) -> Option<TypeValidationError>;
}

trait ValidateString {
    fn validate(&self, value: &str) -> Option<TypeValidationError>;
}

struct AboveValidator {
    min: f64,
}

impl ValidateNumber for AboveValidator {
    fn validate(&self, value: f64) -> Option<TypeValidationError> {
        (value < self.min).then(|| TypeValidationError {
            message: format!("Expected number >= {}, found {}", self.min, value),
        })
    }
}

struct BelowValidator {
    max: f64,
}

impl ValidateNumber for BelowValidator {
    fn validate(&self, value: f64) -> Option<TypeValidationError> {
        (value > self.max).then(|| TypeValidationError {
            message: format!("Expected number <= {}, found {}", self.max, value),
        })
    }
}

struct UrlValidator {}

impl ValidateString for UrlValidator {
    fn validate(&self, value: &str) -> Option<TypeValidationError> {
        Url::parse(value).err().map(|_| TypeValidationError {
            message: format!("\"{}\" is not an URL", value),
        })
    }
}

struct EnumValidator {
    values: Vec<String>,
}

impl ValidateString for EnumValidator {
    fn validate(&self, value: &str) -> Option<TypeValidationError> {
        (!self.values.iter().any(|v| v == value)).then(|| TypeValidationError {
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
    fn validate(&self, value: &str) -> Option<TypeValidationError> {
        (!value.starts_with(&self.prefix)).then(|| TypeValidationError {
            message: format!("\"{}\" don't start with \"{}\"", value, self.prefix),
        })
    }
}

struct EndsWithValidator {
    suffix: String,
}

impl ValidateString for EndsWithValidator {
    fn validate(&self, value: &str) -> Option<TypeValidationError> {
        (!value.ends_with(&self.suffix)).then(|| TypeValidationError {
            message: format!("\"{}\" don't ends with \"{}\"", value, self.suffix),
        })
    }
}

struct RegexValidator {
    pattern: String,
    regex: Regex,
}

impl ValidateString for RegexValidator {
    fn validate(&self, value: &str) -> Option<TypeValidationError> {
        (!self.regex.is_match(value)).then(|| TypeValidationError {
            message: format!("\"{}\" don't match pattern /{}/", value, self.pattern),
        })
    }
}

enum TypeValidator {
    Bool,
    Number(Vec<Box<dyn ValidateNumber>>),
    String(Vec<Box<dyn ValidateString>>),
}

struct FieldValidator {
    name: String,
    multiplicity: Multiplicity,
    rules: TypeValidator,
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

fn type_validator(field_type: &FieldType) -> TypeValidator {
    match field_type {
        FieldType::Bool => TypeValidator::Bool,

        FieldType::Number(constraints) => {
            let validators = constraints.iter().map(|c| number_validator(c)).collect();

            TypeValidator::Number(validators)
        }

        FieldType::String(constraints) => {
            let validators = constraints.iter().map(|c| string_validator(c)).collect();

            TypeValidator::String(validators)
        }
    }
}

impl From<&Field> for FieldValidator {
    fn from(field: &Field) -> Self {
        let rules = type_validator(&field.kind);

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
