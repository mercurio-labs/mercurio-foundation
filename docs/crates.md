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

## `mercurio-foundation`

Owns the public source-language-neutral, KerML-aligned substrate facade:

- KIR-native simulation and dynamic-analysis primitives,
- query parsing and execution,
- package descriptors and workspace configuration,
- session overlays and mutation planning,
- semantic views,
- plugin/package registry helpers,
- KIR-backed authoring projections.

This crate consumes KIR and registered language services. It should not contain concrete source-language parsers, metamodel bundles, command-line UI, or product-specific behavior.

Its core concepts intentionally follow a KerML-style modeling vocabulary, but version-specific KerML/SysML libraries and lowering rules should live outside this crate.

Simulation follows the same rule. Foundation may run event-step simulation over KIR-projected behavior facts, evaluate guards through the runtime, and emit source-neutral trace evidence. SysML-specific behavior lowering, library interpretation, and UI naming should live in language or product layers that call into the foundation service.
