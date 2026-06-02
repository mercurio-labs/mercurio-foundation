use std::path::Path;

use mercurio_kir::KirDocument;

use crate::diagnostics::Diagnostic;
use crate::reports::{SemanticCompileReport, SemanticCompileStatus};

#[derive(Debug, Clone, Copy)]
pub struct CompileContext<'a> {
    pub source_name: &'a str,
    pub library_context: &'a KirDocument,
}

pub trait LanguageService: Send + Sync {
    fn language_id(&self) -> &str;

    fn extensions(&self) -> &[&str];

    fn compile(
        &self,
        source: &str,
        context: CompileContext<'_>,
    ) -> SemanticCompileReport<KirDocument>;

    fn supports_path(&self, path: &Path) -> bool {
        let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
            return false;
        };
        self.extensions()
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(extension))
    }
}

#[derive(Default)]
pub struct LanguageRegistry {
    services: Vec<Box<dyn LanguageService>>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, service: impl LanguageService + 'static) {
        self.services.push(Box::new(service));
    }

    pub fn services(&self) -> &[Box<dyn LanguageService>] {
        &self.services
    }

    pub fn service_for_language(&self, language_id: &str) -> Option<&dyn LanguageService> {
        self.services
            .iter()
            .map(Box::as_ref)
            .find(|service| service.language_id() == language_id)
    }

    pub fn service_for_path(&self, path: &Path) -> Option<&dyn LanguageService> {
        self.services
            .iter()
            .map(Box::as_ref)
            .find(|service| service.supports_path(path))
    }

    pub fn compile_path(
        &self,
        path: &Path,
        source: &str,
        library_context: &KirDocument,
    ) -> SemanticCompileReport<KirDocument> {
        let Some(service) = self.service_for_path(path) else {
            return SemanticCompileReport {
                status: SemanticCompileStatus::Failed,
                diagnostics: vec![Diagnostic::new(
                    format!("no registered language service for `{}`", path.display()),
                    None,
                )],
                document: None,
            };
        };

        let source_name = path.display().to_string();
        service.compile(
            source,
            CompileContext {
                source_name: &source_name,
                library_context,
            },
        )
    }
}
