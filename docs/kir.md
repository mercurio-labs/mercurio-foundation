# KIR

KIR is Mercurio's kernel interchange representation. It is the language-neutral semantic format that foundation loads, validates, merges, indexes, queries, packages, and projects into runtime views.

## Document Shape

A KIR document has metadata plus elements:

```json
{
  "metadata": {
    "kir_schema_version": "0.2"
  },
  "elements": [
    {
      "id": "pkg.Demo",
      "kind": "model.Package",
      "layer": 2,
      "properties": {
        "qualified_name": "Demo",
        "declared_name": "Demo",
        "members": ["type.Demo.Vehicle"]
      }
    }
  ]
}
```

## Element Fields

- `id`: stable element identity inside the document.
- `kind`: semantic kind supplied by a metamodel or profile.
- `layer`: coarse semantic layer used by tools and projections.
- `properties`: scalar values, structured values, metadata, and references.

## Required Invariants

Foundation validation enforces:

- document metadata includes `kir_schema_version`,
- element ids are non-empty and unique,
- known reference fields have the expected shape,
- persisted elements include required stable identity properties such as `qualified_name`,
- unknown persisted fields are rejected unless they are inside explicit extension locations.

## Why KIR Exists

KIR lets every layer speak the same model format:

- language compilers emit KIR,
- packages can carry KIR directly,
- runtime services avoid reparsing source text,
- UI and adapters can inspect model data without owning a parser,
- tests can cover semantic behavior independently from source syntax.
