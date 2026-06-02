# Examples

These examples use language-neutral KIR directly.

## Build A KIR Document

```rust
use std::collections::BTreeMap;

use mercurio_core::{KIR_SCHEMA_VERSION, KirDocument, KirElement};
use serde_json::json;

let document = KirDocument {
    metadata: BTreeMap::from([
        ("kir_schema_version".to_string(), json!(KIR_SCHEMA_VERSION)),
    ]),
    elements: vec![
        KirElement {
            id: "pkg.Demo".to_string(),
            kind: "model.Package".to_string(),
            layer: 2,
            properties: BTreeMap::from([
                ("qualified_name".to_string(), json!("Demo")),
                ("declared_name".to_string(), json!("Demo")),
                ("members".to_string(), json!(["type.Demo.Vehicle"])),
            ]),
        },
        KirElement {
            id: "type.Demo.Vehicle".to_string(),
            kind: "model.Type".to_string(),
            layer: 2,
            properties: BTreeMap::from([
                ("qualified_name".to_string(), json!("Demo.Vehicle")),
                ("declared_name".to_string(), json!("Vehicle")),
                ("owner".to_string(), json!("pkg.Demo")),
            ]),
        },
    ],
};

document.validate()?;
```

## Build A Graph

```rust
use mercurio_core::Graph;

let graph = Graph::from_document(document.clone())?;
let package = graph.node_id("pkg.Demo").expect("package exists");

for edge in graph.outgoing_edges(package) {
    println!("{} -> {:?}", edge.relation, graph.element_id(edge.target));
}
```

## Start A Runtime

```rust
use mercurio_core::Runtime;

let runtime = Runtime::from_document(document)?;
let artifact = runtime.into_artifact();
```

`RuntimeArtifact` can be persisted by a host and restored later without reparsing source text.

## Compile Through A Language Registry

```rust
use std::path::Path;

use mercurio_core::{
    LanguageRegistry, KirDocument, load_model_stack_with_registry,
};

let mut registry = LanguageRegistry::new();
registry.register(ToyLanguage);

let library_context = KirDocument {
    metadata: Default::default(),
    elements: Vec::new(),
}
.with_schema_version();

let compiled = load_model_stack_with_registry(
    Path::new("model.toy"),
    &library_context,
    &registry,
)?;
```

The important point is that foundation sees only the language service contract and the resulting KIR.
