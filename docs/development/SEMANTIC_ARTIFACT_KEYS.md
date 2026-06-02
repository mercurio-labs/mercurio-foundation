# Semantic Artifact Keys

Status: canonical reference.

## Purpose

A semantic artifact key identifies a compiled semantic result and every input that can change that result.

Use this page as the shared reference for server caches, Datalog derived indexes, verification evidence, package provenance, and proposal validation.

## Principle

Never key semantic state by source revision alone.

The same commit can produce different KIR, diagnostics, derived indexes, views, validation results, or evidence when the compiler, standard library, mapping policy, dependency packages, rule packs, or verification policy changes.

## Base Source Inputs

A key should identify the source authority and exact source snapshot:

- `source_authority`: `local_files`, `local_git`, `external_git_indexed`, or `mercurio_hosted`
- `repository_id` or `workspace_id`
- `commit_sha`, `tag`, `branch + commit`, or local workspace revision fingerprint
- `proposal_id` and `overlay_digest` when compiling a proposal or draft overlay
- `source_tree_digest` when there is no immutable Git commit
- `workspace_config_digest`

For externally authoritative Git projects, prefer immutable commits over branch names. Branch names are useful request inputs, but artifact keys should resolve them to commits.

## Semantic Environment Inputs

Include every semantic input that can change compile or runtime interpretation:

- `compiler_version_or_digest`
- `kir_schema_version`
- `stdlib_digest`
- `dependency_package_digests`
- `baseline_library_digests`
- `mapping_rules_digest`
- `metamodel_rulepack_digest`
- `core_rulepack_digest`
- `validation_policy_digest`
- `extension_plugin_digests`
- `view_spec_digest`, for cached view results
- `plugin_service_digest`, for plugin-derived semantic services
- `simulator_version`, for simulation-derived evidence
- `verification_plan_digest`, for verification result keys
- `scenario_digest` and `assertion_digest`, for simulation verification actions
- `external_runner_version`, for delegated unit-test or code-quality actions

## Artifact Families

Different artifact families can share the same base key and append family-specific inputs.

### Compile Artifact

Minimum inputs:

- source snapshot
- workspace config
- compiler digest
- KIR schema version
- standard library digest
- dependency and baseline library digests
- mapping rules digest

Outputs:

- consolidated KIR
- diagnostics
- source-span index
- graph input metadata

### Derived Reasoning Artifact

Append:

- core rule-pack digest
- metamodel rule-pack digest
- validation or profile rule-pack digests
- extension plugin library and rule-pack digests when plugin rules participate

Outputs:

- derived indexes
- materialized facts
- explanation metadata
- validation predicates

### View Artifact

Append:

- view spec digest
- plugin id, version, package digest, and view spec digest for plugin-provided views
- semantic context id
- renderer-neutral projection version

Outputs:

- table, graph, matrix, tree, or dashboard projection DTOs
- warnings
- source anchors

Saved `.view` and `.diagram` files describe intent and presentation state. They are inputs to view artifacts, not cached semantic truth.

### Verification Artifact

Append:

- verification plan digest
- plugin id, version, package digest, ABI version, and service id for plugin-provided verification actions
- action runner versions
- simulator version, scenario digest, and assertion digest when applicable
- external runner version when delegating to another system

Outputs:

- pass/fail result
- diagnostics
- traces
- evidence records
- provider status payloads

## Staleness Rules

A cached artifact is stale if any key input changes.

Key comparisons should be explicit. Do not silently reuse an artifact across:

- compiler upgrades
- KIR schema changes
- standard library or package updates
- mapping file changes
- rule-pack changes
- validation policy changes
- proposal overlay changes
- scenario or assertion changes

If a source authority is mutable and not pinned to a commit or digest, materialize a stable snapshot first, then key artifacts from that snapshot.

## Non-Goals

Semantic artifact keys are not user-facing names, package versions, or permanent model identities.

They are cache and evidence identities. User-facing labels can point to them, but must not replace them where reproducibility matters.
