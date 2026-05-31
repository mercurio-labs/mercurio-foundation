# Language Module Architecture

Status: initial in-tree boundary.

Mercurio separates the shared model/runtime stack from concrete source languages:

```text
KIR + graph/runtime/query
  <- shared frontend infrastructure
      <- KerML language module
          <- SysML language module
          <- future KerML-based language modules
```

KerML is the semantic foundation. It can parse without any library context and can compile trivial self-contained models with an empty context. Non-trivial KerML should use a Kernel/KerML baseline library. SysML should use Kernel/KerML plus SysML libraries, mappings, aliases, rulepacks, and profile data.

## Current Boundary

- `frontend::ast::ParsedModule` is the shared parsed module type.
- `frontend::ast::SysmlModule` remains as a compatibility alias.
- `language::SourceLanguage` is the single source language enum for linting, formatting, profiles, and registry dispatch.
- `mercurio-language-contracts` owns the first extracted language-neutral contracts: `SourceLanguage` and `SemanticConcept`.
- `language::LanguageModule` defines parse, compile, mappings, extensions, and default baseline behavior.
- `language::kerml::parser` and `language::sysml::parser` are the language-facing parser/compiler entrypoint modules inside `mercurio-core`.
- `language::KermlLanguageModule` and `language::SysmlLanguageModule` are in-tree module implementations that delegate through those parser modules.
- `mercurio-kerml` exposes the KerML facade contract: KerML parse/compile entrypoints and Kernel baseline helpers.
- `mercurio-sysml` exposes the SysML facade contract: SysML parse/recovery/compile entrypoints and SysML baseline helpers.
- `language::BaselineLibrary` distinguishes empty, Kernel, SysML, and custom library contexts.
- `default_sysml_library_path()` names the legacy monolithic SysML stdlib artifact; `default_stdlib_path()` remains a compatibility wrapper for it.
- `default_sysml_delta_library_path()` names the split SysML-only delta artifact, which intentionally excludes KerML/Kernel elements.
- `default_sysml_library_path()` prefers `MERCURIO_SYSML_LIBRARY_PATH` and falls back to the legacy `MERCURIO_STDLIB_PATH`.
- `default_sysml_rulepack_path()` prefers `MERCURIO_SYSML_RULEPACK_PATH` and falls back to the legacy `MERCURIO_STDLIB_RULEPACK_PATH`.
- `default_kernel_library_path()` points to the committed KerML/Kernel KIR artifact and can be overridden with `MERCURIO_KERNEL_LIBRARY_PATH`.
- `mercurio-kerml` and `mercurio-sysml` are facade crates over the in-tree language modules. They establish the crate boundary before the parser implementation is physically moved out of `mercurio-core`.
- `cargo run -p mercurio-tools --bin generate_language_baselines` regenerates both committed split artifacts from the legacy monolithic stdlib: `resources/kernel/kerml-kernel.kir.json` and `resources/sysml/sysml-library.kir.json`.
- `build_stdlib_release` emits all three KPAR shapes during migration: the legacy monolithic stdlib KPAR, the KerML/Kernel KPAR, and the SysML delta KPAR.

## Migration Rules

1. Keep public SysML/KerML wrappers while moving internals to registry dispatch.
2. Prefer `library_context` for generic compiler inputs; reserve `stdlib` for concrete packaged standard libraries.
3. Keep KerML independent of SysML library defaults.
4. Move SysML aliases, mappings, rulepacks, and bundled libraries behind the SysML module boundary before considering a crate or repo split.
5. Keep KIR as the canonical output for every language module.

## Default Loading

When no project descriptor or explicit standard-library override is present, source-oriented commands should load the baseline selected by the requested language module:

- KerML: committed KerML/Kernel baseline, or the file pointed to by `MERCURIO_KERNEL_LIBRARY_PATH`.
- SysML: merged KerML/Kernel baseline plus the committed SysML delta artifact.

Project descriptors and explicit `--stdlib` options take precedence over language defaults. Descriptor resolution also has a language-aware entrypoint, `resolve_project_context_for_language`, so descriptor-less KerML uses the Kernel baseline while the compatibility `resolve_project_context` keeps SysML as the default. The legacy monolithic `default_stdlib_path()` artifact is still loadable as raw KIR for compatibility and release-pipeline tooling.

## Extraction Sequence

The facade crates intentionally do not own parser implementation yet. A direct move today would create a cycle because the parser, resolver, AST, transpiler, KIR, project loading, and runtime contracts still live together in `mercurio-core`.

Move implementation ownership in this order:

1. Keep `mercurio-kerml` and `mercurio-sysml` as narrow facade crates and migrate external callers to those crates.
2. Continue expanding `mercurio-language-contracts` from language concepts into AST, diagnostics, lexer contracts, resolver context, and KIR-facing compile traits.
3. Move KerML parser/compiler implementation into `mercurio-kerml` once the shared AST/diagnostic/resolver contracts are independent of `mercurio-core`.
4. Move SysML parser/compiler implementation into `mercurio-sysml`, depending on `mercurio-kerml` only for KerML foundation behavior where needed.
5. Leave `mercurio-core` as the orchestration crate for project/library resolution, model stack loading, runtime, authoring, and compatibility re-exports.
