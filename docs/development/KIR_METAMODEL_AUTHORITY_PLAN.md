# KIR Metamodel Authority Plan

Status: proposed implementation plan.

## Goal

Make KIR the authoritative representation for both model data and metamodel data.
Runtime registries, reflection APIs, UI inspectors, validators, and query helpers
should read metamodel facts from KIR rather than from a separate compact schema or
hard-coded Rust tables.

The target is Ecore-like structural metamodel expressiveness without adopting the
EMF runtime model.

## Current State

Metamodel information is currently distributed across several sources:

- The Pilot/EMF stdlib export captures library elements, EMF class names, selected
  scalar properties, and relationships such as `owner`, `members`, `features`,
  `specializes`, and `type`.
- `pilot_constructs.seed.json` maps textual SysML/KerML constructs and keywords
  to Pilot metaclasses and default semantic anchors.
- `kir_emission.seed.json` maps those metaclasses to Mercurio KIR element shapes.
- `MetamodelAttributeRegistry` infers declared attributes by scanning KIR graph
  elements, reading each metatype element's `features`, and looking at each
  feature's `declared_name` and `type`.

This gives useful inspection today, but the structural feature model is implicit.
Multiplicity, containment/reference classification, read-only/derived/transient
flags, opposites, and default values are not first-class metamodel facts in KIR.

## Design Principle

KIR is the source of truth. Registries are indexes.

```text
Pilot/stdlib import
  -> authoritative KIR metamodel elements and properties
  -> runtime graph
  -> metamodel feature registry/index
  -> reflection, validation, query, UI, codegen
```

No runtime feature should need a separate side schema to understand the metamodel.
A compact schema may still be generated later for documentation or interchange,
but it must not become authoritative.

## Derived Feature Execution Boundary

KIR and packaged standard libraries may declare derived-feature specs, but they
must not contain executable derivation logic. All derived-feature execution lives
in `mercurio-core`.

Manifest rules may select generic core operators such as alias, subset,
subset-chain, inverse, name, qualified name, and library provenance. Truly
exceptional derived features may use `kind: "native"` with a stable function
name, but that name must resolve to a Rust implementation registered in
`mercurio-core`.

Unknown rule kinds, malformed manifests, and unknown native function names are
errors. There is intentionally no Rust scripting, eval, or package-provided code
path in KIR or KPAR artifacts.

## Proposed KIR Shapes

Use ordinary KIR elements and properties for metamodel classes, structural
features, datatypes, enumerations, and constraints.

### Metamodel Class

```json
{
  "id": "SysML::Systems::PartUsage",
  "kind": "MetamodelClass",
  "layer": 1,
  "properties": {
    "declared_name": "PartUsage",
    "qualified_name": "SysML::Systems::PartUsage",
    "is_abstract": false,
    "is_interface": false,
    "specializes": ["SysML::Systems::Usage"],
    "features": [
      "metafeature.SysML::Systems::PartUsage.part_definition"
    ]
  }
}
```

Existing library class-like elements may keep their current ids. The migration
should avoid breaking runtime references; if `kind` changes are too disruptive,
classification can start as an additional property such as
`metamodel_element_kind = "class"` before normalizing `kind`.

### Metamodel Feature

```json
{
  "id": "metafeature.KerML::Core::Feature.type",
  "kind": "MetamodelFeature",
  "layer": 1,
  "properties": {
    "declared_name": "type",
    "qualified_name": "KerML::Core::Feature::type",
    "owner": "KerML::Core::Feature",
    "feature_kind": "reference",
    "type": "KerML::Core::Type",
    "lower": 0,
    "upper": "*",
    "is_containment": false,
    "is_container": false,
    "is_derived": false,
    "is_read_only": false,
    "is_transient": false,
    "is_volatile": false,
    "opposite": null,
    "default_value": null,
    "kir_property": "type"
  }
}
```

`kir_property` is the canonical property key used on ordinary KIR model elements.
It preserves mappings such as EMF `declaredName` to KIR `declared_name`.

### Metamodel Datatype

```json
{
  "id": "Kernel::String",
  "kind": "MetamodelDataType",
  "layer": 1,
  "properties": {
    "declared_name": "String",
    "value_kind": "string"
  }
}
```

### Metamodel Enumeration

```json
{
  "id": "Kernel::DirectionKind",
  "kind": "MetamodelEnum",
  "layer": 1,
  "properties": {
    "declared_name": "DirectionKind",
    "literals": ["in", "out", "inout"]
  }
}
```

## Runtime Query Examples

Find features declared by a metatype:

```text
match ?feature kind "MetamodelFeature"
where ?feature.owner = "SysML::Systems::PartUsage"
select ?feature.declared_name, ?feature.type, ?feature.lower, ?feature.upper
```

Find editable reference features:

```text
match ?feature kind "MetamodelFeature"
where ?feature.feature_kind = "reference"
where ?feature.is_read_only = false
select ?feature.owner, ?feature.declared_name, ?feature.type
```

Inspect a model element reflectively:

```text
element -> metatype
metatype -> features plus inherited features
feature.kir_property -> direct/effective value on element
```

## Implementation Phases

### Phase 1: Preserve Metamodel Feature Facts

Extend the Pilot stdlib export/import path to capture EMF structural feature
metadata and write it into KIR.

Candidate facts:

- declaring class
- feature name
- KIR property name
- feature kind: attribute, reference, containment, derived
- value type or target type
- lower and upper multiplicity
- containment/container flags
- derived/read-only/transient/volatile flags
- opposite feature where available
- default value where available

Implementation anchors:

- `../mercurio-sysml/tools/pilot-exporter/src/main/java/dev/mercurio/pilot/PilotStdlibExporter.java`
- `crates/mercurio-core/src/frontend/pilot.rs`
- `crates/mercurio-tools/src/bin/import_pilot_stdlib.rs`
- `crates/mercurio-tools/src/bin/build_stdlib_release.rs`

Deliverable:

- bundled stdlib KIR contains explicit metamodel feature elements or equivalent
  first-class KIR facts.

### Phase 2: Replace Attribute Registry With Feature Registry

Introduce `MetamodelFeatureRegistry` as an index over KIR metamodel facts.

Required API shape:

```rust
registry.class("SysML::Systems::PartUsage")
registry.declared_features_for("SysML::Systems::PartUsage")
registry.all_features_for("SysML::Systems::PartUsage")
registry.feature_by_property("SysML::Systems::PartUsage", "type")
```

Keep `MetamodelAttributeRegistry` as a compatibility wrapper until callers move.

Implementation anchors:

- `crates/mercurio-core/src/metamodel.rs`
- `crates/mercurio-core/src/element_view.rs`
- `crates/mercurio-core/src/views.rs`
- `crates/mercurio-views/src/lib.rs`
- `crates/mercurio-wasm/src/lib.rs`
- `crates/mercurio-python/src/lib.rs`

Deliverable:

- element details include feature kind, type id/label, multiplicity, containment,
  reference, derived, read-only, and declared/inherited origin.

### Phase 3: Runtime Reflection API

Add a small reflection facade over graph + registry.

Target capabilities:

- `metatype(element_id)`
- `declared_features(metatype_id)`
- `all_features(metatype_id)`
- `feature_value(element_id, feature_name)`
- `effective_feature_value(element_id, feature_name)`
- `validate_element(element_id)`

Implementation anchors:

- `crates/mercurio-core/src/element_view.rs`
- `crates/mercurio-core/src/runtime.rs`
- `crates/mercurio-core/src/views.rs`

Deliverable:

- UI, WASM, Python, and query code can inspect metamodel facts without knowing
  Pilot export conventions.

### Phase 4: Validation

Use KIR metamodel facts to validate ordinary KIR elements.

Initial validations:

- unknown property for metatype
- required feature missing
- scalar/list mismatch
- reference target missing
- reference target does not conform to declared type
- multiplicity lower/upper violation
- read-only feature mutated by authoring path

Implementation anchors:

- `crates/mercurio-core/src/ir.rs`
- `crates/mercurio-core/src/mutation.rs`
- `crates/mercurio-core/src/authoring.rs`
- `crates/mercurio-core/src/semantic_compare.rs`

Deliverable:

- validation diagnostics cite the metamodel feature id and the offending model
  element/property.

### Phase 5: Query Ergonomics

Add query helpers or views for common metamodel questions while keeping the
underlying data queryable as ordinary KIR.

Candidate helpers:

- `from metamodel_classes`
- `from metamodel_features`
- `where feature.owner conforms_to "SysML::Systems::Usage"`
- `select feature.name, feature.type, feature.multiplicity`

Implementation anchors:

- `crates/mercurio-core/src/query.rs`
- `docs/user/QUERY_EVALUATE.md`

Deliverable:

- runtime metamodel queries are concise, documented, and test-covered.

## Migration Strategy

Start additive:

1. Preserve current stdlib KIR ids and existing properties.
2. Add explicit metamodel facts without removing inferred behavior.
3. Teach `MetamodelFeatureRegistry` to prefer explicit KIR facts and fall back to
   current inference.
4. Move views and bindings onto the new registry.
5. Add coherence tests.
6. Remove fallback inference only after the bundled stdlib has complete feature
   facts.

## Coherence Tests

Minimum test set:

- `package A { part def Y {} part y : Y; }` reports metatype and feature table
  for package, definition, and usage.
- `PartUsage.type` is a reference feature targeting `Type`.
- `Feature.is_unique` is a boolean attribute.
- inherited features for `PartUsage` include features from `Usage`, `Feature`,
  `Type`, `Namespace`, and `Element` ancestry as applicable.
- required multiplicities are enforced when present.
- unknown KIR properties produce diagnostics only when validation is run in strict
  mode.

## Non-Goals

This plan does not add:

- EMF notifications/adapters
- generated Java-style implementation classes
- EMF resource sets
- lazy proxy resolution
- EMF editing domain or command stack
- automatic bidirectional reference maintenance as a runtime side effect

Those are runtime services and can be considered later if Mercurio needs them.
The goal here is Ecore-like metamodel expressiveness with Mercurio-native KIR,
runtime, query, and validation semantics.
