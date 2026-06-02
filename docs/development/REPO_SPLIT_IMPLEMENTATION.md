# Repo Split Implementation

Status: physical language and CLI repositories seeded; core compatibility cleanup
still in progress.

Mercurio is moving toward a library-first split:

```text
mercurio-foundation
  language-neutral KIR, runtime, query, package, and host contracts

mercurio-sysml
  KerML and SysML language libraries that compile source to KIR

mercurio-cli
  executable host that registers language services and exposes commands

mercurio-examples
  examples, sample languages, corpora, and golden outputs
```

The split invariant is:

```text
mercurio-foundation must not depend on KerML or SysML.
KerML and SysML depend on core contracts and emit KIR.
Hosts choose which language services to register.
```

## Repository Responsibilities

### `mercurio-foundation`

Owns:

- `mercurio-kir`
- KIR validation, persistence, merging, and schema metadata
- graph, query, runtime, expression, and derived-property indexes
- language-neutral diagnostics and compile reports
- language service registration contracts
- package/library/profile/rulepack loading primitives when language-neutral

Does not own:

- SysML or KerML source syntax
- default `.sysml` or `.kerml` routing
- SysML standard-library paths
- SysML/KerML lowering rules
- concrete language services

### `mercurio-sysml`

Owns:

- `mercurio-kerml`
- `mercurio-sysml`
- KerML parser, resolver, and lowering
- SysML parser, resolver, and lowering
- Kernel and SysML library/profile/mapping resources
- `KermlLanguageService` and `SysmlLanguageService`
- SysML/KerML tests and language-specific maintainer tools

The repository may contain both KerML and SysML while still exposing them as
separate logical language services.

### `mercurio-cli`

Owns the command-line host:

- registers built-in language services
- configures active libraries/profiles/rulepacks
- routes source files by registered language extension
- runs KIR-only commands directly through core

The CLI is not core. It is one host environment that chooses to include KerML
and SysML.

### `mercurio-examples`

Owns:

- larger SysML/KerML examples
- sample unrelated language implementation
- host registration examples
- golden KIR outputs
- cross-repo integration tests

Core should keep only the fixtures required for local unit tests and smoke
checks.

## Current State

The shared host-facing language contract lives in `mercurio-language-contracts`.
The KerML, SysML, and shared language-frontend crates have moved to the sibling
`mercurio-sysml` repository. The CLI has been seeded as the sibling
`mercurio-cli` repository and now acts as a host that registers KerML and SysML
services directly.

`mercurio-foundation` still has transitional path dependencies on `mercurio-sysml`
for compatibility APIs such as source loading, linting, authoring helpers, and
legacy `frontend::*` re-exports. Removing those dependencies is now the next
cleanup cut; the crates no longer live in this repository.

The first contract is intentionally narrow:

```rust
trait LanguageService {
    fn language_id(&self) -> &str;
    fn extensions(&self) -> &[&str];
    fn compile(&self, source: &str, context: CompileContext<'_>)
        -> SemanticCompileReport<KirDocument>;
}
```

This keeps the core contract language-neutral. Parsing, ASTs, formatting, and
linting remain language-specific until those APIs are generalized without
assuming SysML-shaped syntax trees.

## Migration Order

1. Add neutral language service contracts and registry. Done.
2. Implement the contract for KerML and SysML. Done.
3. Convert CLI source commands to register services explicitly. Done.
4. Move compatibility `frontend::kerml` and `frontend::sysml` callers to direct
   `mercurio-kerml` and `mercurio-sysml` imports.
   Partly done for CLI and maintainer tools.
5. Remove `mercurio-foundation` compatibility dependencies on `mercurio-kerml` and
   `mercurio-sysml`.
6. Move KerML/SysML crates and language resources into `mercurio-sysml`. Done.
7. Move CLI into `mercurio-cli`. Seeded.
8. Move large examples and the sample unrelated language into
   `mercurio-examples`. Sample unrelated language added.

After step 5, the physical repository split should be a file movement and
dependency update, not an architecture redesign.

## Optional Packaging

`mpack` remains optional. Direct host registration is the primary runtime
mechanism. A package format can later provide installable discovery for:

- KIR libraries
- profiles
- mappings
- rulepacks
- generated wrappers
- optional executable language-service providers

Core should continue to work without `mpack`.
