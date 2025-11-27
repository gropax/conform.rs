use crate::document;
use crate::schema;
use crate::validation;
use anyhow::Result;
use std::path::PathBuf;
use tracing::info;
use crate::utils;

#[derive(Debug)]
pub struct Args {
    pub schema_file: PathBuf,
    pub document_patterns: Vec<String>,
}

pub fn handle(args: &Args) -> Result<()> {
    let document_files = utils::expand_globs(&args.document_patterns)?;

    let schema = schema::load_schema(&args.schema_file)?;
    info!("Loaded schema from file: {}", args.schema_file.display());

    let validator = schema.compile();
    info!("Compiled schema into validator");

    let mut errors: Vec<validation::SpanError> = Vec::new();

    for document_file in &document_files {
        let document = document::parse(document_file)?;
        info!("Loaded document from file: {}", document_file.display());

        if let Some(document_error) = validator.validate(&document) {
            let span_errors = validation::into_span_errors(&document_error, document_file);
            errors.extend(span_errors);
        }
        info!("Validated document");
    }


    let quickfix = validation::to_quickfix(errors);
    print!("{}", quickfix);

    Ok(())
}
