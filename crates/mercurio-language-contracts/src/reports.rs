use crate::ast::SysmlModule;
use crate::diagnostics::Diagnostic;

#[derive(Debug, Clone)]
pub struct ParseReport {
    pub module: SysmlModule,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticCompileStatus {
    Ok,
    Partial,
    Failed,
}

#[derive(Debug, Clone)]
pub struct SemanticCompileReport<TDocument> {
    pub status: SemanticCompileStatus,
    pub diagnostics: Vec<Diagnostic>,
    pub document: Option<TDocument>,
}
