# Simulation Architecture

Status: active architecture plan.

## Purpose

This document describes how behavioral simulation should fit into Mercurio's compiler, runtime, semantic-service, and verification architecture.

The short version is:

```text
SysML / KerML source
        |
        v
      KIR
        |
        v
 graph + derived indexes
        |
        v
 simulation model extraction
        |
        v
 simulation engine
        |
        v
 trace, assertions, diagnostics, evidence
```

Simulation is a derived runtime service over compiled semantic artifacts. It should not replace KIR, bypass the graph runtime, or infer executable behavior directly from source syntax.

## Architecture Position

KIR remains the semantic compiler boundary.

```text
.sysml / .kerml / libraries
          |
          v
       KIR documents
          |
          v
   consolidated KIR
          |
          v
    graph/runtime/index
          |
          v
   simulation services
```

The simulator consumes compiled model semantics:

- states and state usages
- transitions
- triggers and accepted events
- guards
- effects
- action usages
- parameters
- variable and attribute values
- source provenance

Simulation output is derived evidence. Accepted source still lives in project files and Git history.

## Core Components

### Simulation Model Extractor

`SimulationModelExtractor` reads KIR and graph state and produces an executable simulation model.

Responsibilities:

- identify simulation targets
- collect states, transitions, triggers, guards, and effects
- resolve ownership and containment
- normalize references through graph relationships
- attach source spans and KIR element ids
- reject incomplete executable models with diagnostics

The extractor should use derived indexes where helpful, such as ownership closure, inherited features, and normalized relationship predicates. It should not perform state stepping.

### Simulation Scenario

`SimulationScenario` is the external input to a run.

Typical fields:

- target behavior or state machine id
- initial active state or initial transition policy
- initial variable values
- event queue
- time horizon
- maximum step count
- parameter overrides
- assertion set
- random seed, if stochastic behavior is explicitly supported later

Scenarios should be versioned and digestible so simulation evidence can be reproduced.

### Simulation State

`SimulationState` is mutable state for one run.

It should contain:

- active states
- variable values
- pending events
- emitted events
- current time
- current step
- completed actions
- diagnostics

The simulation engine owns mutation of this state.

### Expression Evaluator

Expression evaluation should be a shared runtime service used by simulation.

Simulation needs expression evaluation for:

- transition guards
- trigger conditions
- derived values
- parameter bindings
- assertions
- constraints used as pass/fail checks

The evaluator should consume KIR `expression_ir`, not source text. It should read from graph state and a simulation-aware execution context.

### Simulation Engine

`SimulationEngine` executes steps.

Responsibilities:

- determine enabled transitions or actions
- evaluate guards
- select transitions deterministically
- apply effects
- enter and exit states
- consume and emit events
- advance event or time steps
- record trace entries
- stop on completion, failed assertion, timeout, or step limit

The first implementation should prefer explicit deterministic semantics over broad language coverage.

### Trace Recorder

`TraceRecorder` captures evidence from a run.

Trace entries should include:

- step id or time
- active states before and after
- consumed event
- selected transition or action
- guard expression id and result
- effects applied
- emitted events
- assertion results
- diagnostics
- KIR element ids
- source provenance when available

Traces should be suitable for CI evidence, debugging, replay, and future UI visualization.

## Expression Evaluation Boundary

Expression evaluation should be pure from the simulator's perspective.

It may read:

- KIR element properties
- graph relationships
- runtime context values
- simulation variables
- current event data
- active-state information when explicitly modeled

It should not:

- mutate simulation state
- select transitions
- emit events
- apply effects
- write trace entries

The simulation engine owns side effects. The expression evaluator returns values used by the engine.

This lets expression evaluation serve multiple runtime services:

```text
expression_ir
   -> ordinary feature evaluation
   -> constraint solving
   -> requirement checks
   -> simulation guards
   -> simulation assertions
```

## Runtime Integration

The current runtime shape already points in the right direction:

```rust
pub struct Runtime {
    graph: Graph,
    derived: DerivedIndexes,
}

pub struct ExecutionContext {
    pub values: HashMap<(String, String), Value>,
    pub version: u64,
}
```

Simulation should extend this pattern rather than create a separate runtime.

One possible shape:

```rust
pub struct SimulationContext {
    pub values: HashMap<(String, String), Value>,
    pub active_states: BTreeSet<String>,
    pub pending_events: VecDeque<SimulationEvent>,
    pub emitted_events: Vec<SimulationEvent>,
    pub time: SimulationTime,
    pub step: u64,
}
```

`SimulationContext` can wrap or convert to `ExecutionContext` when evaluating existing expressions, then add simulation-specific accessors for events and active states as the expression schema grows.

## First Milestone

The first useful simulation slice should be an event-driven state-machine run.

Model shape:

```text
initial -> off
off --VehicleStartSignal / send StartSignal to controller--> starting
starting --VehicleOnSignal [batteryVoltage >= 12]--> on
on --VehicleOffSignal--> off
```

Expected trace:

```text
step 0: active off
step 1: consume VehicleStartSignal, transition off -> starting, emit StartSignal
step 2: consume VehicleOnSignal, evaluate batteryVoltage >= 12 as true, transition starting -> on
step 3: consume VehicleOffSignal, transition on -> off
```

Acceptance criteria:

- source compiles to KIR
- KIR loads into graph and runtime
- extractor identifies states, transitions, triggers, guards, and effects
- guard evaluation uses `expression_ir`
- engine produces deterministic active-state sequence
- trace records KIR ids and source provenance
- failed guards and missing targets produce diagnostics

## Later Capabilities

After the first slice is stable, simulation can grow into:

- nested states
- entry, do, and exit actions
- time events
- parallel regions
- action-flow execution
- parameterized scenarios
- expected trace comparison
- temporal assertions
- requirement verification evidence
- continuous dynamics and integration

Continuous dynamics should be treated as a separate milestone. It needs explicit time semantics, numerical integration policy, error tolerances, and deterministic solver configuration.

## Verification Integration

Simulation runs can be used by verification actions.

Example:

```text
VerificationAction: vehicle.startup.nominal
  type: behavioral_simulation
  target: Vehicle::startupBehavior
  scenario: verification/scenarios/startup_nominal.json
  assertions: verification/assertions/startup_nominal.json
```

The result can become requirement evidence when the run captures:

- semantic artifact id
- simulator version
- scenario digest
- assertion digest
- trace output
- pass/fail status
- linked requirement ids

This integration is described in [VERIFICATION_PIPELINE_ARCHITECTURE.md](VERIFICATION_PIPELINE_ARCHITECTURE.md).

## Non-Goals For The First Slice

- full SysML action-language execution
- arbitrary user code execution
- continuous time integration
- stochastic behavior without explicit seed capture
- parallel state regions
- complete SysML behavioral conformance
- source rewriting
- replacing KIR or graph runtime

## Guiding Rule

Keep simulation downstream of compiled semantics:

```text
Source defines behavior.
KIR carries behavior semantics.
Runtime resolves graph and expressions.
Simulation steps executable state.
Trace output becomes derived evidence.
```
