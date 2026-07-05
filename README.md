# Mercurio Foundation

Mercurio Foundation is the KerML-aligned, source-language-neutral modeling substrate for Mercurio.

It stores models as KIR, projects them into graphs, runs deterministic semantic services, and exposes contracts that source-language repositories can implement. It does not own concrete source syntax, versioned KerML/SysML metamodel bundles, command-line host behavior, UI adapters, or product workflows.

Mercurio is also a semantic operations framework for AI-assisted systems engineering. Rather than treating models as static files or editor-only artifacts, Mercurio represents engineering systems as source-linked semantic workspaces: queryable, explainable, validated, transactional, and auditable. This lets humans, tools, and AI agents operate over the same model substrate.

In that centaur-style workflow, Foundation provides the generic workspace services: stable model identity and source spans, semantic graph and derived features, validation and diagnostics, semantic diff and workspace revisioning, change-set previews, host-authorized transactions, and evidence-producing reasoning services. Domain repositories declare their own semantic capabilities on top of this substrate, with SysML as the first rich profile.

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
- **Element**: a KIR node with an `id`, `kind`, and `properties`; semantic layer is derived by readers.
- **Graph**: a relationship view derived from KIR reference properties.
- **Runtime**: deterministic evaluation over a graph, derived indexes, expression IR, and rulepacks.
- **Language service**: a registered compiler boundary that turns source text into KIR.
- **KerML alignment**: the foundation semantic vocabulary follows the shape of the OMG Kernel Modeling Language while leaving version-specific metamodel packages outside foundation.

See [KIR](docs/kir.md), [Language Services](docs/language-services.md), and the [Sample Language](docs/sample-language.md).

## Crates

This repository is a Cargo workspace. The crates are split by responsibility so the
source-language-neutral core stays small and the higher-level semantic services can
evolve without turning the facade into a catch-all crate.

| Crate | Role |
|-------|------|
| `mercurio-kir` | KIR schema, validation, merge, field registry metadata, and IO. |
| `mercurio-language-contracts` | Diagnostics, reports, expression IR, and language-service contracts. |
| `mercurio-model` | Graph projection, metamodel metadata, derived-model primitives, and expression evaluation support. |
| `mercurio-runtime` | Deterministic graph runtime, derived indexes, expression evaluation, rulepacks, and runtime artifacts. |
| `mercurio-authoring` | Source sets, language registry integration, source-preserving authoring operations, outlines, and frontend helpers. |
| `mercurio-semantic-services` | Semantic identity, mutation planning, feasibility, legality, validation, next actions, and variant previews. |
| `mercurio-workspace` | Repository paths, package libraries, workspace descriptors, model state, compile cache, plugin registry helpers, and performance harnesses. |
| `mercurio-analysis` | AI review contracts, semantic assessment, model inspection and impact capabilities, cognitive context, goals, and semantic comparison. |
| `mercurio-query-dsl` | Query parsing, query execution, Rhai DSL bindings, and capability-backed DSL reports. |
| `mercurio-codegen` | Language profiles, metamodel concept registry, library context, and Python wrapper/code generation. |
| `mercurio-session` | Session overlays, forks, host-authorized commits, and transaction reports. |
| `mercurio-simulation-core` | Source-neutral deterministic simulation primitives over KIR-projected behavior facts. |
| `mercurio-views` | View DTOs, element/model views, diagrams, tables, and deterministic view rendering helpers. |
| `mercurio-core` | Compatibility facade crate. Its Cargo package is `mercurio-core`, and its library target remains `mercurio_core` for existing consumers. |

See [Crates](docs/crates.md).

## Boundary Check

The repository boundary manifest lives at [repo-boundaries.json](repo-boundaries.json). The checker is
hosted in the SysML tooling workspace so it can audit foundation policy while remaining outside the
foundation crate set:

```powershell
cargo run --manifest-path ..\mercurio-sysml\Cargo.toml -p mercurio-tools --bin check_repo_boundaries -- --manifest repo-boundaries.json
cargo run --manifest-path ..\mercurio-sysml\Cargo.toml -p mercurio-tools --bin check_repo_boundaries -- --manifest repo-boundaries.json --strict
```

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
