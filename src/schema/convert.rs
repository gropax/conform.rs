use super::{
    Field, FieldType, Multiplicity, NumberConstraint, Schema, StringConstraint, raw::RawField,
    raw::RawMultiplicity, raw::RawSchema,
};
use regex::Regex;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("Regex parsing error: {0}")]
    Regex(#[from] regex::Error),
}

impl TryFrom<RawMultiplicity> for Multiplicity {
    type Error = SchemaError;

    fn try_from(raw: RawMultiplicity) -> Result<Multiplicity, Self::Error> {
        let mult = match raw {
            RawMultiplicity::One => Multiplicity::One,
            RawMultiplicity::Many => Multiplicity::Many,
        };

        Ok(mult)
    }
}

fn number_constraints(
) -> Result<Vec<NumberConstraint>, SchemaError> {
    let mut constraints = Vec::<NumberConstraint>::new();

    Ok(constraints)
}

fn string_constraints(
    starts_with: Option<String>,
    pattern: Option<String>,
) -> Result<Vec<StringConstraint>, SchemaError> {
    let mut constraints = Vec::<StringConstraint>::new();

    if let Some(starts_with_val) = starts_with {
        constraints.push(StringConstraint::StartsWith(starts_with_val));
    }

    if let Some(pattern_val) = pattern {
        let regex = Regex::new(&pattern_val)?;
        constraints.push(StringConstraint::Pattern(pattern_val, regex));
    }

    Ok(constraints)
}

fn convert_field(name: String, raw: RawField) -> Result<Field, SchemaError> {
    let field = match raw {
        RawField::Number { multiplicity } => {
            let mult = Multiplicity::try_from(multiplicity)?;
            let constraints = number_constraints()?;

            Field {
                name,
                multiplicity: mult,
                kind: FieldType::Number(constraints),
            }
        }

        RawField::Enum {
            multiplicity,
            values,
        } => {
            let mult = Multiplicity::try_from(multiplicity)?;
            let constraints = vec![StringConstraint::Enum(values)];

            Field {
                name,
                multiplicity: mult,
                kind: FieldType::String(constraints),
            }
        }

        RawField::String {
            multiplicity,
            starts_with,
            pattern,
        } => {
            let mult = Multiplicity::try_from(multiplicity)?;
            let constraints = string_constraints(starts_with, pattern)?;

            Field {
                name,
                multiplicity: mult,
                kind: FieldType::String(constraints),
            }
        }

        RawField::Url {
            multiplicity,
            starts_with,
            pattern,
        } => {
            let mult = Multiplicity::try_from(multiplicity)?;
            let mut constraints = string_constraints(starts_with, pattern)?;
            constraints.push(StringConstraint::Url);

            Field {
                name,
                multiplicity: mult,
                kind: FieldType::String(constraints),
            }
        }
    };

    Ok(field)
}

impl TryFrom<RawSchema> for Schema {
    type Error = SchemaError;

    fn try_from(raw: RawSchema) -> Result<Schema, Self::Error> {
        let fields = raw
            .fields
            .into_iter()
            .map(|(name, field)| convert_field(name, field))
            .collect::<Result<Vec<_>, _>>()?;

        let schema = Schema {
            name: raw.name.to_string(),
            fields,
        };

        Ok(schema)
    }
}
