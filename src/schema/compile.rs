use crate::validation::{
    ValidateNumber, ValidateString,
    AboveValidator, BelowValidator,
    EnumValidator, UrlValidator, RegexValidator, StartsWithValidator, EndsWithValidator,
    ScalarValidator, FieldValidator, DocumentValidator
};
use crate::schema::{Field, FieldType, NumberConstraint, Schema, StringConstraint};
use std::collections::HashSet;

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
