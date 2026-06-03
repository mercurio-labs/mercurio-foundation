//! Contracts implemented by Mercurio source-language frontends.
//!
//! This crate exposes the shared AST, diagnostic, expression, report, and
//! service traits used by language plugins. Prefer the root-level re-exports
//! for common integration code.

use std::fmt;
use std::path::Path;

use serde::{Deserialize, Serialize};

pub mod ast;
pub mod diagnostics;
pub mod expression;
pub mod lexer;
pub mod reports;
pub mod service;

pub use ast::*;
pub use diagnostics::Diagnostic;
pub use expression::{
    BinaryExpressionOp, ExpressionEvaluationContext, ExpressionEvaluationError, ExpressionIr,
    ExpressionIrError, ExpressionPathRoot, ExpressionPathSegment, ExpressionValidationError,
    UnaryExpressionOp,
};
pub use lexer::{Token, TokenKind, lex};
pub use reports::{ParseReport, SemanticCompileReport, SemanticCompileStatus};
pub use service::{CompileContext, LanguageRegistry, LanguageService};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceLanguage {
    Core,
    Model,
}

impl SourceLanguage {
    pub fn from_path(path: &Path) -> Option<Self> {
        match path.extension().and_then(|extension| extension.to_str()) {
            Some(extension) if extension.eq_ignore_ascii_case("model") => Some(Self::Model),
            Some(extension) if extension.eq_ignore_ascii_case("core") => Some(Self::Core),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Core => "core",
        }
    }

    pub fn extensions(self) -> &'static [&'static str] {
        match self {
            Self::Model => &["model"],
            Self::Core => &["core"],
        }
    }
}

impl fmt::Display for SourceLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticConcept {
    Package,
    Type,
}
