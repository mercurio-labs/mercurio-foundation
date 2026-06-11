# Simulation Implementation Plan

Status: proposed implementation plan.

## Goal

Implement the first Mercurio-native simulation slice as an event-based dynamic analysis service over compiled KIR.

The first implementation should prove this loop:

```text
source language or generated model data
        |
        v
      KIR
        |
        v
 graph + runtime
        |
        v
 event-based simulation
        |
        v
 deterministic trace + diagnostics + evidence
```

This plan is the implementation companion to [SIMULATION_ARCHITECTURE.md](SIMULATION_ARCHITECTURE.md).

## Foundation Boundary

Simulation can live in foundation only while it remains a KIR-native runtime service. The engine should consume executable facts projected from KIR and produce deterministic traces, diagnostics, and evidence. It should not parse SysML, depend on a SysML metamodel bundle, or require SysML-specific element names.

SysML and KerML support belongs at the projection boundary: language repositories lower source behavior into canonical KIR behavior properties or adapter-normalized simulation facts. Foundation may recognize broadly modeled state/transition shapes when they are already present in KIR, but source-language-specific lowering rules, library semantics, and syntax recovery stay outside foundation.

Product workflows may expose "SysML simulation" or "hybrid simulation" UX, but those labels should wrap the foundation service instead of shaping its core API.

## Scope

### In Scope

- Native event-step simulation engine.
- Simulation module in `mercurio-runtime`.
- Hand-authored KIR fixture for the first state-machine slice.
- Optional source-to-KIR integration only after KIR shape is clear.
- Guard evaluation through runtime `expression_ir`.
- Simple transition effects:
  - emit event
  - assign scalar value
  - enter target state
- Deterministic trace output.
- Scenario and assertion data structures.
- Unit tests for engine behavior and trace output.

### Out Of Scope

- Continuous time integration.
- Parallel state regions.
- Nested state entry/exit semantics beyond a single active state.
- Full SysML action language.
- Arbitrary user code execution.
- FMI/FMU execution.
- UI trace visualization.
- CI provider publishing.

## Module Layout

Add or evolve a dedicated module:

```text
crates/mercurio-runtime/src/simulation/
  mod.rs
  model.rs
  scenario.rs
  state.rs
  engine.rs
  trace.rs
  assertions.rs
```

Expose it from `crates/mercurio-runtime/src/lib.rs`:

```rust
pub mod simulation;

pub use simulation::{
    SimulationDiagnosticDto, SimulationEngine, SimulationEventDto, SimulationResultDto,
    SimulationScenarioDto, SimulationTraceDto, SimulationTraceEntryDto,
};
```

Keep simulation out of `frontend`, `views`, `constraints`, and `datalog`. Those modules may feed simulation or consume its output, but the stepper needs its own state and trace model.

## Data Model

### Simulation Model

`SimulationModel` is the executable view extracted from graph/KIR.

Initial fields:

```rust
pub struct SimulationModel {
    pub target_id: String,
    pub states: BTreeMap<String, SimulationStateNode>,
    pub transitions: Vec<SimulationTransition>,
    pub initial_state: Option<String>,
}
```

`SimulationStateNode`:

```rust
pub struct SimulationStateNode {
    pub id: String,
    pub name: Option<String>,
    pub source: Option<SimulationSourceRef>,
}
```

`SimulationTransition`:

```rust
pub struct SimulationTransition {
    pub id: String,
    pub source: String,
    pub target: String,
    pub trigger: Option<String>,
    pub guard_expression_ir: Option<serde_json::Value>,
    pub effects: Vec<SimulationEffect>,
    pub source_ref: Option<SimulationSourceRef>,
}
```

`SimulationEffect`:

```rust
pub enum SimulationEffect {
    EmitEvent { event: String },
    AssignValue { owner: String, feature: String, value: serde_json::Value },
}
```

### Scenario

`SimulationScenarioDto` is the public input shape.

Initial fields:

```rust
pub struct SimulationScenarioDto {
    pub target_id: String,
    pub initial_state: Option<String>,
    pub events: Vec<SimulationEventDto>,
    pub values: BTreeMap<String, BTreeMap<String, serde_json::Value>>,
    pub max_steps: Option<u64>,
    pub assertions: Vec<SimulationAssertionDto>,
}
```

Use nested values for user-facing input, matching the existing constraint/evaluation ergonomics:

```json
{
  "values": {
    "vehicle1": {
      "batteryVoltage": 12.4
    }
  }
}
```

### Runtime State

`SimulationRunState` is internal mutable state.

```rust
pub struct SimulationRunState {
    pub active_state: String,
    pub values: BTreeMap<(String, String), serde_json::Value>,
    pub pending_events: VecDeque<SimulationEventDto>,
    pub emitted_events: Vec<SimulationEventDto>,
    pub step: u64,
}
```

Start with one active state. Expand to a set of active states only when parallel regions are implemented.

### Trace

Trace output should be deterministic and source-linked.

```rust
pub struct SimulationTraceEntryDto {
    pub step: u64,
    pub active_before: String,
    pub consumed_event: Option<SimulationEventDto>,
    pub transition_id: Option<String>,
    pub guard_result: Option<bool>,
    pub effects: Vec<SimulationEffectDto>,
    pub active_after: String,
    pub diagnostics: Vec<SimulationDiagnosticDto>,
}
```

`SimulationResultDto`:

```rust
pub struct SimulationResultDto {
    pub status: SimulationStatusDto,
    pub trace: SimulationTraceDto,
    pub diagnostics: Vec<SimulationDiagnosticDto>,
    pub assertion_results: Vec<SimulationAssertionResultDto>,
}
```

## KIR Fixture Shape

Do not wait for complete SysML behavior lowering. The first engine tests should use hand-authored KIR so the runtime semantics can be designed independently from parser coverage.

Minimum properties for state-machine fixtures:

```json
{
  "id": "behavior.Vehicle.startup",
  "kind": "Mercurio::Simulation::StateMachine",
  "properties": {
    "states": ["state.Vehicle.off", "state.Vehicle.starting", "state.Vehicle.on"],
    "transitions": ["transition.Vehicle.off_starting"]
  }
}
```

```json
{
  "id": "transition.Vehicle.off_starting",
  "kind": "Mercurio::Simulation::Transition",
  "properties": {
    "source": "state.Vehicle.off",
    "target": "state.Vehicle.starting",
    "trigger": "VehicleStartSignal",
    "effects": [
      {
        "kind": "emit_event",
        "event": "StartSignal"
      }
    ]
  }
}
```

Guard example:

```json
{
  "id": "transition.Vehicle.starting_on",
  "kind": "Mercurio::Simulation::Transition",
  "properties": {
    "source": "state.Vehicle.starting",
    "target": "state.Vehicle.on",
    "trigger": "VehicleOnSignal",
    "guard_expression_ir": {
      "kind": "binary",
      "op": "greater_equal",
      "left": {
        "kind": "path",
        "root": "self",
        "segments": ["batteryVoltage"]
      },
      "right": {
        "kind": "literal",
        "value": 12
      }
    }
  }
}
```

This `Mercurio::Simulation::*` fixture vocabulary is an internal bootstrap shape. Later source-derived behavior should lower into canonical KIR behavior properties or adapter-normalized simulation facts. Any SysML/KerML-specific interpretation belongs in language packages before the data reaches this engine.

## Expression Evaluation Work

Simulation needs access to `expression_ir` evaluation.

Short-term:

- add a narrow public method on `Runtime` for expression evaluation against an owner and `ExecutionContext`
- keep behavior identical to current private evaluation path

Possible API:

```rust
impl Runtime {
    pub fn evaluate_expression_ir_value(
        &self,
        expression: &serde_json::Value,
        owner_id: &str,
        context: &ExecutionContext,
    ) -> Result<serde_json::Value, RuntimeError> {
        self.evaluate_expression_ir(expression, owner_id, context)
    }
}
```

Long-term:

- move expression evaluation into a shared `expression.rs` module
- use it from runtime feature evaluation, constraints, requirement checks, and simulation

Do not duplicate expression parsing or graph path traversal in the simulation module.

## Implementation Phases

### Phase 1: Skeleton And DTOs

Deliverables:

- add `simulation/` module
- add public DTOs and errors
- export module from `lib.rs`
- add empty engine entrypoint

API:

```rust
pub fn simulate_scenario(
    runtime: &Runtime,
    scenario: SimulationScenarioDto,
) -> Result<SimulationResultDto, SimulationError>
```

Tests:

- DTO construction
- empty or missing target diagnostics
- max step default behavior

### Phase 2: Hand-Authored KIR Extraction

Deliverables:

- extract `Mercurio::Simulation::StateMachine`
- extract states and transitions
- validate missing source/target references
- preserve source metadata

Tests:

- extractor finds three states and three transitions
- invalid transition target returns diagnostic
- duplicate state ids are deterministic

### Phase 3: Event-Step Engine

Deliverables:

- initialize active state
- consume queued events
- find transitions matching active state and event
- apply target state
- emit trace entries

Tests:

- `off -> starting -> on -> off`
- unmatched event leaves active state unchanged and records diagnostic or no-op trace
- max step stops runaway scenarios

### Phase 4: Guard Evaluation

Deliverables:

- convert simulation values to `ExecutionContext`
- call runtime expression evaluation for transition guards
- treat non-boolean guard result as diagnostic
- support blocked transition trace entries

Tests:

- `batteryVoltage >= 12` allows transition
- `batteryVoltage < 12` blocks transition
- missing guard value records diagnostic

### Phase 5: Simple Effects

Deliverables:

- emit event effect
- assign scalar value effect
- record effects in trace
- add emitted events to run state

Tests:

- `VehicleStartSignal` emits `StartSignal`
- assignment updates later guard input
- trace contains effect source transition

### Phase 6: Assertions

Deliverables:

- final active state assertion
- emitted event count assertion
- trace contains transition assertion
- assertion result DTOs

Tests:

- passing startup scenario
- failing final state assertion
- failing missing emitted event assertion

### Phase 7: CLI/API Hook

Deliverables:

- add CLI command only after core API is stable
- accept KIR input and scenario JSON
- emit JSON result by default

Possible command:

```powershell
mercurio simulate --kir startup.kir.json --scenario startup.scenario.json --format json
```

Tests:

- CLI fixture run
- JSON output is stable enough for CI

## First Acceptance Fixture

Scenario:

```json
{
  "target_id": "behavior.Vehicle.startup",
  "initial_state": "state.Vehicle.off",
  "values": {
    "vehicle1": {
      "batteryVoltage": 12.4
    }
  },
  "events": [
    { "name": "VehicleStartSignal" },
    { "name": "VehicleOnSignal" },
    { "name": "VehicleOffSignal" }
  ],
  "assertions": [
    { "kind": "final_state", "state": "state.Vehicle.off" },
    { "kind": "emitted_event_count", "event": "StartSignal", "count": 1 }
  ]
}
```

Expected active state sequence:

```text
off
starting
on
off
```

Expected evidence:

- `StartSignal` emitted once
- guard expression evaluated as true
- each transition trace entry includes transition id

## Error And Diagnostic Policy

Use diagnostics for model or scenario issues that can be reported with partial output.

Examples:

- missing target behavior
- missing initial state
- transition target not found
- guard did not evaluate to boolean
- unmatched event
- step limit reached
- unsupported effect kind

Use hard errors for invalid API inputs or internal failures where no meaningful result can be produced.

## Integration With Verification

After the core simulation API is stable, verification actions can wrap it.

```text
VerificationAction(type = behavioral_simulation)
  -> load semantic artifact
  -> load scenario
  -> simulate_scenario
  -> convert trace/assertions to evidence
  -> publish pass/fail status
```

Do not put verification-specific fields into the core simulation engine. Keep requirement ids and CI provider details in the verification layer unless the assertion explicitly references model requirements.

## Future FMI/FMU Adapter

FMI integration should be a later adapter under the broader dynamic analysis layer.

The native engine should define reusable scenario, assertion, and evidence concepts first. Later:

```text
DynamicAnalysisAdapter
  -> NativeEventSimulationAdapter
  -> FmiCoSimulationAdapter
  -> FmiModelExchangeAdapter
  -> FmiScheduledExecutionAdapter
```

FMUs should execute math-heavy dynamic behavior. Mercurio should preserve semantic mappings, scenarios, assertions, and evidence.

## Definition Of Done

The first simulation implementation is complete when:

- `mercurio-foundation` exposes `simulate_scenario`
- a hand-authored KIR state-machine fixture runs deterministically
- guard evaluation uses runtime `expression_ir`
- simple emitted-event effects appear in trace output
- final-state and emitted-event assertions pass/fail deterministically
- diagnostics are source/KIR linked where possible
- tests cover pass, guard-blocked, missing-target, and assertion-failure cases
- the implementation remains downstream of KIR and runtime graph semantics
