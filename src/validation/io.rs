use super::{DocumentError, FlattenErrors};
use std::path::PathBuf;
use std::fmt::Write;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct SpanError {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
}

pub fn into_span_errors(document_error: &DocumentError, file_path: &PathBuf) -> Vec<SpanError> {
    let file = file_path.to_string_lossy();

    let mut val_errs = document_error.flatten();
    val_errs.sort_by_key(|e| (e.span.line, e.span.column));

    val_errs
        .iter()
        .chunk_by(|e| e.span.clone())
        .into_iter()
        .map(|(span, group)| SpanError {
            file: file.to_string(),
            line: span.line,
            column: span.column,
            message: group
                .map(|e| e.message.as_str())
                .join(", "),
        })
        .collect()
}

pub fn to_quickfix(errors: Vec<SpanError>) -> String {
    let mut out = String::new();

    for err in errors {
        let _ = writeln!(
            out,
            "{}:{}:{}: {}",
            err.file,
            err.line,
            err.column,
            &err.message
        );
    }

    out
}
