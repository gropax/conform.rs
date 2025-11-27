use crate::document;

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
