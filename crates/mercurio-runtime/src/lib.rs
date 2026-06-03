pub mod behavior;
pub mod constraints;
pub mod datalog;
pub mod runtime;
pub mod simulation;

pub use behavior::{
    StateMachineExecutionReport, StateMachineExecutionStatus, StateMachineModel,
    StateMachineScenario, StateMachineScenarioEvent, StateMachineTraceStep,
    StateMachineValidationFinding, StateMachineValidationSeverity, StateNode,
    StateTransitionTriggerKind, TransitionNode, project_state_machines,
    project_state_machines_from_graph,
};
pub use constraints::{
    ConstraintDiagnosticDto, ConstraintExplanationDto, ConstraintGraphEdgeDto,
    ConstraintGraphRequestDto, ConstraintGraphViewDto, ConstraintRecordDto,
    ConstraintSolveRequestDto, ConstraintSolveResultDto, ConstraintStatusDto,
    ConstraintVariableDto, ConstraintVariableStatusDto, RequirementCheckDto, RequirementStatusDto,
    execution_context_from_nested_values, render_constraint_graph, solve_constraints,
};
pub use datalog::{
    Atom, CORE_RULEPACK_ID, CORE_RULEPACK_VERSION, DatalogError, DerivedIndexes, Evaluation,
    Explanation, Fact, Rule, RulePack, Term, evaluate, extract_graph_facts, load_default_rulepacks,
    materialize_core_indexes,
};
pub use runtime::{
    ExecutionContext, QueryResult, Runtime, RuntimeArtifact, RuntimeError, RuntimeProfile,
    RuntimeProfileTimings,
};
pub use simulation::{
    CriticalSimulationEvent, HybridSimulationReport, HybridSimulationScenario,
    HybridSimulationStatus, HybridSimulationTraceEntry, SimulationError, SimulationSubject,
    run_hybrid_simulation,
};
