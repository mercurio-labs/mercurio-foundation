use std::path::Path;

use mercurio_language_contracts::{CompileContext, LanguageRegistry};

use crate::frontend::diagnostics::Diagnostic;
use crate::ir::{KirDocument, KirError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDocument {
    pub path: String,
    pub content: String,
}

impl SourceDocument {
    pub fn new(path: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            content: content.into(),
        }
    }
}

pub fn compile_source_documents_with_registry(
    source_documents: Vec<SourceDocument>,
    library_context: &KirDocument,
    registry: &LanguageRegistry,
) -> Result<KirDocument, KirError> {
    let documents = source_documents
        .iter()
        .map(|source| compile_source_document_with_registry(source, library_context, registry))
        .collect::<Result<Vec<_>, _>>()?;

    KirDocument::merge(documents)
}

pub fn compile_source_document_with_registry(
    source: &SourceDocument,
    library_context: &KirDocument,
    registry: &LanguageRegistry,
) -> Result<KirDocument, KirError> {
    let service = registry
        .service_for_path(Path::new(&source.path))
        .ok_or_else(|| {
            KirError::Frontend(format!(
                "no registered language service for `{}`",
                source.path
            ))
        })?;
    let report = service.compile(
        &source.content,
        CompileContext {
            source_name: &source.path,
            library_context,
        },
    );

    report.document.ok_or_else(|| {
        let details = report
            .diagnostics
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("; ");
        KirError::Frontend(format!(
            "language service `{}` did not produce KIR for `{}`{}",
            service.language_id(),
            source.path,
            if details.is_empty() {
                String::new()
            } else {
                format!(": {details}")
            }
        ))
    })
}

#[cfg(not(test))]
pub fn compile_source_documents(
    _source_documents: Vec<SourceDocument>,
    _library_context: &KirDocument,
) -> Result<KirDocument, KirError> {
    Err(KirError::Frontend(
        "source compilation requires a registered language service".to_string(),
    ))
}

#[cfg(test)]
pub fn compile_source_documents(
    source_documents: Vec<SourceDocument>,
    _library_context: &KirDocument,
) -> Result<KirDocument, KirError> {
    let documents = source_documents
        .iter()
        .map(fake_compile_source_document)
        .collect::<Vec<_>>();
    KirDocument::merge(documents)
}

#[cfg(test)]
fn fake_compile_source_document(source: &SourceDocument) -> KirDocument {
    use std::collections::BTreeMap;

    use serde_json::json;

    use crate::ir::{KIR_SCHEMA_VERSION, KirElement};

    let package = fake_package_name(&source.content)
        .or_else(|| {
            std::path::Path::new(&source.path)
                .file_stem()
                .and_then(|value| value.to_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "Model".to_string());
    let names = fake_definition_names(&source.content);
    let member_ids = names
        .iter()
        .map(|name| format!("type.{package}.{name}"))
        .collect::<Vec<_>>();
    let mut elements = vec![KirElement {
        id: format!("pkg.{package}"),
        kind: "model.Package".to_string(),
        layer: 2,
        properties: BTreeMap::from([
            ("qualified_name".to_string(), json!(package)),
            ("declared_name".to_string(), json!(package)),
            ("members".to_string(), json!(member_ids)),
            (
                "metadata".to_string(),
                json!({ "source_file": source.path }),
            ),
        ]),
    }];
    for name in names {
        elements.push(KirElement {
            id: format!("type.{package}.{name}"),
            kind: "model.PartDefinition".to_string(),
            layer: 2,
            properties: BTreeMap::from([
                (
                    "qualified_name".to_string(),
                    json!(format!("{package}.{name}")),
                ),
                ("declared_name".to_string(), json!(name)),
                (
                    "metadata".to_string(),
                    json!({ "source_file": source.path }),
                ),
            ]),
        });
    }
    KirDocument {
        metadata: BTreeMap::from([("kir_schema_version".to_string(), json!(KIR_SCHEMA_VERSION))]),
        elements,
    }
}

#[cfg(test)]
fn fake_package_name(source: &str) -> Option<String> {
    source
        .split_whitespace()
        .collect::<Vec<_>>()
        .windows(2)
        .find_map(|window| {
            (window[0] == "package").then(|| {
                window[1]
                    .trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
                    .to_string()
            })
        })
}

#[cfg(test)]
fn fake_definition_names(source: &str) -> Vec<String> {
    let tokens = source.split_whitespace().collect::<Vec<_>>();
    let mut names = Vec::new();
    for window in tokens.windows(3) {
        if window[1] == "def" {
            names.push(
                window[2]
                    .trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
                    .to_string(),
            );
        }
    }
    if names.is_empty() {
        names.push("Thing".to_string());
    }
    names.sort();
    names.dedup();
    names
}

pub fn parse_source_module(_path: &str, _content: &str) -> Result<(), Diagnostic> {
    Err(Diagnostic::new(
        "source parsing requires a language-specific parser".to_string(),
        None,
    ))
}
