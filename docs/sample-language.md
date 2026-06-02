# Sample Language

This page sketches a minimal source language that compiles into KIR. It is intentionally small so the boundary is visible: source text is parsed outside foundation, and foundation receives KIR plus diagnostics through `LanguageService`.

The toy syntax accepts a package and type declarations:

```text
package Demo
type Vehicle
type Wheel
```

## Define The Service

```rust
use std::collections::BTreeMap;

use mercurio_language_contracts::{
    CompileContext, Diagnostic, LanguageService, SemanticCompileReport,
    SemanticCompileStatus,
};
use mercurio_kir::{KIR_SCHEMA_VERSION, KirDocument, KirElement};
use serde_json::json;

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
        match compile_toy(source, context.source_name) {
            Ok(document) => SemanticCompileReport {
                status: SemanticCompileStatus::Ok,
                diagnostics: Vec::new(),
                document: Some(document),
            },
            Err(message) => SemanticCompileReport {
                status: SemanticCompileStatus::Failed,
                diagnostics: vec![Diagnostic::new(message, None)],
                document: None,
            },
        }
    }
}

fn compile_toy(source: &str, source_name: &str) -> Result<KirDocument, String> {
    let mut words = source.split_whitespace();
    if words.next() != Some("package") {
        return Err("toy source must start with `package <name>`".to_string());
    }
    let package = words
        .next()
        .ok_or_else(|| "toy package requires a name".to_string())?;

    let mut elements = Vec::new();
    let mut members = Vec::new();

    while let Some(keyword) = words.next() {
        match keyword {
            "type" => {
                let name = words
                    .next()
                    .ok_or_else(|| "toy type requires a name".to_string())?;
                let id = format!("type.{package}.{name}");
                members.push(id.clone());
                elements.push(KirElement {
                    id,
                    kind: "model.Type".to_string(),
                    layer: 2,
                    properties: BTreeMap::from([
                        ("qualified_name".to_string(), json!(format!("{package}.{name}"))),
                        ("declared_name".to_string(), json!(name)),
                        ("owner".to_string(), json!(format!("pkg.{package}"))),
                        ("source_file".to_string(), json!(source_name)),
                    ]),
                });
            }
            other => return Err(format!("unsupported toy declaration `{other}`")),
        }
    }

    elements.insert(
        0,
        KirElement {
            id: format!("pkg.{package}"),
            kind: "model.Package".to_string(),
            layer: 2,
            properties: BTreeMap::from([
                ("qualified_name".to_string(), json!(package)),
                ("declared_name".to_string(), json!(package)),
                ("members".to_string(), json!(members)),
                ("source_file".to_string(), json!(source_name)),
            ]),
        },
    );

    Ok(KirDocument {
        metadata: BTreeMap::from([
            ("kir_schema_version".to_string(), json!(KIR_SCHEMA_VERSION)),
        ]),
        elements,
    })
}
```

## Use The Service

```rust
use std::path::Path;

use mercurio_core::{KirDocument, LanguageRegistry, load_model_stack_with_registry};

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
document.validate()?;
```

The host can now pass `document` to graph, runtime, query, package, cache, or adapter APIs. Those APIs do not need to know the toy syntax.

## What A Real Language Adds

A production language service normally expands this skeleton with:

- a parser that produces a language-owned AST,
- a metamodel or schema package for its supported versions,
- semantic checks and diagnostics with source spans,
- lowering from language concepts into KIR elements and properties,
- package/library resolution,
- formatting and editor services outside foundation.

Foundation still only depends on the service contract and the emitted KIR.
