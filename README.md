# Mercurio Foundation

Mercurio Foundation is the KerML-aligned, source-language-neutral modeling substrate for Mercurio.

It stores models as KIR, projects them into graphs, runs deterministic semantic services, and exposes contracts that source-language repositories can implement. It does not own concrete source syntax, versioned KerML/SysML metamodel bundles, command-line host behavior, UI adapters, or product workflows.

## Philosophy

Foundation is meant to work like a small reflective modeling core:

- models are persisted as structured semantic data,
- model elements are inspectable without a parser,
- references become graph edges,
- runtime services operate over the graph,
- source languages plug in through explicit services,
- hosts decide how models are edited, stored, packaged, and presented.

The current foundation vocabulary is intentionally aligned with the OMG Kernel Modeling Language (KerML) concepts of packages, types, features, definitions, usages, specialization, typing, and ownership. Foundation should not import a concrete KerML parser or bundled version-specific KerML library; those belong in language repositories and packages.

See [Foundation Philosophy](docs/philosophy.md) for the longer version, including a short comparison to classic modeling-framework responsibilities.

## Core Terms

- **KIR**: the kernel interchange representation. KIR is the validated JSON model format consumed by graph, runtime, query, package, and adapter APIs.
- **Element**: a KIR node with an `id`, `kind`, `layer`, and `properties`.
- **Graph**: a relationship view derived from KIR reference properties.
- **Runtime**: deterministic evaluation over a graph, derived indexes, expression IR, and rulepacks.
- **Language service**: a registered compiler boundary that turns source text into KIR.
- **KerML alignment**: the foundation semantic vocabulary follows the shape of the OMG Kernel Modeling Language while leaving version-specific metamodel packages outside foundation.

See [KIR](docs/kir.md), [Language Services](docs/language-services.md), and the [Sample Language](docs/sample-language.md).

## Crates

- `mercurio-kir`: KIR schema, validation, merge, and IO.
- `mercurio-language-contracts`: diagnostics, reports, expression IR, and language-service contracts.
- `mercurio-foundation`: graph, runtime, query, package, workspace, session, mutation, and view APIs built on KIR.

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

Large-model timing and memory checks are documented in [Performance](docs/performance.md).

## Build

```powershell
cargo build
```

## Test

```powershell
cargo test --no-run
```

Foundation tests use language-neutral KIR fixtures and a small test-only toy language service for registry, cache, graph, and runtime coverage.
