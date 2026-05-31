//! SysML language facade.
//!
//! This crate is the public SysML language implementation boundary: parsing,
//! recovery/reporting, compilation to KIR, and the SysML baseline library.

pub mod parser;

pub use mercurio_kir::{KirDocument, KirError};
pub use mercurio_language_contracts::ast::{ParsedModule, QualifiedName, SourceSpan, SysmlModule};
pub use mercurio_language_contracts::diagnostics::Diagnostic;
pub use mercurio_language_contracts::reports::{ParseReport, SemanticCompileStatus};
pub use mercurio_language_contracts::{SemanticConcept, SourceLanguage};
pub use parser::{
    SemanticCompileReport, SysmlError, compile_sysml_module, compile_sysml_module_with_context,
    compile_sysml_module_with_context_report, compile_sysml_module_with_context_report_with_limit,
    compile_sysml_module_with_resolver_context,
    compile_sysml_module_with_resolver_context_report_with_limit, compile_sysml_text,
    compile_sysml_text_with_context, compile_sysml_text_with_context_report,
    default_sysml_delta_library_path, load_sysml_baseline, load_sysml_document,
    load_sysml_document_with_stdlib, parse_sysml, parse_sysml_recovering,
};

#[derive(Debug)]
pub struct SysmlLanguageModule;

pub fn parse(input: &str) -> Result<ParsedModule, Diagnostic> {
    parse_sysml(input)
}

pub fn compile_text(
    input: &str,
    source_name: &str,
    library_context: &KirDocument,
) -> Result<KirDocument, Diagnostic> {
    compile_sysml_text(input, source_name, library_context)
}

pub fn compile_text_with_context(
    input: &str,
    source_name: &str,
    context_modules: &[ParsedModule],
    library_context: &KirDocument,
) -> Result<KirDocument, Diagnostic> {
    compile_sysml_text_with_context(input, source_name, context_modules, library_context)
}

pub fn default_sysml_library_path() -> std::path::PathBuf {
    default_sysml_delta_library_path()
}

pub fn legacy_monolithic_sysml_library_path() -> std::path::PathBuf {
    parser::default_sysml_library_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_parses_minimal_sysml() {
        let module = parse("package Demo { part def Vehicle; }").unwrap();

        assert!(module.package.is_some());
    }

    #[test]
    fn facade_compiles_minimal_sysml() {
        let stdlib = load_sysml_baseline().unwrap();
        let document = compile_sysml_text(
            "package Demo { part def Vehicle; part vehicle : Vehicle; }",
            "inline.sysml",
            &stdlib,
        )
        .unwrap();

        assert!(document.elements.iter().any(|element| {
            element.id == "part_definition.Demo.Vehicle"
                || element.id == "definition.Demo.Vehicle"
                || element.properties.get("declared_name")
                    == Some(&serde_json::Value::String("Vehicle".to_string()))
        }));
    }

    #[test]
    fn baseline_is_kernel_plus_sysml_delta() {
        let baseline = load_sysml_baseline().unwrap();

        assert!(
            baseline
                .elements
                .iter()
                .any(|element| { element.id.contains("Kernel") || element.kind.contains("KerML") })
        );
        assert!(
            baseline
                .elements
                .iter()
                .any(|element| { element.id.contains("SysML") || element.kind.contains("SysML") })
        );
    }
}
