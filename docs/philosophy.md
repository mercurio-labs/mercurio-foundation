# Foundation Philosophy

Mercurio Foundation is the model substrate underneath language-specific tooling. Its job is to provide a stable representation and runtime for models without deciding which surface language, metamodel version, editor, or product owns those models.

Foundation is source-language neutral, not metamodel-agnostic. Its semantic vocabulary is aligned with the OMG Kernel Modeling Language (KerML): packages, types, features, definitions, usages, specialization, typing, ownership, multiplicity, and derived metamodel views are first-class modeling concepts in foundation. The current normative reference point is the [OMG KerML 1.0 specification](https://www.omg.org/spec/KerML/1.0), formally published in September 2025.

## Design Intent

Foundation treats a model as structured semantic data:

- A model is a set of identified elements.
- Elements have kinds, layers, scalar properties, and references to other elements.
- References form a graph.
- Runtime services query that graph, materialize derived facts, evaluate expressions, and build projections.
- Source languages are adapters that compile text or other authoring forms into KIR.

This keeps the substrate reusable. A language repository can provide parsing and metamodel-specific compilation. A product can provide persistence, collaboration, UI, and authorization. Foundation remains the common semantic core.

## AI-Assisted Semantic Operations

Mercurio is designed around a different assumption than classic modeling frameworks: modern engineering tools will not only render and edit models, they will collaborate with humans over model intent.

An AI assistant cannot safely work from raw text alone. It needs a semantic workspace that can answer:

- What does this element mean?
- Where is it defined?
- What depends on it?
- What can be changed safely?
- What would this mutation affect?
- Is the result valid?
- What evidence supports this conclusion?

Mercurio's core role is to provide that workspace. KIR, graph indexes, source spans, semantic diffs, mutation proposals, validation reports, reasoning artifacts, and evidence graphs are not auxiliary features; they are the substrate for AI-assisted systems engineering.

In this model, domain languages plug in declarative semantic capabilities. Foundation owns workspace identity, graph traversal, transactions, diagnostics, evidence, and interface orchestration. A language profile owns domain meaning, such as SysML definition and usage semantics, feature typing, specialization, subsetting, redefinition, allocation, requirement traceability, behavior, and simulation readiness.

## KerML Alignment

Foundation should be honest about the shape of that core:

- KIR uses metamodel-style `kind` values such as package, type, feature, definition, and usage.
- Graph and runtime services understand common modeling relationships such as ownership, membership, specialization, and typing.
- Metamodel views expose classes, features, attributes, inherited values, and specialization chains.
- Authoring and mutation APIs use package, definition, usage, and type concepts.

That alignment is deliberate. KerML is the modeling kernel underneath SysML v2 and provides a useful conceptual baseline for a reflective modeling runtime.

The boundary is version ownership. Foundation should not bundle a specific KerML library release, parse KerML textual notation, or hardcode a complete versioned KerML metamodel. Version-specific KerML/SysML libraries, parsers, and lowering rules belong in language repositories and packages that compile into KIR.

## Modeling Framework Analogy

The closest familiar shape is a modeling framework that separates:

- metamodel concepts from model instances,
- model storage from editor presentation,
- generated or interpreted APIs from raw serialized data,
- and resource loading from application-specific tools.

Foundation follows that separation, but it does not require a specific code generation strategy or editor stack. It is closer to a small, explicit runtime contract:

- KIR is the persisted model representation.
- `Graph` is the navigable relationship view.
- `Runtime` is the deterministic evaluation layer.
- `LanguageService` is the contract for plugging in source-language compilers.

The AI-facing distinction is that Foundation also exposes semantic operations, not just reflective access. The same substrate can support a CLI, language server, workbench UI, MCP server, or AI agent because each interface can ask the same validated questions and receive the same source-linked evidence.

## Contract Compared To EMF

This is a comparison of responsibilities, not a dependency or a promise to mirror any framework exactly.

| Concern | Foundation Contract | Comparable Modeling-Framework Idea |
| --- | --- | --- |
| Model instance | `KirDocument` and `KirElement` | Serialized model resource |
| Metamodel identity | element `kind`, profile metadata, and external language packages | EClass / structural feature identity |
| References | KIR properties classified by the field registry and projected as graph edges | Cross-resource or intra-resource references |
| Resource loading | KIR IO, package resolution, and language-service compilation hooks | Resource sets and factories |
| Runtime navigation | `Graph`, `Runtime`, `QueryEngine`, views | Reflective navigation and generated APIs |
| Source language | External `LanguageService` implementation | Parser/resource factory outside the core substrate |

The key rule is that foundation owns the reflective substrate and deterministic services, while language repositories own source syntax and metamodel-specific compilation.
