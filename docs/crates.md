# Crates

## `mercurio-kir`

Owns the KIR data contract:

- `KirDocument`
- `KirElement`
- schema version constants
- validation diagnostics
- KIR merge and file IO
- field registry metadata used to classify references

This crate should stay small and stable. It is the lowest-level data contract used by every other crate.

## `mercurio-language-contracts`

Owns contracts that language-specific repositories implement:

- lexical and parsed-module data structures used by shared tooling,
- diagnostics,
- parse and compile reports,
- `LanguageService`,
- `LanguageRegistry`,
- expression IR shared by runtime and language compilers.

This crate defines the boundary. It must not depend on a concrete source language.

## `mercurio-model`

Owns source-language-neutral model structures and graph projection:

- graph projection from KIR,
- metamodel and metadata views,
- derived-model primitives,
- expression evaluation primitives shared by runtime services.

This crate consumes KIR but should not contain source parsing, host behavior, UI behavior, AI orchestration, or plugin-host contracts.

## `mercurio-runtime`

Owns deterministic runtime services over graph artifacts:

- runtime construction from KIR or graph artifacts,
- derived indexes and rulepack materialization,
- semantic queries,
- expression IR evaluation,
- runtime artifacts and profiling.

This crate should stay deterministic and should not depend on product, reasoning, adapter, AI, or UI crates.

## `mercurio-authoring`

Owns source-language-neutral authoring services:

- source sets and language registry integration,
- source-preserving semantic edits,
- generated companion-file fallback edits,
- semantic and editor outlines,
- lightweight frontend helpers and test language support.

This crate may consume registered language services, but it should not contain a concrete production parser or version-specific metamodel bundle.

## `mercurio-semantic-services`

Owns semantic operations over KIR, graph, and authoring contexts:

- semantic anchors and workspace revisions,
- mutation plans and semantic diffs,
- feasibility and legality checks,
- semantic validation,
- next-action and variant-preview services.

This crate should remain source-language-neutral. Language-specific rules should enter through profiles, rulepacks, registries, or explicit host contracts.

## `mercurio-workspace`

Owns workspace and package infrastructure:

- repository paths and default resource lookup,
- package descriptors and package repositories,
- workspace descriptors and resolved contexts,
- model state and revision envelopes,
- persistent compile cache,
- plugin registry helpers,
- local performance harnesses.

This crate hosts filesystem-aware workspace behavior. Deterministic model evaluation still belongs in `mercurio-runtime`.

## `mercurio-analysis`

Owns reusable semantic analysis contracts and reports:

- AI review request and feedback contracts,
- semantic assessment and evidence,
- generic inspection and impact capabilities,
- cognitive context and quality goals,
- semantic comparison reports.

This crate should describe analysis over foundation data rather than owning AI orchestration, product workflows, or language-specific lowering.

## `mercurio-query-dsl`

Owns user-facing query execution surfaces:

- structured query parsing and execution,
- Rhai DSL bindings,
- DSL schemas and reports,
- capability-backed query artifacts.

This crate consumes model, runtime, session, workspace, and semantic-service APIs. It should not own the underlying graph or runtime primitives.

## `mercurio-codegen`

Owns code-generation and profile helper APIs:

- language profiles,
- metamodel concept registry,
- library context helpers,
- Python wrapper/code generation.

This crate should generate from KIR/profile data without becoming a language compiler.

## `mercurio-session`

Owns interactive semantic sessions:

- session state,
- forks and overlays,
- host-authorized commit operations,
- transaction reports.

This crate coordinates authoring, workspace, and semantic services. It should keep host authorization explicit and avoid direct product or UI dependencies.

## `mercurio-simulation-core`

Owns source-neutral deterministic simulation primitives:

- event-step simulation over KIR-projected behavior facts,
- guard evaluation through runtime services,
- source-neutral trace evidence.

SysML-specific behavior lowering, library interpretation, and UI naming should live in language or product layers that call this crate.

## `mercurio-views`

Owns source-language-neutral view DTOs and rendering helpers:

- element and model summary views,
- explorer graph DTOs,
- table and diagram view documents,
- deterministic SVG rendering helpers.

This crate depends on KIR, model, and runtime primitives directly. It intentionally does not sit behind the `mercurio-core` facade because view APIs are an adapter-facing support layer.

## `mercurio-core`

Owns the public source-language-neutral, KerML-aligned compatibility facade.

The Cargo package is named `mercurio-core`, while the Rust library target remains `mercurio_core` for existing consumers. The crate reexports the substrate APIs from the focused crates above and keeps only small local glue modules such as the KIR model-stack loader, logging helpers, proposal helpers, semantic-target resolution, and binaries.

This facade should not accumulate new subsystem ownership. New behavior should usually land in the focused crate that owns the noun: KIR, model, runtime, authoring, semantic services, workspace, analysis, query DSL, codegen, session, simulation, or views.

Its core concepts intentionally follow a KerML-style modeling vocabulary, but version-specific KerML/SysML libraries and lowering rules should live outside this crate.
