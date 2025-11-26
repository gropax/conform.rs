use super::ValidationError;
use std::fmt::Write;

pub fn errors_to_quickfix(errors: Vec<ValidationError>) -> String {
    let mut out = String::new();

    for err in errors {
        let _ = write!(
            out,
            "{}:{}:{}: {}\n",
            err.span.file,
            err.span.line,
            err.span.column,
            &err.message
        );
    }

    out
}
