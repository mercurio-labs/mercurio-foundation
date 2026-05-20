# Verification Pipeline Architecture

Status: active architecture plan.

## Purpose

This document describes how Mercurio should model CI/CD commit actions, requirement compliance checks, code quality checks, unit tests, and behavioral simulation runs.

The short version is:

```text
commit / PR / release candidate
        |
        v
  compile source authority
        |
        v
  semantic artifact
        |
        v
  verification plan
        |
        v
  verification actions
        |
        v
  results + evidence + provider status checks
```

Verification actions are consumers of compiled semantic artifacts. They should not become a second semantic compiler, a source authority, or a parser-specific path.

## Architecture Position

Mercurio already separates source authority, KIR compilation, runtime semantics, semantic services, and user surfaces:

```text
Source authority -> KIR compiler/runtime -> semantic services -> user surfaces
```

Verification pipelines fit into the semantic-service layer. They run against commits, pull requests, proposals, release candidates, or local workspace snapshots, but the source of truth remains external source files and Git history.

```text
.sysml / .kerml / code / tests
          |
          v
   frontend compile
          |
          v
   consolidated KIR
          |
          v
   graph + runtime + derived indexes
          |
          v
   verification actions
          |
          v
   evidence, diagnostics, status checks
```

## Core Concepts

### Verification Plan

A `VerificationPlan` is a named set of checks to run for a trigger.

Typical triggers:

- `commit`
- `pull_request`
- `proposal`
- `release_candidate`
- `scheduled`
- `manual`

Plans should be versioned inputs to the verification result. A change to the plan can change pass/fail behavior, so plan identity belongs in evidence metadata.

### Verification Action

A `VerificationAction` is one executable check inside a plan.

Common action types:

- `semantic_compile`
- `model_lint`
- `requirement_compliance`
- `traceability_coverage`
- `constraint_solve`
- `behavioral_simulation`
- `unit_test`
- `code_quality`
- `external_command`

Each action should declare:

- stable id
- display name
- action type
- target artifact or source scope
- inputs
- expected result or assertion set
- timeout and resource limits
- gate policy
- evidence outputs

### Verification Run

A `VerificationRun` records one execution of a plan against a specific input.

Required provenance:

- repository URL or local workspace id
- branch, tag, commit, pull request, or proposal id
- semantic artifact id
- compiler/runtime version
- stdlib and package dependency digests
- mapping and rule-pack digests
- verification plan digest
- action runner versions

### Verification Evidence

Evidence is Mercurio-owned derived state. It should be reproducible and source-linked.

Evidence examples:

- compile diagnostics
- lint findings
- requirement coverage tables
- constraint solver records
- unit test output
- simulation traces
- failed assertion explanations
- semantic diff impact summaries
- links to provider-native status checks

Evidence should keep KIR element ids and source provenance where possible so failures can be traced back to authored model source.

## Action Layers

Verification actions fall into three broad layers.

### Static Checks

Static checks are fast, deterministic checks that do not execute model behavior.

Examples:

- parser and compile diagnostics
- KIR validation
- model lint
- unresolved reference checks
- code formatting and static analysis
- ordinary unit tests

### Semantic Checks

Semantic checks use the graph, runtime, and derived indexes.

Examples:

- every requirement has satisfy or verify evidence
- every safety requirement links to a verification case
- semantic diff impact for changed elements
- constraint evaluation
- requirement table generation
- traceability completeness

### Dynamic Checks

Dynamic checks execute or replay model behavior.

Examples:

- behavioral simulation
- scenario replay
- expected trace comparison
- temporal or state assertions
- generated executable tests

Dynamic checks still consume KIR and runtime semantics. They should not reparse source text or infer behavior from raw syntax.

## Behavioral Simulation

Behavioral simulation should be one verification action type. It belongs downstream of KIR and should use the runtime graph and expression evaluator.
The standalone runtime architecture is described in [SIMULATION_ARCHITECTURE.md](SIMULATION_ARCHITECTURE.md).

```text
KIR semantic artifact
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
trace + assertions + evidence
```

### Simulation Components

`SimulationModelExtractor`

Reads compiled KIR and graph state. Extracts executable behavior shapes such as states, transitions, triggers, guards, effects, parameters, initial nodes, ownership, and source provenance.

`SimulationScenario`

Provides run-specific inputs such as initial values, event queue, time horizon, step limit, parameter overrides, expected outputs, and random seed when stochastic behavior is explicitly supported.

`SimulationState`

Stores mutable run state: active states, variable values, pending events, emitted events, current time, current step, completed actions, and diagnostics.

`ExpressionEvaluator`

Evaluates KIR `expression_ir` against the current simulation context. Simulation should depend on expression evaluation for guards, derived values, trigger conditions, parameter bindings, constraints, and assertions.

`SimulationEngine`

Selects enabled transitions or actions, evaluates guards, applies effects, advances event or time steps, and records trace entries.

`TraceRecorder`

Captures reproducible evidence: active state before and after, consumed event, selected transition, guard result, effects, diagnostics, timestamps or step ids, source spans, and KIR element ids.

### Expression Evaluation Boundary

Expression evaluation should be factored as a pure service.

It may read:

- model properties
- graph relationships
- execution context values
- simulation variables
- active state and event context when explicitly modeled

It should not:

- mutate simulation state
- emit events
- perform transition selection
- apply action side effects
- write evidence directly

The simulation engine owns state changes and trace recording. Expression evaluation returns values used by the engine.

## First Simulation Test Case

A good first behavioral simulation verification case is a small event-driven state machine:

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
step 2: consume VehicleOnSignal, guard batteryVoltage >= 12 is true, transition starting -> on
step 3: consume VehicleOffSignal, transition on -> off
```

Acceptance assertions:

- active state sequence is `off`, `starting`, `on`, `off`
- `StartSignal` is emitted once
- `batteryVoltage >= 12` is evaluated through `expression_ir`
- every trace entry includes source or KIR provenance
- failed guards produce explainable diagnostics

Avoid continuous dynamics, parallel regions, full action bodies, and stochastic scheduling in the first slice. Those can be added after the core KIR to simulation trace loop is deterministic.

## Requirement Compliance

Requirement compliance actions should use semantic graph relationships and derived indexes.

Examples:

- all authored requirements have verification evidence
- safety requirements have at least one verification case
- changed requirements have updated or still-valid evidence
- simulation actions produce evidence linked to requirement ids
- failed assertions mark linked requirements as failed or unverified

Simulation output can satisfy verification evidence when the scenario, assertions, trace, and semantic artifact are all captured.

Example evidence relationship:

```text
Requirement R-START-001
  verified_by: vehicle.startup.nominal
  commit: abc123
  semantic_artifact: sha256:...
  result: passed
  trace: traces/vehicle.startup.nominal.json
```

## CI Provider Integration

Mercurio should publish concise provider-native checks while keeping detailed evidence in Mercurio.

Provider checks:

```text
mercurio/semantic-compile: pass
mercurio/requirements: fail
mercurio/simulation/vehicle-startup: pass
mercurio/code-quality: pass
```

Mercurio evidence:

- full diagnostics
- semantic impact
- failed requirement ids
- source-linked simulation traces
- assertion results
- explanations and dependency digests

GitHub, Gitea, GitLab, Jenkins, or another backing system may own the native CI workflow and status display. Mercurio owns semantic evidence, traceability, and provider-neutral bindings.

## Example Plan Shape

```json
{
  "id": "vehicle.default",
  "triggers": ["pull_request", "commit"],
  "actions": [
    {
      "id": "model.compile",
      "type": "semantic_compile",
      "gate": "required"
    },
    {
      "id": "requirements.coverage",
      "type": "requirement_compliance",
      "gate": "required",
      "policy": "safety_requirements_must_have_verify"
    },
    {
      "id": "vehicle.startup.nominal",
      "type": "behavioral_simulation",
      "gate": "required",
      "target": "Vehicle::startupBehavior",
      "scenario": "verification/scenarios/startup_nominal.json",
      "assertions": "verification/assertions/startup_nominal.json"
    }
  ]
}
```

This shape is illustrative, not a committed schema. A future implementation should define a versioned plan schema and validation rules.

## Artifact Keys

Verification result keys should follow [Semantic Artifact Keys](SEMANTIC_ARTIFACT_KEYS.md).

Verification appends action-specific inputs such as verification plan digest, simulator version, scenario and assertion digests, and external runner versions. This keeps CI results reproducible and makes stale evidence detectable after semantic, runtime, simulator, or policy changes.

## Non-Goals For The First Slice

- full SysML action-language execution
- arbitrary user code execution inside model simulation
- continuous time integration
- stochastic scheduling without explicit seed capture
- parallel state regions
- replacing ordinary CI systems
- replacing KIR or graph runtime semantics
- storing verification evidence as accepted source

## Guiding Rule

Compile once, verify many times:

```text
Source authority owns source.
KIR owns compiled semantics.
Runtime owns graph, derived indexes, and expression evaluation.
Verification actions consume semantic artifacts.
Simulation produces derived evidence.
CI providers display status; Mercurio preserves semantic evidence.
```
