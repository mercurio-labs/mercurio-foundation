# Mercurio Foundation Documentation

Mercurio Foundation is a KerML-aligned, source-language-neutral modeling substrate. It stores models as KIR, builds graph/runtime views over those models, and exposes contracts that language-specific repositories can implement.

## Sections

- [Philosophy](philosophy.md): why foundation exists, what it owns, and how it compares to classic modeling frameworks.
- [KIR](kir.md): the core interchange representation and its invariants.
- [Crates](crates.md): intent and ownership of each crate in this workspace.
- [Language Services](language-services.md): how source languages connect to foundation without becoming foundation dependencies.
- [Sample Language](sample-language.md): a small source-language service that compiles custom syntax into KIR.
- [Examples](examples.md): small Rust snippets for KIR, graph, runtime, and language-service usage.
- [Performance](performance.md): large-model KIR timing and memory harness.

## Boundary

Foundation should not know any specific source language, metamodel lineage, command-line host, UI adapter, or product workflow. Those layers consume foundation through KIR and language-service contracts.
