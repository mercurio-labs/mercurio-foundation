//! KerML language facade.
//!
//! This crate is the public KerML-facing boundary while the parser/compiler
//! implementation is still hosted in `mercurio-core`. Keep this surface narrow:
//! parsing KerML, compiling KerML to KIR, and loading the KerML/Kernel baseline.

pub mod compiler;
pub mod parser;

pub use compiler::{
    BaselineLibrary, KermlError, KermlLanguageModule, compile_kerml_module,
    compile_kerml_module_with_context, compile_kerml_module_with_resolver_context,
    compile_kerml_text, compile_kerml_text_with_context, compile_kerml_text_with_empty_context,
    compile_text, compile_text_with_context, default_kernel_library_path, load_kerml_document,
    load_kerml_document_with_stdlib, load_kernel_baseline,
};
pub use mercurio_kir::{KirDocument, KirError};
pub use mercurio_language_contracts::ast::{ParsedModule, QualifiedName, SourceSpan};
pub use mercurio_language_contracts::diagnostics::Diagnostic;
pub use mercurio_language_contracts::{SemanticConcept, SourceLanguage};
pub use parser::{parse, parse_kerml};

#[cfg(any())]
pub use mercurio_core::language::kerml::parser::{
    KermlError, compile_kerml_module, compile_kerml_module_with_context, compile_kerml_text,
    compile_kerml_text_with_context, compile_kerml_text_with_empty_context, compile_text,
    compile_text_with_context, load_kerml_document, load_kerml_document_with_stdlib,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_parses_minimal_kerml() {
        let module = parse("package Demo { classifier Vehicle; }").unwrap();

        assert!(module.package.is_some());
    }
}
