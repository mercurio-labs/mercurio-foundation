# Proposal And Draft Overlay Lifecycle

Status: canonical lifecycle reference.

## Purpose

This page defines how Mercurio should model browser proposals, local draft changes, semantic edits from views, and PR submission without making Mercurio the default source authority.

## Core Rule

Accepted source changes flow through the source authority.

For normal team projects, that authority is external Git. Mercurio can create overlays, validate them, export patches, create branches or pull requests, and record semantic impact, but provider merges and Git commits remain the acceptance mechanism.

## Concepts

- `Base`: immutable source snapshot, usually a Git commit.
- `Overlay`: file or semantic changes applied on top of a base without mutating accepted source.
- `Draft Change Set`: local or interactive overlay that may not yet be reviewable.
- `Proposal`: Mercurio-owned reviewable overlay with rationale, validation, semantic diff, and submission state.
- `PR Binding`: link from a Mercurio proposal to an external provider pull request.
- `Semantic Artifact`: compiled result of `base + overlay` and its semantic environment.

## Lifecycle

```text
base source snapshot
  -> draft change set
  -> virtual compile
  -> diagnostics and semantic diff
  -> proposal
  -> export patch or create provider branch/PR
  -> provider merge
  -> indexed accepted commit
```

Not every draft must become a proposal. Not every proposal must become a PR.

## Draft Change Sets

A draft change set is useful for:

- semantic edits from tables, diagrams, or forms
- AI-proposed changes
- unsaved browser edits
- local what-if analysis
- before/after view rendering

Drafts may contain:

- file overlays
- semantic operations, such as rename, move, retype, add relationship, or edit requirement text
- generated source patches
- validation results
- source patch previews

Drafts should be short-lived unless explicitly saved. They should always identify their base source snapshot and semantic environment.

## Proposals

A proposal is a durable Mercurio object over an immutable base.

It should store:

- project or repository id
- base commit or stable snapshot id
- file overlays or generated patch
- author and rationale
- validation result
- semantic diff
- review comments
- provider bindings, if any
- status

Recommended statuses:

- `Draft`
- `Validated`
- `Needs Fixes`
- `Ready for PR`
- `Submitted`
- `Superseded`
- `Abandoned`

## Validation

Proposal validation compiles:

```text
base source snapshot + overlay -> KIR -> graph -> derived indexes -> diagnostics and semantic diff
```

The resulting semantic artifact key must include the base source snapshot, overlay digest, compiler/runtime inputs, dependencies, mappings, rule packs, and validation policy. See [Semantic Artifact Keys](SEMANTIC_ARTIFACT_KEYS.md).

## View And Diagram Edits

Views may support presentation edits and semantic edits.

Presentation edits update saved view state only.

Semantic edits must become draft operations or proposal overlay changes. A table, diagram, matrix, or form should not directly patch accepted source. The expected path is:

```text
view edit
  -> semantic operation
  -> draft overlay
  -> validation
  -> source patch preview
  -> apply locally, save proposal, export patch, or submit PR
```

## PR Binding

A PR binding connects a Mercurio proposal to a provider-native pull request.

The Git provider owns:

- branch existence
- commits
- reviews
- protection rules
- merge decision
- merge audit trail

Mercurio owns:

- semantic impact
- validation evidence
- proposal rationale
- provider-neutral discussion and decision links
- semantic status comments or checks

If the target branch moves after validation, the proposal must be rebased or revalidated before submission or merge guidance.

## Outcomes

A proposal can:

- remain a draft
- be validated
- export a patch
- create an external branch and PR
- bind to an existing external PR
- be superseded by another proposal
- be abandoned

Successful provider merge creates a new accepted commit. Mercurio then indexes that commit as ordinary source authority state; it does not treat the proposal overlay itself as accepted source.

## Non-Goals

This lifecycle does not require browser-first full authoring, server-owned Git hosting, or direct mutation of indexed mirrors.

Server-authoritative projects may exist later as a separate source provider, but that mode should not drive the default proposal model.
