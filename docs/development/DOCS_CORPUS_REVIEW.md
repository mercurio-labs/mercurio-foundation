# Developer Docs Corpus Review

Status: corpus organization review.

## Purpose

This review organizes the developer-docs corpus, calls out overlapping documents, and records logical gaps or unmet expectations that should be resolved before the docs are treated as a stable engineering contract.

## Current Organization

The corpus now has four useful layers:

1. Canonical architecture: `ARCHITECTURE_MODEL.md`, `KIR_SPEC.md`, `FRONTEND_TO_KIR_THEORY_OF_OPERATION.md`, and `PROJECT_DESCRIPTOR_AND_MOUNT_PLAN.md`.
2. Runtime and semantic services: Datalog, views, simulation, and verification.
3. Active implementation plans: KerML support, expression support, server, Python, installer, and diagrams.
4. Historical or benchmark notes: the original L2 parser plan, compile benchmark snapshot, and the older proposal API plan.

This is a better split than treating every file as an equal active plan. Some documents are contracts, some are direction records, and some are historical implementation notes.

## Keep Separate

- Keep [Server Workspace Plan](SERVER_WORKSPACE_PLAN.md) and [Server Implementation Plan](SERVER_IMPLEMENTATION_PLAN.md) separate. The first defines product/authority strategy; the second records implementation sequence and the product-repository boundary.
- Keep [KIR Spec](KIR_SPEC.md) separate from [Frontend To KIR Theory Of Operation](FRONTEND_TO_KIR_THEORY_OF_OPERATION.md). One is the interchange contract; the other is the compiler path into that contract.
- Keep [Simulation Architecture](SIMULATION_ARCHITECTURE.md) and [Verification Pipeline Architecture](VERIFICATION_PIPELINE_ARCHITECTURE.md) separate, but preserve cross-links. Simulation is an executable runtime service; verification is the evidence and gating layer that can consume simulation.
- Keep [Project Descriptor and Library Provider Plan](PROJECT_DESCRIPTOR_AND_MOUNT_PLAN.md) separate because it owns source/library/cache boundaries that affect CLI, desktop, server, and package workflows.

## Combine Or Reconcile

- Reconcile [Diagram Implementation Plan](DIAGRAM_IMPLEMENTATION_PLAN.md) with [Views Architecture](VIEWS_ARCHITECTURE.md). Diagrams should become a specialized view kind instead of a parallel concept. The diagram plan can stay as a delivery plan, but its API and file format should align with the view context model.
- Reconcile [Datalog Reasoning Engine Plan](DATALOG_REASONING_ENGINE_PLAN.md) with current implementation status. `mercurio-core` now exposes `datalog`, `DerivedIndexes`, rule-pack loading, and graph fact extraction, so the document should move from pure plan language to architecture-plus-remaining-work language.
- Reconcile [SysML Expression Implementation Plan](SYSML_EXPRESSION_IMPLEMENTATION_PLAN.md) with current code. `expression_ir` parsing, emission, runtime evaluation, and constraint rendering appear to exist, so the plan should become either a completed slice note or a backlog for remaining expression coverage.
- Treat [L2 Parser Plan](L2_PARSER_PLAN.md) as historical baseline. The active source-compiler explanation is now [Frontend To KIR Theory Of Operation](FRONTEND_TO_KIR_THEORY_OF_OPERATION.md), and KerML evolution belongs in [KerML Support Plan](KERML_SUPPORT_PLAN.md).
- The old proposal API plan is obsolete. The maintained proposal model is [Proposal And Draft Overlay Lifecycle](PROPOSAL_DRAFT_LIFECYCLE.md), backed by the source-authority model in [Architecture Model](ARCHITECTURE_MODEL.md).

## Logical Gaps

- No explicit status header across all development docs. Each doc should say one of: `Canonical`, `Active Plan`, `Partially Implemented`, `Historical`, or `Superseded`.
- Cross-plan delivery order is now centralized in [Development Roadmap](DEVELOPMENT_ROADMAP.md). Remaining cleanup is to keep local phase lists aligned with that roadmap.
- KIR still lacks a formal schema and versioning contract. [KIR Schema Roadmap](KIR_SCHEMA_ROADMAP.md) now tracks this work, and downstream docs should link there when they need stable `expression_ir`, source provenance, artifact keys, or reference semantics.
- Semantic artifact key vocabulary is now centralized in [Semantic Artifact Keys](SEMANTIC_ARTIFACT_KEYS.md). Remaining cleanup is to keep new docs linking there instead of redefining key fields.
- Proposal and draft-overlay concepts are now centralized in [Proposal And Draft Overlay Lifecycle](PROPOSAL_DRAFT_LIFECYCLE.md). Remaining cleanup is to keep product/API docs aligned with that lifecycle.
- Source authority terminology is strong in the architecture docs but not consistently applied to Python, installer, and older API plans. Anything that opens, edits, stages, saves, or indexes files should state whether it acts on local files, external Git, a proposal overlay, or derived artifacts.
- Product-repository boundaries are still uneven. Server and diagram docs mention routes or UI files that are now outside `mercurio-core`; docs should consistently distinguish reusable core DTO/services from product HTTP/UI implementation.
- User-facing docs are thin compared with CLI capabilities. The root README carries most user documentation; `docs/user/` currently only adds the KIR guide.

## Unmet Expectations

- A new contributor should start with [Development Roadmap](DEVELOPMENT_ROADMAP.md); remaining work is to keep individual plan status sections current.
- A maintainer cannot quickly tell whether a plan has been implemented, partially implemented, or superseded without checking code.
- A product engineer may infer that `mercurio-core` should contain HTTP routes from older docs, despite the current boundary that privileged HTTP/console API code lives in the product repository.
- A user looking under `docs/user/` will not find installation, project descriptor, KPAR, linting, package, or troubleshooting guides even though the README describes those workflows.
- Runtime-defined views, semantic edits from views, proposal overlays, and verification evidence now have a shared draft/proposal lifecycle reference. Remaining work is to align product APIs and code comments with that model.

## Recommended Cleanup Sequence

1. Add status blocks to every development doc.
2. Convert stale plans that are partly implemented into "architecture and remaining work" docs.
3. Keep [Semantic Artifact Keys](SEMANTIC_ARTIFACT_KEYS.md) authoritative and avoid reintroducing duplicate key lists.
4. Keep [Proposal And Draft Overlay Lifecycle](PROPOSAL_DRAFT_LIFECYCLE.md) authoritative for drafts, proposals, overlays, and PR bindings.
5. Fold diagram terminology into the view architecture, leaving the diagram plan as a concrete implementation slice.
6. Split user docs out of the root README into focused guides once the developer corpus has stable names.
