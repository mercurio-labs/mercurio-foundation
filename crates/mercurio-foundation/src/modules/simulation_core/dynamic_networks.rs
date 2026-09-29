//! Algebraic networks coupled to the deterministic simulation scheduler.
use super::*;
use constraint_network::{
    ConstraintEquation, ConstraintNetworkRequest, ConstraintNetworkResult, ConstraintNetworkStatus,
    solve_constraint_network,
};

/// One instance-scoped network. Expression identities map to runtime feature names.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationConstraintNetwork {
    pub id: String,
    pub subject_id: String,
    pub variables: BTreeMap<String, String>,
    pub unknowns: Vec<String>,
    pub equations: Vec<ConstraintEquation>,
    pub tolerance: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationNetworkEvaluation {
    pub network_id: String,
    pub subject_id: String,
    pub result: ConstraintNetworkResult,
}

fn invalid(message: impl Into<String>) -> CoreSimulationError {
    CoreSimulationError::InvalidExpression(format!("simulation.network: {}", message.into()))
}

fn request(
    network: &SimulationConstraintNetwork,
    values: &BTreeMap<(String, String), Value>,
) -> Result<ConstraintNetworkRequest, CoreSimulationError> {
    let mut knowns = BTreeMap::new();
    for (identity, feature) in &network.variables {
        if network.unknowns.contains(identity) {
            continue;
        }
        let value = values
            .get(&(network.subject_id.clone(), feature.clone()))
            .and_then(Value::as_f64)
            .filter(|v| v.is_finite())
            .ok_or_else(|| {
                invalid(format!(
                    "{} missing finite given {}.{}",
                    network.id, network.subject_id, feature
                ))
            })?;
        knowns.insert(identity.clone(), value);
    }
    Ok(ConstraintNetworkRequest {
        knowns,
        unknowns: network.unknowns.clone(),
        equations: network.equations.clone(),
        tolerance: network.tolerance,
    })
}

/// Topological order across networks; coupled algebraic loops belong in one network.
fn order(model: &SimulationModel) -> Result<Vec<usize>, CoreSimulationError> {
    let mut writers = BTreeMap::new();
    let mut ids = BTreeSet::new();
    for (index, network) in model.constraint_networks.iter().enumerate() {
        if network.id.trim().is_empty()
            || network.subject_id.trim().is_empty()
            || !ids.insert(&network.id)
        {
            return Err(invalid(
                "network/subject identities must be nonempty and network IDs unique",
            ));
        }
        if network
            .variables
            .values()
            .any(|feature| feature.trim().is_empty())
            || network.variables.values().collect::<BTreeSet<_>>().len() != network.variables.len()
        {
            return Err(invalid(format!(
                "{} has empty or aliased feature bindings",
                network.id
            )));
        }
        let mut unknowns = BTreeSet::new();
        for identity in &network.unknowns {
            let feature = network.variables.get(identity).ok_or_else(|| {
                invalid(format!(
                    "{} unknown has no runtime binding: {identity}",
                    network.id
                ))
            })?;
            if !unknowns.insert(identity)
                || writers
                    .insert((&network.subject_id, feature), index)
                    .is_some()
            {
                return Err(invalid(
                    "a solved feature must have exactly one network writer",
                ));
            }
        }
    }
    let mut pending = model
        .constraint_networks
        .iter()
        .enumerate()
        .map(|(i, n)| (n.id.clone(), i))
        .collect::<BTreeSet<_>>();
    let mut done = BTreeSet::new();
    let mut result = Vec::new();
    while !pending.is_empty() {
        let next = pending
            .iter()
            .find(|(_, index)| {
                let network = &model.constraint_networks[*index];
                network
                    .variables
                    .iter()
                    .filter(|(id, _)| !network.unknowns.contains(id))
                    .all(|(_, feature)| {
                        writers
                            .get(&(&network.subject_id, feature))
                            .is_none_or(|writer| done.contains(writer))
                    })
            })
            .cloned()
            .ok_or_else(|| {
                invalid("cyclic network dependency; put coupled equalities in a single network")
            })?;
        pending.remove(&next);
        done.insert(next.1);
        result.push(next.1);
    }
    Ok(result)
}

pub(super) fn validate(
    model: &SimulationModel,
    scenario: &ConcurrentSimulationScenario,
) -> Result<(), CoreSimulationError> {
    order(model)?;
    for network in &model.constraint_networks {
        let features = network.variables.values().collect::<BTreeSet<_>>();
        let outputs = network
            .unknowns
            .iter()
            .filter_map(|id| network.variables.get(id))
            .collect::<BTreeSet<_>>();
        // Explicit bounded composition: state is input, algebraic outputs feed guards/rates.
        // Mixed derived/binding feedback requires a unified dataflow dependency graph.
        if model.derived_rules.iter().any(|rule| {
            rule.subject_id
                .as_ref()
                .is_none_or(|id| id == &network.subject_id)
                && features.contains(&rule.feature)
        }) || model.binding_rules.iter().any(|rule| {
            [&rule.left, &rule.right].iter().any(|end| {
                end.subject_id
                    .as_ref()
                    .is_none_or(|id| id == &network.subject_id)
                    && features.contains(&end.feature)
            })
        }) {
            return Err(invalid(format!(
                "{} mixes derived/binding writers with network variables; express those equalities in the network",
                network.id
            )));
        }
        let subject = scenario
            .subjects
            .iter()
            .find(|subject| subject.subject_id == network.subject_id)
            .ok_or_else(|| invalid("network subject is not in the scenario"))?;
        let machine = model
            .machines
            .iter()
            .find(|machine| machine.id == subject.machine_id || machine.label == subject.machine_id)
            .ok_or_else(|| invalid("network subject has no state machine"))?;
        fn writes(actions: &SimulationActionSequence, outputs: &BTreeSet<&String>) -> bool {
            actions.actions.iter().any(|action| match action {
                SimulationActionNode::Effect(SimulationEffect::Assign(effect)) => {
                    outputs.contains(&effect.feature)
                }
                SimulationActionNode::Effect(SimulationEffect::AssignExpression {
                    feature,
                    ..
                }) => outputs.contains(feature),
                SimulationActionNode::Decision {
                    then_branch,
                    else_branch,
                    ..
                } => {
                    writes(then_branch, outputs)
                        || else_branch
                            .as_ref()
                            .is_some_and(|branch| writes(branch, outputs))
                }
                _ => false,
            })
        }
        let competing = machine.states.iter().any(|state| {
            state
                .entry_behavior
                .as_ref()
                .is_some_and(|a| writes(a, &outputs))
                || state
                    .exit_behavior
                    .as_ref()
                    .is_some_and(|a| writes(a, &outputs))
                || match &state.do_behavior {
                    Some(StateDoBehavior::RateIntegration { rates }) => {
                        rates.iter().any(|rate| outputs.contains(&rate.feature))
                    }
                    Some(StateDoBehavior::LookupTable { tables }) => {
                        tables.iter().any(|table| outputs.contains(&table.feature))
                    }
                    None => false,
                }
        }) || machine.transitions.iter().any(|transition| {
            transition.effects.iter().any(|effect| match effect {
                SimulationEffect::Assign(effect) => outputs.contains(&effect.feature),
                SimulationEffect::AssignExpression { feature, .. } => outputs.contains(feature),
                _ => false,
            })
        });
        if competing {
            return Err(invalid(format!(
                "{} solved outputs cannot also be assigned, integrated, or table-driven",
                network.id
            )));
        }
    }
    Ok(())
}

/// Solve into a staging map: failures cannot leak a partially solved network chain.
pub(super) fn solve(
    model: &SimulationModel,
    values: &mut BTreeMap<(String, String), Value>,
) -> Result<(), CoreSimulationError> {
    if model.constraint_networks.is_empty() {
        return Ok(());
    }
    let mut staged = values.clone();
    for index in order(model)? {
        let network = &model.constraint_networks[index];
        let result = solve_constraint_network(&request(network, &staged)?);
        if result.status != ConstraintNetworkStatus::Solved {
            return Err(invalid(format!(
                "{}: {:?}; {}",
                network.id,
                result.status,
                result
                    .diagnostics
                    .iter()
                    .map(|d| d.message.as_str())
                    .collect::<Vec<_>>()
                    .join("; ")
            )));
        }
        for identity in &network.unknowns {
            let value = result
                .values
                .get(identity)
                .ok_or_else(|| invalid("solver omitted an unknown"))?;
            let feature = network
                .variables
                .get(identity)
                .ok_or_else(|| invalid("unknown has no binding"))?;
            staged.insert(
                (network.subject_id.clone(), feature.clone()),
                Value::from(*value),
            );
        }
    }
    *values = staged;
    Ok(())
}

pub(super) fn evidence(
    model: &SimulationModel,
    values: &BTreeMap<(String, String), Value>,
) -> Result<Vec<SimulationNetworkEvaluation>, CoreSimulationError> {
    model
        .constraint_networks
        .iter()
        .map(|network| {
            let result = solve_constraint_network(&request(network, values)?);
            Ok(SimulationNetworkEvaluation {
                network_id: network.id.clone(),
                subject_id: network.subject_id.clone(),
                result,
            })
        })
        .collect()
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use crate::kir::{BinaryExpressionOp, ExpressionIr, ExpressionPathRoot, ExpressionPathSegment};
    fn path(name: &str) -> ExpressionIr {
        ExpressionIr::Path {
            root: ExpressionPathRoot::SelfRef,
            segments: vec![ExpressionPathSegment::Name(name.into())],
        }
    }
    fn number(value: f64) -> ExpressionIr {
        ExpressionIr::Literal {
            value: Value::from(value),
        }
    }
    fn binary(left: ExpressionIr, op: BinaryExpressionOp, right: ExpressionIr) -> ExpressionIr {
        ExpressionIr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
    fn state() -> SimulationState {
        SimulationState {
            id: "running".into(),
            label: "running".into(),
            parent_state_id: None,
            is_initial: true,
            is_final: false,
            is_orthogonal: false,
            is_history: false,
            entry_behavior: None,
            exit_behavior: None,
            do_behavior: Some(StateDoBehavior::RateIntegration {
                rates: vec![SimulationRate {
                    feature: "x".into(),
                    source: SimulationRateSource::Feature("y".into()),
                }],
            }),
        }
    }
    pub(crate) fn fixture() -> (
        SimulationModel,
        ConcurrentSimulationScenario,
        SimulationClockConfig,
    ) {
        let network = SimulationConstraintNetwork {
            id: "algebra".into(),
            subject_id: "plant".into(),
            variables: BTreeMap::from([("x".into(), "x".into()), ("y".into(), "y".into())]),
            unknowns: vec!["y".into()],
            equations: vec![ConstraintEquation {
                id: "feedback".into(),
                left: path("y"),
                right: binary(number(-1.0), BinaryExpressionOp::Multiply, path("x")),
            }],
            tolerance: 1e-9,
        };
        let model = SimulationModel {
            id: "coupled".into(),
            constraint_networks: vec![network],
            derived_rules: vec![],
            binding_rules: vec![],
            machines: vec![SimulationStateMachine {
                id: "machine".into(),
                label: "machine".into(),
                states: vec![state()],
                transitions: vec![],
            }],
        };
        let scenario = ConcurrentSimulationScenario {
            id: "case".into(),
            subjects: vec![ConcurrentSubjectScenario {
                subject_id: "plant".into(),
                machine_id: "machine".into(),
                initial_state_id: None,
                events: vec![],
            }],
            max_steps: 100,
            step_duration_s: 1.0,
            clock_config: None,
            initial_values: BTreeMap::from([(("plant".into(), "x".into()), Value::from(1.0))]),
            requirements: vec![],
            objectives: vec![],
            termination_policy: Default::default(),
        };
        (
            model,
            scenario,
            SimulationClockConfig {
                max_time_s: 1.0,
                fixed_step_s: 1.0,
                sample_interval_s: 1.0,
                adaptive: None,
                change_loop_limit: 20,
            },
        )
    }
    #[test]
    fn network_feedback_is_resolved_at_every_rk4_stage_and_matches_incremental_execution() {
        let (model, scenario, clock) = fixture();
        let batch =
            run_concurrent_simulation_model(&model, scenario.clone(), clock.clone()).unwrap();
        let mut session = SimulationSession::initialize(model, scenario, clock).unwrap();
        while session.snapshot().lifecycle == SimulationSessionLifecycle::Paused {
            session.advance(1).unwrap();
        }
        assert_eq!(batch, session.snapshot().trace);
        let last = batch.timeline.last().unwrap();
        // RK4 applied to x'=-x gives 1 - 1 + 1/2 - 1/6 + 1/24 = 0.375.
        assert!(
            (last.values[&("plant".into(), "x".into())].as_f64().unwrap() - 0.375).abs() < 1e-12
        );
        assert_eq!(
            last.values[&("plant".into(), "y".into())],
            Value::from(-0.375)
        );
        assert!(batch.timeline.iter().all(|frame| {
            frame.network_evaluations[0]
                .result
                .residuals
                .iter()
                .all(|residual| residual.within_tolerance)
        }));
    }
    #[test]
    fn failed_intermediate_solve_restores_clock_values_states_and_tentative_samples() {
        let (mut model, scenario, mut clock) = fixture();
        model.constraint_networks[0].equations[0] = ConstraintEquation {
            id: "singular".into(),
            left: binary(path("x"), BinaryExpressionOp::Multiply, path("y")),
            right: number(1.0),
        };
        model.machines[0].states[0].do_behavior = Some(StateDoBehavior::RateIntegration {
            rates: vec![SimulationRate {
                feature: "x".into(),
                source: SimulationRateSource::Constant(-1.0),
            }],
        });
        clock.sample_interval_s = 0.5;
        let mut session = SimulationSession::initialize(model, scenario, clock).unwrap();
        let before = session.snapshot();
        assert!(session.advance(1).is_err());
        let after = session.snapshot();
        assert_eq!(after.lifecycle, SimulationSessionLifecycle::Failed);
        assert_eq!(after.logical_time_s, before.logical_time_s);
        assert_eq!(after.trace.timeline, before.trace.timeline);
        assert_eq!(after.advance_cycles, 0);
        assert!(after.error.unwrap().contains("Inconsistent"));
    }
    #[test]
    fn solved_output_drives_guard_after_entry_assignment_in_the_same_cycle() {
        let (mut model, mut scenario, clock) = fixture();
        model.machines[0].states[0].id = "idle".into();
        model.machines[0].states[0].do_behavior = None;
        let mut running = state();
        running.is_initial = false;
        running.do_behavior = None;
        running.entry_behavior = Some(SimulationActionSequence {
            actions: vec![SimulationActionNode::Effect(SimulationEffect::Assign(
                AssignEffect {
                    feature: "x".into(),
                    value: Value::from(2.0),
                },
            ))],
        });
        let mut done = state();
        done.id = "done".into();
        done.is_initial = false;
        done.is_final = true;
        done.do_behavior = None;
        model.machines[0].states.extend([running, done]);
        model.machines[0].transitions = vec![
            SimulationTransition {
                id: "start".into(),
                source: "idle".into(),
                target: "running".into(),
                trigger: SimulationTrigger {
                    kind: SimulationTriggerKind::Event,
                    value: Some("start".into()),
                },
                guard: None,
                effects: vec![],
            },
            SimulationTransition {
                id: "guard".into(),
                source: "running".into(),
                target: "done".into(),
                trigger: SimulationTrigger {
                    kind: SimulationTriggerKind::Change,
                    value: Some("y < -1.0".into()),
                },
                guard: None,
                effects: vec![],
            },
        ];
        scenario.subjects[0].events.push(SimulationEvent {
            id: "start".into(),
            trigger: "start".into(),
        });
        let mut session = SimulationSession::initialize(model, scenario, clock).unwrap();
        let next = session.advance(1).unwrap();
        assert_eq!(next.lifecycle, SimulationSessionLifecycle::Completed);
        assert_eq!(next.logical_time_s, 0.0);
        assert_eq!(
            next.trace.timeline.last().unwrap().states["plant"],
            vec!["done"]
        );
    }
    #[test]
    fn network_writer_conflicts_and_dependency_cycles_are_rejected() {
        let (mut model, scenario, clock) = fixture();
        model.machines[0].states[0].do_behavior = Some(StateDoBehavior::RateIntegration {
            rates: vec![SimulationRate {
                feature: "y".into(),
                source: SimulationRateSource::Constant(1.0),
            }],
        });
        assert!(
            SimulationSession::initialize(model.clone(), scenario.clone(), clock.clone())
                .err()
                .unwrap()
                .to_string()
                .contains("solved outputs")
        );
        model.machines[0].states[0] = state();
        let mut duplicate = model.constraint_networks[0].clone();
        duplicate.id = "duplicate".into();
        model.constraint_networks.push(duplicate);
        assert!(
            SimulationSession::initialize(model.clone(), scenario.clone(), clock.clone())
                .err()
                .unwrap()
                .to_string()
                .contains("one network writer")
        );
        model.constraint_networks[1].unknowns = vec!["x".into()];
        assert!(
            SimulationSession::initialize(model, scenario, clock)
                .err()
                .unwrap()
                .to_string()
                .contains("cyclic network dependency")
        );
    }
}
