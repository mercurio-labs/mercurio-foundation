use crate::frontend::ast::ParsedModule;
use crate::frontend::diagnostics::Diagnostic;
use crate::ir::KirDocument;

pub use mercurio_language_contracts::reports::{SemanticCompileReport, SemanticCompileStatus};

#[cfg(not(test))]
pub fn compile_model_text(
    _input: &str,
    source_name: &str,
    _library_context: &KirDocument,
) -> Result<KirDocument, Diagnostic> {
    Err(Diagnostic::new(
        format!("source compilation requires a language service for `{source_name}`"),
        None,
    ))
}

#[cfg(test)]
pub fn compile_model_text(
    input: &str,
    source_name: &str,
    library_context: &KirDocument,
) -> Result<KirDocument, Diagnostic> {
    crate::source_set::compile_source_document_with_registry(
        &crate::source_set::SourceDocument::new(source_name, input),
        library_context,
        &fake_model_registry(),
    )
    .map_err(|err| Diagnostic::new(err.to_string(), None))
}

#[cfg(not(test))]
pub fn compile_model_text_with_context_report(
    _input: &str,
    source_name: &str,
    _context_modules: &[ParsedModule],
    _library_context: &KirDocument,
) -> SemanticCompileReport<KirDocument> {
    SemanticCompileReport {
        status: SemanticCompileStatus::Failed,
        diagnostics: vec![Diagnostic::new(
            format!("source compilation requires a language service for `{source_name}`"),
            None,
        )],
        document: None,
    }
}

#[cfg(test)]
pub fn compile_model_text_with_context_report(
    input: &str,
    source_name: &str,
    _context_modules: &[ParsedModule],
    library_context: &KirDocument,
) -> SemanticCompileReport<KirDocument> {
    match compile_model_text(input, source_name, library_context) {
        Ok(document) => SemanticCompileReport {
            status: SemanticCompileStatus::Ok,
            diagnostics: Vec::new(),
            document: Some(document),
        },
        Err(diagnostic) => SemanticCompileReport {
            status: SemanticCompileStatus::Failed,
            diagnostics: vec![diagnostic],
            document: None,
        },
    }
}

#[cfg(test)]
fn fake_model_registry() -> mercurio_language_contracts::LanguageRegistry {
    struct FakeModelLanguage;

    impl mercurio_language_contracts::LanguageService for FakeModelLanguage {
        fn language_id(&self) -> &str {
            "fake-model"
        }

        fn extensions(&self) -> &[&str] {
            &["model"]
        }

        fn compile(
            &self,
            source: &str,
            context: mercurio_language_contracts::CompileContext<'_>,
        ) -> SemanticCompileReport<KirDocument> {
            match crate::source_set::compile_source_documents(
                vec![crate::source_set::SourceDocument::new(
                    context.source_name,
                    source,
                )],
                context.library_context,
            ) {
                Ok(document) => SemanticCompileReport {
                    status: SemanticCompileStatus::Ok,
                    diagnostics: Vec::new(),
                    document: Some(document),
                },
                Err(err) => SemanticCompileReport {
                    status: SemanticCompileStatus::Failed,
                    diagnostics: vec![Diagnostic::new(err.to_string(), None)],
                    document: None,
                },
            }
        }
    }

    let mut registry = mercurio_language_contracts::LanguageRegistry::new();
    registry.register(FakeModelLanguage);
    registry
}
