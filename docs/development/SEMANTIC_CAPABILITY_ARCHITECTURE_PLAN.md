# Semantic Capability Architecture Plan

## Purpose

Mercurio should treat validation, impact analysis, requirement analysis, simulation, constraint solving, contract analysis, mutation preview, and decision assessment as additive semantic capabilities.

The goal is to clean up scattered feature-specific endpoints and demo paths by giving every evidence-producing service the same architectural shape:

- declare what it can analyze,
- declare what model concepts it operates on,
- report readiness before running,
- run against a semantic workspace snapshot,
- emit normalized partial insights,
- attach source-linked evidence and artifacts,
- compose with other capabilities in decision analysis and centaur workflows.

Simulation is the motivating example. It should not be a freestanding demo feature. It should be a dynamic analysis capability that produces decision-relevant behavioral evidence.

## Architectural Principle

Capabilities do not answer broad design questions by themselves. They produce scoped evidence.

Examples:

- Requirement analysis can report that a requirement has no verification trace.
- Simulation can report that a scenario reaches a hazardous state.
- Constraint analysis can report that a power budget is infeasible.
- Impact analysis can report that a proposed change affects many downstream usages.
- Mutation preview can report that an edit is feasible but introduces a validation regression.

The decision layer combines these partial insights into engineering recommendations, gaps, risks, and next actions.

## Layering

```text
AI / Centaur Workflow
Decision Analysis
Capability Registry and Orchestration
Capability Implementations
  - generic foundation capabilities
  - SysML profile capabilities
  - plugin-backed capabilities
Semantic Workspace
KIR / Graph / Runtime / Source Mapping / Sessions
Source Language Frontends
```

MCP, LSP, CLI, and product UI adapters should call the capability registry. They should not invent domain semantics.

## Foundation Ownership

Foundation should own the generic capability contracts and language-neutral substrate:

- `SemanticCapability`
- `CapabilityRegistry`
- `CapabilityDescriptor`
- `CapabilityReadinessReport`
- `CapabilityRunRequest`
- `CapabilityRunReport`
- `SemanticInsight`
- `SemanticArtifact`
- `SemanticDiagnostic`
- `EvidenceGraph`
- `DecisionContext`
- `DecisionAssessment`
- workspace snapshots, revisions, graph indexes, source spans, sessions, forks, and semantic diffs

Foundation can also provide baseline capabilities that operate on any KIR graph:

- generic graph impact,
- semantic diff and compare,
- mutation preview envelope,
- runtime/datalog assessment,
- generic simulation execution contracts,
- generic search/query substrate.

## SysML Ownership

SysML should own capabilities whose meaning depends on SysML concepts:

- requirement analysis,
- SysML traceability,
- SysML validation,
- SysML-enriched impact analysis,
- SysML mutation support,
- SysML simulation readiness and behavior projection,
- SysML constraint solving, graph rendering, and projection,
- SysML contract/interface mapping,
- allocation analysis,
- port/interface analysis,
- variant/configuration analysis.

SysML capabilities should be declarative about the element kinds, relationship kinds, scopes, and mutations they understand.

## Plugin Ownership

Plugins are implementation vehicles, not the architecture boundary.

A plugin-backed capability should still register through the same capability descriptor and return the same run report shape. For example, contract analysis may be implemented by a native Rust engine, a WASM plugin, or a Python backend, but callers should still see it as `ContractAnalysisCapability`.

## Product And Interface Ownership

Product services, MCP tools, CLI commands, LSP features, and UI routes should be adapters over the capability registry.

They may choose presentation, filtering, authorization, and persistence behavior. They should not own the semantics of requirements, simulation, constraints, contracts, or impact analysis.

## Capability Contract

The core trait should be small and practical:

```rust
pub trait SemanticCapability {
    fn descriptor(&self) -> CapabilityDescriptor;

    fn readiness(
        &self,
        workspace: &SemanticWorkspaceSnapshot,
        target: &SemanticTarget,
    ) -> CapabilityReadinessReport;

    fn run(
        &self,
        workspace: &SemanticWorkspaceSnapshot,
        request: CapabilityRunRequest,
    ) -> Result<CapabilityRunReport, CapabilityError>;
}
```

The descriptor should declare enough for AI and UI orchestration:

```rust
pub struct CapabilityDescriptor {
    pub id: String,
    pub name: String,
    pub kind: CapabilityKind,
    pub profile_id: Option<String>,
    pub target_kinds: Vec<String>,
    pub relationship_kinds: Vec<String>,
    pub input_artifact_kinds: Vec<String>,
    pub produced_insight_kinds: Vec<InsightKind>,
    pub produced_artifact_kinds: Vec<String>,
    pub deterministic: bool,
    pub cost_class: CapabilityCostClass,
    pub maturity: CapabilityMaturity,
}
```

Readiness should avoid forced pass/fail:

```text
ready
partial
not_applicable
blocked
error
```

Examples:

- `ready`: target package contains SysML requirements and trace relationships.
- `partial`: requirements exist, but source spans or validation state are incomplete.
- `not_applicable`: no supported target model kinds exist in the selected scope.
- `blocked`: the workspace has compile errors that prevent semantic classification.

## Normalized Insights

Every capability should emit partial insights in the same shape:

```rust
pub struct SemanticInsight {
    pub id: String,
    pub kind: InsightKind,
    pub subject: SemanticElementRef,
    pub claim: String,
    pub polarity: InsightPolarity,
    pub severity: InsightSeverity,
    pub confidence: InsightConfidence,
    pub scope: InsightScope,
    pub evidence_ids: Vec<String>,
    pub source_spans: Vec<SourceSpanRef>,
    pub metrics: BTreeMap<String, Value>,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
}
```

Capabilities should be free to emit incomplete but useful evidence. The decision layer is responsible for combining insights, identifying contradictions, and marking missing evidence.

## Requirement Analysis Example

Requirement analysis is a SysML capability because requirements, satisfy, verify, refine, and requirement constraints are SysML/domain semantics.

Its descriptor should declare the model concepts it operates on:

```rust
CapabilityDescriptor {
    id: "sysml.requirement.analysis",
    profile_id: Some("sysml"),
    kind: CapabilityKind::RequirementAnalysis,
    target_kinds: vec![
        "SysML::Requirements::RequirementUsage",
        "SysML::Requirements::RequirementDefinition",
        "SysML::Verification::VerificationCaseUsage",
        "SysML::Dependency::SatisfyRequirementUsage",
    ],
    relationship_kinds: vec![
        "satisfy",
        "verify",
        "refine",
        "derive",
        "trace",
    ],
    produced_insight_kinds: vec![
        InsightKind::CoverageGap,
        InsightKind::VerificationGap,
        InsightKind::SatisfactionEvidence,
        InsightKind::RequirementRisk,
        InsightKind::RequirementConflict,
        InsightKind::TraceCompleteness,
    ],
    // other descriptor fields omitted
}
```

Supported scopes:

- workspace,
- package,
- requirement definition,
- requirement usage,
- requirement hierarchy,
- system element,
- verification case.

Example outputs:

- `CoverageGap`: `REQ-42` has no satisfy trace.
- `VerificationGap`: `REQ-42` has satisfy evidence but no verify relationship.
- `SatisfactionEvidence`: `BrakeController` satisfies `REQ-17`.
- `RequirementRisk`: `REQ-88` is affected by a proposed change to `PowerSubsystem`.
- `TraceCompleteness`: package `Safety` has 84 percent satisfy coverage and 52 percent verify coverage.

This same pattern applies to other SysML capabilities:

- port/interface analysis operates on ports, interfaces, item flows, and connections,
- allocation analysis operates on allocations, actions, parts, and functions,
- behavior analysis operates on states, actions, transitions, and events,
- variant analysis operates on variants and configurations,
- constraint analysis operates on constraints, parameters, values, quantities, and requirement checks.

## Capabilities To Migrate

### Requirement Coverage And Requirement Analysis

Current shape:

- `requirement_coverage_report`,
- `requirements-coverage` plugin,
- reference requirement coverage capability,
- deprecated requirement trace query helpers.

Target:

```text
SysmlRequirementAnalysisCapability
```

Produced insights:

- `CoverageGap`,
- `VerificationGap`,
- `SatisfactionEvidence`,
- `RequirementRisk`,
- `TraceCompleteness`.

Migration action:

- Move SysML-specific trace interpretation into the SysML profile.
- Keep generic evidence/report shapes in foundation.
- Deprecate the old requirement-specific query entry point once the capability exists.

### Semantic Impact

Current shape:

- `semantic_impact_report`,
- `semantic-impact` plugin,
- `SemanticImpact` proposal structs.

Target:

```text
GenericImpactCapability
SysmlImpactCapability
```

Produced insights:

- `ImpactHotspot`,
- `DependencyClosure`,
- `AffectedElement`,
- `ChangeRisk`.

Migration action:

- Keep baseline graph impact in foundation.
- Add SysML impact enrichment for definition/usage, typing, specialization, redefinition, subsetting, allocation, and requirement traces.

### Simulation And Behavior

Current shape:

- `state_machine_simulation_report`,
- `run_hybrid_simulation`,
- SysML behavior module `behavior::simulation`,
- SysML behavior module `behavior`,
- product-specific simulation DTOs.

Target:

```text
SysmlDynamicBehaviorCapability
SysmlSimulationProjectionCapability
```

Produced insights:

- `BehaviorObserved`,
- `ScenarioFailure`,
- `RequirementViolation`,
- `ReachabilityFinding`,
- `RuntimeMetric`,
- `SimulationTrace`.

Migration action:

- Register dynamic behavior as a SysML capability (`sysml.behavior.dynamic`) rather than a foundation built-in.
- Move state-machine model, execution, projection, simulation overlays, and hybrid simulation reports into the SysML behavior module.
- Keep foundation responsible only for generic runtime expression evaluation, KIR, graph, capability reports, insights, evidence, and decision vocabulary.
- Route product simulation APIs through SysML behavior while preserving product DTO compatibility.
- Normalize traces and metrics as artifacts and insights.

### Constraint Analysis

Previous shape:

- `solve_constraints`,
- `constraint_graph`,
- runtime `constraints.rs`.

Target:

```text
ConstraintAnalysisCapability
SysmlConstraintProjectionCapability
```

Produced insights:

- `ConstraintViolation`,
- `SatisfiedConstraint`,
- `UnknownVariable`,
- `FeasibleRegion`,
- `DerivedMetric`.

Migration action:

- Move the solver, graph rendering, SysML constraint interpretation, quantities, values, parameters, and requirement checks into SysML for now.
- Revisit a smaller generic arithmetic/constraint substrate only after another non-SysML profile needs it.

### Contract Analysis

Current shape:

- `contract-analysis` plugin,
- product `mercurio-reasoning-services`,
- optional Python backend,
- duplicated report-building paths.

Target:

```text
ContractAnalysisCapability
SysmlContractMappingCapability
```

Produced insights:

- `AssumptionGap`,
- `GuaranteeViolation`,
- `InterfaceRisk`,
- `ContractCompatibility`.

Migration action:

- Consolidate duplicated product/plugin logic behind one capability interface.
- Treat backend selection as an implementation detail.
- Move SysML port/interface/assumption/guarantee mapping into SysML.

### Mutation Feasibility And Preview

Current shape:

- `SemanticMutation`,
- `MutationProposal`,
- `CoreMutationFeasibilityService`,
- `ModelFork`,
- `ModelSession`,
- `SemanticDiff`.

Target:

```text
MutationPreviewCapability
SysmlMutationCapability
```

Produced insights:

- `ProposedEdit`,
- `FeasibilityIssue`,
- `RequiredChoice`,
- `ValidationDelta`,
- `SemanticDiff`.

Migration action:

- Keep sessions, forks, revisions, and diffs in foundation.
- Keep generic semantic mutation envelopes, authoring render profile hooks, and profile-aware feasibility substrate in foundation.
- Move SysML mutation vocabulary, guidance, context enrichment, relationship-source semantics, definition keyword normalization, and SysML oracle-backed feasibility into SysML.
- Return mutation previews as capability reports so AI workflows can compare edits against other evidence.

### Semantic Compare

Current shape:

- `semantic_compare.rs`.

Target:

```text
SemanticComparisonCapability
```

Produced insights:

- `ModelDelta`,
- `ConformanceGap`,
- `Regression`,
- `Improvement`.

Migration action:

- Keep generic compare in foundation.
- Use it to compare alternatives, generated output, before/after mutations, and external reference artifacts.

### Assessment And Decision Analysis

Current shape:

- `assessment.rs`,
- hardcoded `centaur.rs` report collection and candidate proposal.

Target:

```text
DecisionAssessmentCapability
CapabilityOrchestrator
```

Produced insights:

- `CriterionPass`,
- `CriterionFail`,
- `MissingEvidence`,
- `DecisionRisk`,
- `RecommendedNextAction`.

Migration action:

- Keep datalog-backed assessment generic.
- Keep parsed-source fact extraction out of foundation; language/profile crates own predicates such as SysML packages, definitions, usages, connections, interfaces, and endpoints.
- Replace hardcoded centaur report collection with capability discovery, readiness checks, and normalized insight aggregation.

## Keep As Substrate

These should not become capabilities themselves:

- KIR,
- graph,
- runtime,
- datalog,
- derived feature registry,
- element views,
- workspace snapshots,
- workspace cache,
- language services,
- source compilation,
- library resolution,
- source spans,
- diagnostics,
- sessions and forks.

Capabilities run on this substrate.

## Excise Or Deprecate Candidates

The capability migration should create a basis for deleting or narrowing half-baked code.

Candidates:

- deprecated requirement tracing in `query.rs`, after requirement analysis exists,
- standalone syntax comparison as a product capability; keep it only as dev/test tooling unless it produces decision evidence,
- UI-specific views, outlines, diagrams, tables, and explorers as semantic capabilities; keep them as presentation services,
- product endpoints that manually call named reports instead of using a registry,
- duplicate contract-analysis report builders across product services and plugin code,
- demo-only simulation flows that do not emit normalized insights, artifacts, and evidence.

## Implementation Phases

Current milestone status:

| Milestone | Area | Status |
| --- | --- | --- |
| 1 | Foundation capability contracts | Implemented |
| 2 | Semantic workspace snapshot | Implemented |
| 3 | First real capabilities: requirements, graph impact, dynamic behavior | Implemented |
| 4 | SysML capability declarations and registration | Implemented for initial SysML capabilities |
| 5 | Decision composition | Implemented as foundation decision composition |
| 6 | Adapter cleanup: CLI probe and centaur orchestration | Partially implemented |
| 7 | Legacy path excision | In progress |
| 8 | MCP and AI/centaur capability evaluation harness | Planned |

### Phase 1: Foundation Contracts

Add the capability contract types to foundation:

- `SemanticCapability`,
- `CapabilityRegistry`,
- `CapabilityDescriptor`,
- `CapabilityReadinessReport`,
- `CapabilityRunRequest`,
- `CapabilityRunReport`,
- `SemanticInsight`,
- `InsightKind`,
- `InsightPolarity`,
- `DecisionContext`,
- `DecisionAssessment`.

Use existing `ReasoningReport`, `ReasoningFinding`, and `EvidenceGraph` as input to the design, but do not force every capability to remain reasoning-plugin shaped.

### Phase 2: Semantic Workspace Snapshot

Create or formalize a common snapshot object used by all capabilities:

```rust
SemanticWorkspaceSnapshot {
    revision,
    kir,
    graph,
    metamodel_registry,
    source_index,
    profile_id,
}
```

This snapshot should be the shared input for CLI, MCP, LSP, UI, and AI workflows.

### Phase 3: Migrate Three Real Capabilities

Start with:

- requirement analysis,
- semantic impact,
- dynamic behavior simulation.

These cover static domain analysis, graph impact, and dynamic execution. They will keep the abstraction practical.

### Phase 4: Add SysML Profile Declarations

SysML should declare:

- supported concept kinds,
- supported relationship kinds,
- supported target scopes,
- supported mutations,
- supported capability IDs,
- readiness rules.

This makes SysML capabilities discoverable and testable.

### Phase 5: Add Decision Composition

Implement a decision layer that consumes normalized insights:

```text
decision question
target elements
alternatives
criteria
assumptions
scenarios
capability insights
evidence gaps
recommended next actions
```

This replaces hardcoded report collection with compositional assessment.

Initial implementation:

- `assess_decision_context` consumes readiness reports and capability run reports.
- It returns a reusable `DecisionAssessment` with decision status, normalized insights, missing evidence, and recommended next actions.
- `centaur assess` now calls this foundation composer instead of owning the evidence-gap and next-action rules locally.

### Phase 6: Adapter Cleanup

Update product, CLI, MCP, and LSP surfaces to call:

```text
capability_registry.list()
capability_registry.readiness(...)
capability_registry.run(...)
```

Adapters should only shape requests and present results.

First add a low-level capability probe CLI for deterministic integration testing:

```text
capability_probe --model model.kir.json list
capability_probe --model model.kir.json readiness --capability sysml.requirement.analysis
capability_probe --model model.kir.json run --capability sysml.requirement.analysis
```

Then replace the existing hardcoded centaur tool with a capability-driven orchestrator:

- discover registered capabilities,
- check readiness for the decision context,
- run ready and partial capabilities,
- aggregate normalized insights,
- identify missing evidence,
- recommend next actions.

The old centaur report collection path should be deprecated once the capability-driven flow reaches parity. Keep a temporary compatibility wrapper only if product users still depend on the existing command shape.

Initial implementation:

- `capability_probe` provides deterministic list/readiness/run checks for the capability registry.
- `centaur` discovers registered capabilities, checks readiness, runs ready and partial capabilities, and delegates decision composition to foundation.

### Phase 7: Excise Legacy Paths

After equivalent capability coverage exists:

- remove or deprecate duplicate report paths,
- remove or deprecate the legacy hardcoded centaur report orchestration,
- retire demo-only simulation endpoints or wrap them as capability runs,
- move deprecated requirement query logic into SysML requirement analysis,
- narrow old comparison/query modules to substrate or test support.

### Phase 8: MCP And AI Evaluation Harness

Add an MCP/AI-facing harness that treats capabilities as testable semantic operations:

- expose capability list/readiness/run/assess operations through MCP,
- provide deterministic fixtures for requirement, behavior, impact, and constraint scenarios,
- compare AI or centaur recommendations against the normalized `DecisionAssessment`,
- record which capabilities contributed evidence, which were missing, and which recommended next actions were followed,
- use the same harness for CLI smoke tests, MCP integration tests, and product regression tests.

## Acceptance Criteria

The architecture is working when:

- every evidence-producing service has a descriptor, readiness report, and normalized run report,
- SysML capabilities declare the model kinds and relationships they understand,
- simulation emits insights and evidence that can be combined with requirement and constraint analysis,
- centaur workflows discover and run capabilities instead of hardcoding named reports,
- UI/MCP/CLI adapters do not contain domain semantics,
- half-baked features are either migrated, clearly marked as substrate, or removed.
