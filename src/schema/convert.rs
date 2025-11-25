use super::{
    Field, FieldType, Multiplicity, NumberContraint, RawField, RawMultiplicity, RawSchema, Schema,
    StringContraint,
};
use regex::Regex;
use std::collections::HashMap;
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

fn string_constraints(
    starts_with: Option<String>,
    pattern: Option<String>,
) -> Result<Vec<StringContraint>, SchemaError> {
    let mut constraints = Vec::<StringContraint>::new();

    if let Some(starts_with_val) = starts_with {
        constraints.push(StringContraint::StartsWith(starts_with_val));
    }

    if let Some(pattern_val) = pattern {
        let regex = Regex::new(&pattern_val)?;
        constraints.push(StringContraint::Pattern(regex));
    }

    Ok(constraints)
}

fn convert_field(name: String, raw: RawField) -> Result<Field, SchemaError> {
    let field = match raw {
        RawField::Number { multiplicity } => {
            let mult = Multiplicity::try_from(multiplicity)?;

            Field {
                name,
                multiplicity: mult,
                r#type: FieldType::Number,
            }
        }

        RawField::Enum {
            multiplicity,
            values,
        } => {
            let mult = Multiplicity::try_from(multiplicity)?;
            let constraints = vec![StringContraint::Enum(values)];

            Field {
                name,
                multiplicity: mult,
                r#type: FieldType::String(constraints),
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
                r#type: FieldType::String(constraints),
            }
        }

        RawField::Url {
            multiplicity,
            starts_with,
            pattern,
        } => {
            let mult = Multiplicity::try_from(multiplicity)?;
            let mut constraints = string_constraints(starts_with, pattern)?;
            constraints.push(StringContraint::Url);

            Field {
                name,
                multiplicity: mult,
                r#type: FieldType::String(constraints),
            }
        }
    };

    Ok(field)
}

impl TryFrom<RawSchema> for Schema {
    type Error = SchemaError;

    fn try_from(raw: RawSchema) -> Result<Schema, Self::Error> {
        let field_results = raw
            .fields
            .into_iter()
            .map(|(name, field)| convert_field(name, field))
            .collect::<Result<Vec<_>, _>>()?;

        let fields: HashMap<String, Field> = field_results
            .into_iter()
            .map(|f| (f.name.to_string(), f))
            .collect();

        let schema = Schema {
            name: raw.name.to_string(),
            fields,
        };

        Ok(schema)
    }
}
