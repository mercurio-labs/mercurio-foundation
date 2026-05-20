# KIR Schema Roadmap

Status: active contract roadmap.

## Purpose

KIR is already the semantic handoff between frontends and runtime services. This roadmap defines the remaining work needed to make that handoff a stricter, versioned, testable schema.

The current implementation contract is documented in [KIR Specification](KIR_SPEC.md).

## Goals

- Define a formal JSON Schema for `KirDocument` and `KirElement`.
- Version the KIR document contract.
- Stabilize `expression_ir`.
- Stabilize source provenance metadata.
- Define required property sets for supported semantic kinds.
- Define compatibility and migration behavior for breaking changes.

## Current Flexible Areas

KIR currently allows flexible JSON properties. That is useful while frontend coverage grows, but downstream services increasingly rely on stable fields:

- graph edge discovery
- runtime expression evaluation
- constraints
- Datalog fact extraction
- views and diagrams
- semantic comparison
- server cache identity
- verification evidence
- package provenance

The schema work should tighten these contracts incrementally without blocking current compiler growth.

## Schema Milestones

### Milestone 1: Document Envelope

Define:

- required `version`
- required `elements`
- optional `metadata`
- metadata fields for source set, compiler, stdlib, mappings, and schema version
- duplicate-id and supported-layer validation

Exit condition:

- KIR documents declare a schema version and validate through one schema entry point.

### Milestone 2: Element Core

Define:

- `id`
- `kind`
- `layer`
- `properties`
- `properties.metadata`
- source provenance fields

Exit condition:

- Every KIR element has stable identity, kind, layer, and validated provenance shape when provenance is present.

### Milestone 3: Reference Fields

Define which properties are references and whether they are scalar ids, arrays, or structured references.

Initial candidates:

- `owner`
- `owning_type`
- `type`
- `features`
- `members`
- `specializes`
- `subsets`
- `redefines`
- `imports`

Exit condition:

- Graph edge discovery no longer depends only on ad hoc string matching for known semantic reference fields.

### Milestone 4: Expression IR

Define the `expression_ir` schema:

- `literal`
- `self`
- `path`
- `tuple`
- `unary`
- `binary`
- `call`
- comparison and constraint forms, if kept in expression IR
- source span or provenance hooks where needed

Exit condition:

- Frontend emission, runtime evaluation, constraint rendering, simulation guards, and verification assertions share one expression contract.

### Milestone 5: Kind Profiles

Define required and optional properties for high-value kinds first:

- packages and namespaces
- part definitions and usages
- item definitions and usages
- attribute usages
- requirement definitions and usages
- satisfy and verify relationships
- constraint definitions and usages

Exit condition:

- The most common UI, query, and verification workflows can rely on required fields instead of defensive guessing.

### Milestone 6: Compatibility And Migration

Define:

- compatibility window for old KIR artifacts
- migration behavior between schema versions
- warnings versus hard errors
- package compatibility policy
- cache invalidation rules tied to schema version

Exit condition:

- Breaking KIR changes have a documented path for existing artifacts, packages, and semantic caches.

## Relationship To Artifact Keys

Every semantic artifact key should include `kir_schema_version`. See [Semantic Artifact Keys](SEMANTIC_ARTIFACT_KEYS.md).

Changing the KIR schema version should invalidate compile artifacts, derived indexes, view caches, verification evidence, and package artifacts unless an explicit migration path preserves compatibility.

## Non-Goals

- Freezing every SysML/KerML concept before frontend coverage matures.
- Replacing KIR with parser ASTs or metamodel-native objects.
- Requiring product UI code to understand parser-specific source forms.

## Near-Term Recommendation

Add `version` to the KIR envelope, formalize source provenance and `expression_ir`, then define kind profiles for requirements and part/attribute workflows because they feed views, constraints, and verification.
