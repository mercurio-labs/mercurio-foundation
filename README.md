# Mercurio Foundation

Mercurio Foundation is the language-neutral modeling substrate for Mercurio.

It stores models as KIR, projects them into graphs, runs deterministic semantic services, and exposes contracts that source-language repositories can implement. It does not own concrete source syntax, source-language metamodel bundles, command-line host behavior, UI adapters, or product workflows.

## Philosophy

Foundation is meant to work like a small reflective modeling core:

- models are persisted as structured semantic data,
- model elements are inspectable without a parser,
- references become graph edges,
- runtime services operate over the graph,
- source languages plug in through explicit services,
- hosts decide how models are edited, stored, packaged, and presented.

See [Foundation Philosophy](docs/philosophy.md) for the longer version, including a short comparison to classic modeling-framework responsibilities.

## Core Terms

- **KIR**: the kernel interchange representation. KIR is the validated JSON model format consumed by graph, runtime, query, package, and adapter APIs.
- **Element**: a KIR node with an `id`, `kind`, `layer`, and `properties`.
- **Graph**: a relationship view derived from KIR reference properties.
- **Runtime**: deterministic evaluation over a graph, derived indexes, expression IR, and rulepacks.
- **Language service**: a registered compiler boundary that turns source text into KIR.

See [KIR](docs/kir.md) and [Language Services](docs/language-services.md).

## Crates

- `mercurio-kir`: KIR schema, validation, merge, and IO.
- `mercurio-language-contracts`: diagnostics, reports, expression IR, and language-service contracts.
- `mercurio-foundation`: graph, runtime, query, package, project, session, mutation, and view APIs built on KIR.

See [Crates](docs/crates.md).

## Quick Example

```rust
use std::collections::BTreeMap;

use mercurio_core::{Graph, KIR_SCHEMA_VERSION, KirDocument, KirElement};
use serde_json::json;

let document = KirDocument {
    metadata: BTreeMap::from([
        ("kir_schema_version".to_string(), json!(KIR_SCHEMA_VERSION)),
    ]),
    elements: vec![KirElement {
        id: "pkg.Demo".to_string(),
        kind: "model.Package".to_string(),
        layer: 2,
        properties: BTreeMap::from([
            ("qualified_name".to_string(), json!("Demo")),
            ("declared_name".to_string(), json!("Demo")),
        ]),
    }],
};

document.validate()?;
let graph = Graph::from_document(document)?;
```

More snippets are in [Examples](docs/examples.md).

## Build

```powershell
cargo build
```

## Test

```powershell
cargo test --no-run
```

The repository is currently being split away from language-specific fixtures. During that cleanup, `cargo check` and test compilation are expected to be the reliable boundary checks; full test replacement is tracked separately.
