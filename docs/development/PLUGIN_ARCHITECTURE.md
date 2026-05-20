# Plugin Architecture

Status: architecture proposal.

## Purpose

This document describes how Mercurio should discover, install, verify, load, cache, and execute plugins.

Plugins extend Mercurio downstream of the source-to-KIR compiler boundary. They may add SysML/KerML libraries, rule packs, views, verification actions, and sandboxed executable services. They must not replace the core parser, mutate accepted source directly, or redefine standard KIR semantics.

```text
registry / local path
        |
        v
 plugin package
        |
        v
 verify + unpack + index
        |
        v
 libraries + rule packs + views + services
        |
        v
 KIR / graph / runtime derived services
        |
        v
 diagnostics, evidence, views, analysis results
```

## Design Boundary

Mercurio's core architecture remains:

```text
Source authority -> KIR compiler/runtime -> semantic services -> user surfaces
```

Plugins live in the semantic-service layer. A plugin may consume compiled KIR, graph facts, runtime context values, and derived indexes. A plugin may produce diagnostics, facts, derived artifacts, views, verification evidence, or proposed semantic edits through explicit host APIs.

Plugins must not:

- become an alternate source authority
- bypass source-to-KIR compilation for normal operation
- silently reinterpret standard SysML or KerML semantics
- mutate source files without an explicit host-mediated proposal or authoring operation
- depend on parser-specific AST access as their primary runtime interface

## Plugin Package

The primary package unit should be a versioned archive, for example `.mpack`.

```text
contract-analysis.mpack
  extension.json
  libraries/
    contracts.kpar
  rules/
    contracts.rulepack.json
  views/
    contract_table.view.json
    contract_graph.view.json
  wasm/
    contracts.wasm
  docs/
    README.md
```

The package should be content-addressable and reproducible. The manifest declares all files the host is allowed to load.

## Manifest

`extension.json` is the plugin manifest.

```json
{
  "id": "org.mercurio.contracts",
  "version": "0.1.0",
  "name": "Contract Analysis",
  "description": "Assume-guarantee compositional contract analysis over KIR.",
  "requires": {
    "mercurio": ">=0.1.0",
    "kir": ">=0.1",
    "pluginAbi": "0.1"
  },
  "libraries": [
    "libraries/contracts.kpar"
  ],
  "rulepacks": [
    "rules/contracts.rulepack.json"
  ],
  "views": [
    "views/contract_table.view.json",
    "views/contract_graph.view.json"
  ],
  "services": [
    {
      "id": "contract.compose",
      "runtime": "wasm",
      "module": "wasm/contracts.wasm",
      "function": "compose"
    },
    {
      "id": "contract.refine",
      "runtime": "wasm",
      "module": "wasm/contracts.wasm",
      "function": "refine"
    }
  ],
  "verificationActions": [
    {
      "id": "contract_compatibility",
      "service": "contract.compose"
    },
    {
      "id": "contract_refinement",
      "service": "contract.refine"
    }
  ],
  "capabilities": {
    "filesystem": "none",
    "network": "none",
    "sourceMutation": false,
    "nondeterminism": false
  }
}
```

Manifest fields should be stable and minimal. Rich plugin-specific configuration belongs in project descriptors, verification plans, or service request payloads rather than in the package manifest.

## Discovery

Mercurio should support registry-backed and local plugin discovery.

Registry configuration belongs in project, workspace, or server configuration:

```json
{
  "pluginRegistries": [
    "https://registry.mercurio.dev/index.json",
    "https://company.example.com/mercurio/plugins/index.json",
    "file://./.mercurio/plugins/index.json"
  ]
}
```

A registry index lists metadata and immutable version records:

```json
{
  "plugins": [
    {
      "id": "org.mercurio.contracts",
      "name": "Contract Analysis",
      "latest": "0.1.0",
      "description": "Assume-guarantee contract analysis.",
      "versions": {
        "0.1.0": {
          "url": "https://registry.mercurio.dev/org.mercurio.contracts/0.1.0/plugin.mpack",
          "sha256": "sha256-value",
          "signature": "signature-value",
          "requires": {
            "mercurio": ">=0.1.0",
            "kir": ">=0.1",
            "pluginAbi": "0.1"
          }
        }
      }
    }
  ]
}
```

Local development should support direct installation from an archive or manifest directory:

```text
mercurio plugin install ./target/contract-analysis.mpack
mercurio plugin install ../plugins/contracts/extension.json
```

## Resolution And Project Pinning

Projects should pin plugin identity, version, and digest in `mercurio-project.json`.

```json
{
  "extensions": [
    {
      "id": "org.mercurio.contracts",
      "version": "0.1.0",
      "sha256": "sha256-value"
    }
  ]
}
```

Pinning makes desktop, server, CI, and proposal validation resolve the same executable semantics. Floating plugin versions are acceptable as user input, but resolved semantic artifacts should record immutable plugin versions and digests.

Resolution should:

- select a version satisfying project constraints
- resolve all plugin dependencies
- detect incompatible ABI, KIR, Mercurio, or stdlib requirements
- produce a lock record with exact package URLs and digests
- avoid loading multiple incompatible versions of the same plugin into one semantic session unless explicitly supported

## Obtain And Verify

Plugin installation follows:

```text
download or read local package
        |
        v
verify digest
        |
        v
verify signature and registry policy
        |
        v
validate archive layout
        |
        v
validate manifest
        |
        v
validate declared KPAR, rule packs, views, and WASM exports
        |
        v
unpack to content-addressed cache
```

The verifier should reject:

- digest mismatches
- missing or invalid signatures when a registry requires signing
- manifest ID/version mismatches
- archive path traversal
- undeclared executable files
- declared services whose WASM exports are missing
- rule packs or KPAR packages that fail schema validation
- capability requests disallowed by workspace or server policy

## Local Cache Layout

The cache should separate original packages, unpacked packages, compiled runtime artifacts, and project-specific derived state.

```text
~/.mercurio/plugins/
  registry-cache/
    registry-key/
      index.json
  blobs/
    sha256-abc123/
      plugin.mpack
  unpacked/
    org.mercurio.contracts/
      0.1.0/
        extension.json
        libraries/
        rules/
        views/
        wasm/
  compiled-wasm/
    wasm-runtime-key/
      sha256-abc123.cache

~/.mercurio/projects/
  project-id/
    extension-state/
      org.mercurio.contracts/
```

Cache keys should include:

- plugin id
- plugin version
- plugin package digest
- plugin ABI version
- Mercurio host version or digest
- KIR schema version
- WASM runtime version
- stdlib digest
- dependency package digests
- rule-pack digests

## Load Phases

Plugins should load in two phases.

Static load:

- read manifest
- register KPAR libraries
- register rule packs
- register view specs
- register verification action declarations
- index service declarations

Runtime load:

- instantiate WASM only when a service is invoked
- validate request shape
- apply resource limits
- pass a bounded input payload
- collect structured outputs
- dispose or pool the module according to host policy

Lazy runtime loading keeps startup predictable and avoids running plugin code just because a project declares the plugin.

## WASM Runtime

WASM should be the default executable plugin runtime.

Native DLLs or shared libraries may be useful for controlled deployments, but they should not be the primary extension model because they are OS-specific and harder to sandbox. WASM gives Mercurio one portable runtime across desktop, server, CI, and eventually browser contexts.

Default WASM capabilities:

- no filesystem access
- no network access
- no wall-clock time except host-provided deterministic values
- no nondeterministic random values except host-provided seeded values
- memory limit
- timeout
- fuel or instruction budget where supported
- JSON or another stable ABI payload boundary

The host should not expose raw Rust structs as the plugin ABI. The ABI should use stable payloads and explicit schema versions.

## Service ABI

A service invocation should be request/response oriented.

Host input:

```json
{
  "abiVersion": "0.1",
  "service": "contract.compose",
  "semanticArtifact": {
    "kirSchemaVersion": "0.1",
    "artifactDigest": "semantic-artifact-digest"
  },
  "request": {},
  "kir": {},
  "graphFacts": [],
  "contextValues": {}
}
```

Plugin output:

```json
{
  "status": "ok",
  "result": {},
  "diagnostics": [
    {
      "severity": "error",
      "code": "contract.incompatible",
      "message": "Guarantee does not discharge downstream assumption.",
      "elementIds": ["component.a", "component.b"]
    }
  ],
  "evidence": [
    {
      "kind": "contract_composition",
      "elementIds": ["component.a", "component.b"],
      "details": {}
    }
  ]
}
```

All diagnostics and evidence should reference KIR element IDs when possible so user surfaces can trace results back to authored source spans.

## Extension Inputs

Plugins may contribute several input types.

### Model Libraries

KPAR model libraries define vocabulary using SysML/KerML-native mechanisms such as packages, metadata definitions, annotations, and profile-like libraries.

Example:

```sysml
package Contracts {
  metadata def ContractComponent;
  metadata def ContractInput;
  metadata def ContractOutput;
  metadata def Assumption;
  metadata def Guarantee;
}
```

### Rule Packs

Rule packs contribute derived relational semantics such as normalized extension facts, validation predicates, and view support.

Example facts:

```text
contract_component(Component)
contract_input(Component, Variable)
contract_output(Component, Variable)
contract_assumption(Component, Constraint)
contract_guarantee(Component, Constraint)
```

### Views

Views declare renderer-neutral projections over KIR, graph facts, derived indexes, and plugin service outputs. Saved views describe intent and presentation state; they are not source authority.

### Verification Actions

Verification actions bind plugin services into Mercurio verification plans. A plugin-provided action should declare stable action type, expected inputs, resource policy, output evidence shape, and pass/fail status mapping.

## Security And Policy

Workspace, server, and CI policy should control which plugins can run.

Policy knobs:

- allowed registries
- trusted signing keys
- allowed plugin IDs or publishers
- maximum WASM memory
- maximum runtime duration
- network denied or allowlisted
- filesystem denied or scoped
- source mutation denied or host-mediated only
- deterministic execution required for verification evidence

The default policy should be deny-by-default for executable privileges beyond pure computation.

## Artifact Keys

Plugin identity must participate in semantic artifact keys whenever plugin output affects derived state, validation, views, or verification evidence.

At minimum, derived artifacts should record:

- plugin id
- plugin version
- plugin package digest
- plugin ABI version
- service id and service version when applicable
- plugin rule-pack digests
- plugin library package digests
- plugin view spec digests
- plugin runtime configuration digest

This prevents stale evidence when a plugin changes without source files changing.

## Contract Analysis Example

A Pacti-inspired contract analysis extension is a representative plugin:

```text
org.mercurio.contracts
  libraries/contracts.kpar
    ContractComponent, ContractInput, ContractOutput, Assumption, Guarantee
  rules/contracts.rulepack.json
    contract facts and compatibility predicates
  wasm/contracts.wasm
    compose, refine, merge, diagnose
  views/
    contract table and contract graph projections
  verification actions/
    contract_compatibility, contract_refinement
```

Runtime flow:

```text
user imports Contracts::*
        |
        v
SysML/KerML compiles to KIR
        |
        v
extension rule pack derives contract facts
        |
        v
WASM service runs contract algebra
        |
        v
diagnostics + evidence link back to KIR element IDs
```

## Open Questions

- Should plugin lock files be separate from `mercurio-project.json` or embedded in the resolved project descriptor?
- Should service schemas be JSON Schema, a compact custom schema, or generated from Rust DTOs?
- Should WASM modules use WASI component model interfaces from the start or a simpler JSON function ABI first?
- How should plugin state migration work for project-specific extension caches?
- What is the minimum signing model for local development, internal company registries, and public registries?

