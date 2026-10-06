# KIR

KIR is Mercurio's kernel interchange representation. It is the source-language-neutral semantic format that foundation loads, validates, merges, indexes, queries, packages, and projects into runtime views.

KIR is currently KerML-aligned: its common element kinds and relationship fields follow a modeling-kernel vocabulary of packages, types, features, definitions, usages, ownership, typing, and specialization. The OMG Kernel Modeling Language specification is the conceptual reference for that vocabulary; version-specific KerML libraries and source syntax remain outside foundation.

## Document Shape

A KIR document has metadata plus elements:

```json
{
  "metadata": {
    "kir_schema_version": "0.4"
  },
  "elements": [
    {
      "id": "pkg.Demo",
      "kind": "model.Package",
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
- Semantic layer is derived by readers from element kind and metadata hints; it is not persisted as a KIR element field.
- `properties`: scalar values, structured values, metadata, and references.

`MetamodelFeature` descriptors can declare a scalar attribute list with
`feature_kind: "attribute"` and an explicit `upper` other than 1. Its values
persist as arrays of scalar primitives. A descriptor may provide
`kir_owner_kind` or `kir_owner_kinds` to bind its field shape to exact element
kinds; other kinds continue using the document-wide field contract. Language
layers supply inheritance expansion when needed.

Language layers can also call `KirFieldRegistry::register_scoped_field` to
provide exact-kind contracts directly. Pass the complete registry to both
`normalized_for_persistence_with_registry` and
`validate_persisted_with_registry`; global field registration alone cannot
represent a field that is singular on one kind and a list on another.
These methods preserve the existing structural validation and normalization
rules. The language layer remains responsible for types, bounds, inverse
consistency and other metamodel semantics.

## Required Invariants

Foundation validation enforces:

- document metadata includes `kir_schema_version`,
- element ids are non-empty and unique,
- known reference fields have the expected shape,
- persisted elements include required stable identity properties such as `qualified_name`,
- unknown persisted fields are rejected unless they are inside explicit extension locations.

Foundation also produces semantic validation reports in warning mode for compile-cache,
workspace-library, and KPAR verification paths. These reports use metamodel feature facts
to detect issues such as unsupported feature ownership or incomplete relationship endpoints
without rejecting otherwise loadable KIR.

## Why KIR Exists

KIR lets every layer speak the same model format:

- language compilers emit KIR,
- packages can carry KIR directly,
- runtime services avoid reparsing source text,
- UI and adapters can inspect model data without owning a parser,
- tests can cover semantic behavior independently from source syntax.
