use crate::validation::ValidationError;
use crate::document;
use regex::Regex;
use url::Url;

pub trait ValidateNumber {
    fn validate(&self, span: document::Span, value: f64) -> Option<ValidationError>;
}

pub trait ValidateString {
    fn validate(&self, span: document::Span, value: &str) -> Option<ValidationError>;
}

pub struct AboveValidator {
    pub min: f64,
}

impl ValidateNumber for AboveValidator {
    fn validate(&self, span: document::Span, value: f64) -> Option<ValidationError> {
        (value < self.min).then(|| ValidationError {
            message: format!("Expected number >= {}, found {}", self.min, value),
            span,
        })
    }
}

pub struct BelowValidator {
    pub max: f64,
}

impl ValidateNumber for BelowValidator {
    fn validate(&self, span: document::Span, value: f64) -> Option<ValidationError> {
        (value > self.max).then(|| ValidationError {
            message: format!("Expected number <= {}, found {}", self.max, value),
            span,
        })
    }
}

pub struct UrlValidator {}

impl ValidateString for UrlValidator {
    fn validate(&self, span: document::Span, value: &str) -> Option<ValidationError> {
        Url::parse(value).err().map(|_| ValidationError {
            message: format!("\"{}\" is not an URL", value),
            span,
        })
    }
}

pub struct EnumValidator {
    pub values: Vec<String>,
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

pub struct StartsWithValidator {
    pub prefix: String,
}

impl ValidateString for StartsWithValidator {
    fn validate(&self, span: document::Span, value: &str) -> Option<ValidationError> {
        (!value.starts_with(&self.prefix)).then(|| ValidationError {
            message: format!("\"{}\" don't start with \"{}\"", value, self.prefix),
            span,
        })
    }
}

pub struct EndsWithValidator {
    pub suffix: String,
}

impl ValidateString for EndsWithValidator {
    fn validate(&self, span: document::Span, value: &str) -> Option<ValidationError> {
        (!value.ends_with(&self.suffix)).then(|| ValidationError {
            message: format!("\"{}\" don't ends with \"{}\"", value, self.suffix),
            span,
        })
    }
}

pub struct RegexValidator {
    pub pattern: String,
    pub regex: Regex,
}

impl ValidateString for RegexValidator {
    fn validate(&self, span: document::Span, value: &str) -> Option<ValidationError> {
        (!self.regex.is_match(value)).then(|| ValidationError {
            message: format!("\"{}\" don't match pattern /{}/", value, self.pattern),
            span,
        })
    }
}
