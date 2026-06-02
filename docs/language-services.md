# Language Services

Foundation does not parse source text by itself. A source language connects by implementing `LanguageService` from `mercurio-language-contracts`.

The contract is intentionally small:

```rust
use mercurio_language_contracts::{
    CompileContext, Diagnostic, LanguageService, SemanticCompileReport,
    SemanticCompileStatus,
};
use mercurio_kir::KirDocument;

struct ToyLanguage;

impl LanguageService for ToyLanguage {
    fn language_id(&self) -> &str {
        "toy"
    }

    fn extensions(&self) -> &[&str] {
        &["toy"]
    }

    fn compile(
        &self,
        source: &str,
        context: CompileContext<'_>,
    ) -> SemanticCompileReport<KirDocument> {
        let _ = (source, context);
        SemanticCompileReport {
            status: SemanticCompileStatus::Ok,
            diagnostics: Vec::<Diagnostic>::new(),
            document: KirDocument {
                metadata: Default::default(),
                elements: Vec::new(),
            }
            .with_schema_version(),
        }
    }
}
```

A host registers language services and then asks foundation to compile source through that registry:

```rust
use std::path::Path;

use mercurio_core::{LanguageRegistry, KirDocument, load_model_stack_with_registry};

let mut registry = LanguageRegistry::new();
registry.register(ToyLanguage);

let library_context = KirDocument {
    metadata: Default::default(),
    elements: Vec::new(),
}
.with_schema_version();

let document = load_model_stack_with_registry(
    Path::new("demo.toy"),
    &library_context,
    &registry,
)?;
```

Language repositories should own:

- source parsing,
- source formatting,
- source linting,
- metamodel versions,
- default libraries,
- source-backed package compilation,
- language-specific test corpora.

Foundation should own:

- KIR validation,
- KIR package IO,
- registry dispatch,
- graph/runtime/query behavior after compilation.
