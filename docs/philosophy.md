# Foundation Philosophy

Mercurio Foundation is the model substrate underneath language-specific tooling. Its job is to provide a stable representation and runtime for models without deciding which surface language, metamodel version, editor, or product owns those models.

## Design Intent

Foundation treats a model as structured semantic data:

- A model is a set of identified elements.
- Elements have kinds, layers, scalar properties, and references to other elements.
- References form a graph.
- Runtime services query that graph, materialize derived facts, evaluate expressions, and build projections.
- Source languages are adapters that compile text or other authoring forms into KIR.

This keeps the substrate reusable. A language repository can provide parsing and metamodel-specific compilation. A product can provide persistence, collaboration, UI, and authorization. Foundation remains the common semantic core.

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
