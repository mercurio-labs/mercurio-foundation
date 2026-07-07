use std::fmt;

use serde::{Deserialize, Serialize};

use crate::ast::SourceSpan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub message: String,
    pub span: Option<SourceSpan>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<String>,
}

impl Diagnostic {
    pub fn new(message: impl Into<String>, span: Option<SourceSpan>) -> Self {
        Self {
            message: message.into(),
            span,
            subjects: Vec::new(),
        }
    }

    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subjects.push(subject.into());
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.span {
            Some(span) => write!(
                f,
                "{} at {}:{}-{}:{}",
                self.message, span.start_line, span.start_col, span.end_line, span.end_col
            ),
            None => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for Diagnostic {}
