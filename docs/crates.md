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

## `mercurio-foundation`

Owns language-neutral semantic services:

- graph projection from KIR,
- runtime and derived indexes,
- query parsing and execution,
- package and project descriptors,
- session overlays and mutation planning,
- semantic views,
- plugin/package registry helpers,
- KIR-backed authoring projections.

This crate consumes KIR and registered language services. It should not contain concrete source-language parsers, metamodel bundles, command-line UI, or product-specific behavior.
