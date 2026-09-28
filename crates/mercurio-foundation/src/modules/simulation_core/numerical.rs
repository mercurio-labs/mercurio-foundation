//! Opt-in RK4 step doubling and bracketed change-event localization.
//! Trials are pure; only an accepted endpoint is committed by the session.
use super::*;
type Values = BTreeMap<(String, String), Value>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdaptiveIntegrationConfig {
    pub absolute_tolerance: f64,
    pub relative_tolerance: f64,
    pub minimum_step_s: f64,
    pub event_tolerance_s: f64,
    pub max_refinements: usize,
}
impl Default for AdaptiveIntegrationConfig {
    fn default() -> Self {
        Self {
            absolute_tolerance: 1e-8,
            relative_tolerance: 1e-6,
            minimum_step_s: 1e-8,
            event_tolerance_s: 1e-7,
            max_refinements: 32,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegrationEvidence {
    pub accepted_step_s: f64,
    /// Max normalized local error estimate from RK4 step doubling (difference / 15).
    pub error_ratio: f64,
    pub rejected_trials: usize,
    pub event_refinements: usize,
    pub event_bracket_s: Option<f64>,
}
fn invalid(message: &str) -> CoreSimulationError {
    CoreSimulationError::InvalidSessionOperation(format!("simulation.numerics: {message}"))
}
pub(super) fn validate(
    model: &SimulationModel,
    scenario: &ConcurrentSimulationScenario,
    clock: &SimulationClockConfig,
) -> Result<(), CoreSimulationError> {
    let Some(config) = &clock.adaptive else {
        return Ok(());
    };
    if !clock.fixed_step_s.is_finite() || clock.fixed_step_s <= 0.0 {
        return Err(invalid(
            "adaptive integration requires a positive maximum step (fixed_step_s)",
        ));
    }
    for value in [
        config.absolute_tolerance,
        config.relative_tolerance,
        config.minimum_step_s,
        config.event_tolerance_s,
    ] {
        if !value.is_finite() || value <= 0.0 {
            return Err(invalid(
                "tolerances and minimum step must be finite and positive",
            ));
        }
    }
    if config.minimum_step_s > clock.fixed_step_s
        || config.max_refinements == 0
        || config.max_refinements > 64
    {
        return Err(invalid(
            "minimum step must not exceed maximum step; max_refinements must be 1..64",
        ));
    }
    if model
        .machines
        .iter()
        .filter(|machine| {
            scenario.subjects.iter().any(|subject| {
                subject.machine_id == machine.id || subject.machine_id == machine.label
            })
        })
        .flat_map(|machine| &machine.states)
        .any(|state| matches!(state.do_behavior, Some(StateDoBehavior::LookupTable { .. })))
    {
        return Err(invalid(
            "adaptive lookup-table dynamics are not supported; use fixed-step execution",
        ));
    }
    Ok(())
}

/// Refresh algebraic/derived values at every stage even without an explicit network.
fn rk4(
    model: &SimulationModel,
    evaluator: &ExpressionEvaluator,
    subjects: &[CoreSubjectRunState],
    initial: &Values,
    dt: f64,
) -> Result<Values, CoreSimulationError> {
    let rates = active_rates(subjects);
    let stage = |increments: &BTreeMap<(String,String),f64>| -> Result<BTreeMap<(String,String),f64>,CoreSimulationError> {
        let mut values = values_with_increments(initial, increments);
        propagate_model_values(evaluator, model, subjects, &mut values)?;
        rates.iter().map(|rate| Ok((rate.key.clone(), rate_value(evaluator,rate.source,&rate.subject_id,&values)?))).collect()
    };
    let k1 = stage(&BTreeMap::new())?;
    let k2 = stage(&scaled_increments(&k1, dt / 2.0))?;
    let k3 = stage(&scaled_increments(&k2, dt / 2.0))?;
    let k4 = stage(&scaled_increments(&k3, dt))?;
    let mut values = initial.clone();
    for rate in &rates {
        let value = initial
            .get(&rate.key)
            .and_then(Value::as_f64)
            .ok_or_else(|| invalid("integrated state requires a finite scalar initial value"))?;
        let next = value
            + dt * (k1[&rate.key] + 2.0 * k2[&rate.key] + 2.0 * k3[&rate.key] + k4[&rate.key])
                / 6.0;
        if !next.is_finite() {
            return Err(invalid("non-finite adaptive stage value"));
        }
        values.insert(rate.key.clone(), Value::from(next));
    }
    propagate_model_values(evaluator, model, subjects, &mut values)?;
    Ok(values)
}
fn trial(
    model: &SimulationModel,
    evaluator: &ExpressionEvaluator,
    subjects: &[CoreSubjectRunState],
    initial: &Values,
    dt: f64,
    config: &AdaptiveIntegrationConfig,
) -> Result<(Values, f64), CoreSimulationError> {
    let full = rk4(model, evaluator, subjects, initial, dt)?;
    let half = rk4(model, evaluator, subjects, initial, dt / 2.0)?;
    let fine = rk4(model, evaluator, subjects, &half, dt / 2.0)?;
    let mut ratio: f64 = 0.0;
    for rate in active_rates(subjects) {
        let read = |values: &Values| {
            values
                .get(&rate.key)
                .and_then(Value::as_f64)
                .filter(|value| value.is_finite())
                .ok_or_else(|| invalid("non-finite error estimate"))
        };
        let (before, coarse, after) = (read(initial)?, read(&full)?, read(&fine)?);
        let scale =
            config.absolute_tolerance + config.relative_tolerance * before.abs().max(after.abs());
        if !scale.is_finite() || scale <= 0.0 {
            return Err(invalid("non-finite or zero error scale"));
        }
        let error = (after - coarse).abs() / 15.0 / scale;
        if !error.is_finite() {
            return Err(invalid("non-finite error estimate"));
        }
        ratio = ratio.max(error);
    }
    Ok((fine, ratio))
}
fn enabled(
    evaluator: &ExpressionEvaluator,
    transition: &SimulationTransition,
    subject: &str,
    values: &Values,
) -> Result<bool, CoreSimulationError> {
    if !guard_allows(evaluator, &transition.guard, subject, values)? {
        return Ok(false);
    }
    if transition.guard.is_none() {
        if let Some(text) = transition
            .trigger
            .value
            .as_deref()
            .filter(|text| !text.is_empty())
        {
            return eval_bool(evaluator, &legacy_guard_ir(text)?, subject, values);
        }
    }
    Ok(true)
}

pub(super) struct AcceptedStep {
    pub values: Values,
    pub evidence: IntegrationEvidence,
}
/// One accepted step per scheduling cycle. Rejected trials and root probes never
/// mutate values, event queues, clocks, or samples. A fresh proposal starts at the
/// configured cap each cycle, so pausing needs no hidden numerical continuation.
pub(super) fn advance(
    model: &SimulationModel,
    evaluator: &ExpressionEvaluator,
    subjects: &[CoreSubjectRunState],
    initial: &Values,
    maximum: f64,
    config: &AdaptiveIntegrationConfig,
) -> Result<AcceptedStep, CoreSimulationError> {
    let mut dt = maximum;
    let mut rejected = 0;
    loop {
        let (mut end, mut error) = trial(model, evaluator, subjects, initial, dt, config)?;
        if error <= 1.0 {
            let candidate_end = end.clone();
            let candidate_error = error;
            let mut earliest = dt;
            let mut bracket = None;
            let mut refinements = 0;
            // Each false->true bracket is localized independently, then the first
            // endpoint is selected. Transition priority stays in the scheduler.
            for subject in subjects {
                if configuration_is_final(&subject.machine, &subject.active) {
                    continue;
                }
                for transition in &subject.machine.transitions {
                    if transition.trigger.kind != SimulationTriggerKind::Change
                        || !subject.active.contains(&transition.source)
                        || enabled(evaluator, transition, &subject.subject_id, initial)?
                        || !enabled(evaluator, transition, &subject.subject_id, &candidate_end)?
                    {
                        continue;
                    }
                    let (mut low, mut high) = (0.0, dt);
                    let mut high_values = candidate_end.clone();
                    let mut high_error = candidate_error;
                    let mut converged = false;
                    for _ in 0..64 {
                        if high - low <= config.event_tolerance_s {
                            converged = true;
                            break;
                        }
                        let mid = low + (high - low) / 2.0;
                        if mid <= low || mid >= high {
                            return Err(invalid("event tolerance is below time resolution"));
                        }
                        let (values, ratio) =
                            trial(model, evaluator, subjects, initial, mid, config)?;
                        refinements += 1;
                        // Smaller probes usually have smaller errors; never silently
                        // accept one that violates the requested tolerance.
                        if ratio > 1.0 {
                            return Err(invalid(
                                "event probe exceeds integration tolerance; reduce maximum step",
                            ));
                        }
                        if enabled(evaluator, transition, &subject.subject_id, &values)? {
                            high = mid;
                            high_values = values;
                            high_error = ratio;
                        } else {
                            low = mid;
                        }
                    }
                    if !converged && high - low > config.event_tolerance_s {
                        return Err(invalid("event refinement budget exhausted"));
                    }
                    if high <= earliest {
                        earliest = high;
                        end = high_values;
                        error = high_error;
                        bracket = Some(high - low);
                    }
                }
            }
            return Ok(AcceptedStep {
                values: end,
                evidence: IntegrationEvidence {
                    accepted_step_s: earliest,
                    error_ratio: error,
                    rejected_trials: rejected,
                    event_refinements: refinements,
                    event_bracket_s: bracket,
                },
            });
        }
        if rejected >= config.max_refinements {
            return Err(invalid("adaptive refinement budget exhausted"));
        }
        let next = dt / 2.0;
        if next < config.minimum_step_s || next <= 0.0 {
            return Err(invalid(
                "integration tolerance cannot be met at the minimum step",
            ));
        }
        dt = next;
        rejected += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (
        SimulationModel,
        ConcurrentSimulationScenario,
        SimulationClockConfig,
    ) {
        let (model, mut scenario, mut clock) = dynamic_networks::tests::fixture();
        scenario.max_steps = 1000;
        clock.adaptive = Some(AdaptiveIntegrationConfig {
            absolute_tolerance: 1e-11,
            relative_tolerance: 1e-9,
            event_tolerance_s: 1e-9,
            minimum_step_s: 1e-10,
            max_refinements: 32,
        });
        (model, scenario, clock)
    }
    fn finish(
        model: SimulationModel,
        scenario: ConcurrentSimulationScenario,
        clock: SimulationClockConfig,
    ) -> SimulationSessionSnapshot {
        let mut session = SimulationSession::initialize(model, scenario, clock).unwrap();
        while session.snapshot().lifecycle == SimulationSessionLifecycle::Paused {
            session.advance(3).unwrap();
        }
        session.snapshot()
    }
    fn final_x(snapshot: &SimulationSessionSnapshot) -> f64 {
        snapshot.trace.timeline.last().unwrap().values[&("plant".into(), "x".into())]
            .as_f64()
            .unwrap()
    }
    #[test]
    fn adaptive_decay_matches_analytic_solution_and_batch_while_rejecting_large_trials() {
        let (model, scenario, clock) = fixture();
        let batch =
            run_concurrent_simulation_model(&model, scenario.clone(), clock.clone()).unwrap();
        let done = finish(model, scenario, clock);
        assert_eq!(done.trace, batch);
        assert_eq!(done.logical_time_s, 1.0);
        assert_eq!(
            done.trace.termination,
            Some(SimulationTermination::TimeBudgetExhausted)
        );
        assert!((final_x(&done) - (-1.0_f64).exp()).abs() < 1e-8);
        assert!(
            done.trace
                .timeline
                .iter()
                .filter_map(|f| f.integration.as_ref())
                .any(|n| n.rejected_trials > 0)
        );
        assert!(
            done.trace
                .timeline
                .iter()
                .filter_map(|f| f.integration.as_ref())
                .all(|n| n.error_ratio <= 1.0)
        );
    }
    #[test]
    fn localizes_first_solved_guard_including_strict_comparison_and_preserves_priority() {
        for guard in ["y >= -0.5", "y > -0.5"] {
            let (mut model, scenario, clock) = fixture();
            let mut done = model.machines[0].states[0].clone();
            done.id = "done".into();
            done.is_initial = false;
            done.is_final = true;
            done.do_behavior = None;
            model.machines[0].states.push(done);
            let transition = |id: &str, guard: &str| SimulationTransition {
                id: id.into(),
                source: "running".into(),
                target: "done".into(),
                trigger: SimulationTrigger {
                    kind: SimulationTriggerKind::Change,
                    value: Some(guard.into()),
                },
                guard: None,
                effects: vec![],
            };
            model.machines[0].transitions = vec![
                transition("later", "x <= 0.25"),
                transition("earlier", guard),
            ];
            let done = finish(model, scenario, clock);
            assert_eq!(
                done.trace.termination,
                Some(SimulationTermination::FinalState)
            );
            assert!(
                (done.logical_time_s - 2.0_f64.ln()).abs() < 2e-8,
                "{}",
                done.logical_time_s
            );
            let frame = done.trace.timeline.last().unwrap();
            assert_eq!(frame.events[0].transition_id.as_deref(), Some("earlier"));
            assert!(frame.integration.as_ref().unwrap().event_bracket_s.unwrap() <= 1e-9);
            assert!(frame.integration.as_ref().unwrap().event_refinements > 0);
        }
    }
    #[test]
    fn minimum_step_and_refinement_exhaustion_restore_committed_boundary() {
        for minimum in [true, false] {
            let (model, scenario, mut clock) = fixture();
            if minimum {
                clock.adaptive.as_mut().unwrap().minimum_step_s = 0.75;
            } else {
                clock.adaptive.as_mut().unwrap().max_refinements = 1;
            }
            let mut session = SimulationSession::initialize(model, scenario, clock).unwrap();
            let before = session.snapshot();
            let error = session.advance(1).unwrap_err();
            assert!(error.to_string().contains(if minimum {
                "minimum step"
            } else {
                "budget exhausted"
            }));
            let after = session.snapshot();
            assert_eq!(after.lifecycle, SimulationSessionLifecycle::Failed);
            assert_eq!(after.trace.timeline, before.trace.timeline);
            assert_eq!(after.logical_time_s, before.logical_time_s);
            assert_eq!(after.advance_cycles, before.advance_cycles);
        }
    }
    #[test]
    fn sampling_boundaries_and_final_partial_interval_do_not_drift() {
        let (model, scenario, mut clock) = fixture();
        clock.sample_interval_s = 0.17;
        let done = finish(model, scenario, clock);
        for sample in 1..=5 {
            assert!(
                done.trace
                    .timeline
                    .iter()
                    .any(|f| (f.t - sample as f64 * 0.17).abs() < 1e-12)
            );
        }
        assert_eq!(done.logical_time_s, 1.0);
        assert!(
            done.trace
                .timeline
                .windows(2)
                .all(|frames| frames[1].t > frames[0].t)
        );
    }
    #[test]
    fn invalid_adaptive_configuration_is_rejected_before_execution() {
        let (model, scenario, mut clock) = fixture();
        clock.adaptive.as_mut().unwrap().relative_tolerance = f64::NAN;
        assert!(SimulationSession::initialize(model, scenario, clock).is_err());
    }
    #[test]
    fn competing_brackets_select_earliest_event_and_due_timer_caps_trial() {
        for with_timer in [false, true] {
            let (mut model, scenario, clock) = fixture();
            model.machines[0].states[0].do_behavior = Some(StateDoBehavior::RateIntegration {
                rates: vec![SimulationRate {
                    feature: "x".into(),
                    source: SimulationRateSource::Constant(-1.0),
                }],
            });
            let mut done = model.machines[0].states[0].clone();
            done.id = "done".into();
            done.is_initial = false;
            done.is_final = true;
            done.do_behavior = None;
            model.machines[0].states.push(done);
            let transition =
                |id: &str, kind: SimulationTriggerKind, guard: &str| SimulationTransition {
                    id: id.into(),
                    source: "running".into(),
                    target: "done".into(),
                    trigger: SimulationTrigger {
                        kind,
                        value: Some(guard.into()),
                    },
                    guard: None,
                    effects: vec![],
                };
            model.machines[0].transitions = vec![
                transition("later", SimulationTriggerKind::Change, "x <= 0.3"),
                transition("earlier", SimulationTriggerKind::Change, "y >= -0.7"),
            ];
            if with_timer {
                model.machines[0].transitions.push(transition(
                    "timer",
                    SimulationTriggerKind::After,
                    "0.2s",
                ));
            }
            let result = finish(model, scenario, clock);
            let expected = if with_timer { 0.2 } else { 0.3 };
            assert!(
                (result.logical_time_s - expected).abs() <= 1e-9,
                "{}",
                result.logical_time_s
            );
            assert_eq!(
                result.trace.timeline.last().unwrap().events[0]
                    .transition_id
                    .as_deref(),
                Some(if with_timer { "timer" } else { "earlier" })
            );
        }
    }
    #[test]
    fn overflowing_error_scale_fails_without_a_false_zero_error_estimate() {
        let (model, scenario, mut clock) = fixture();
        let config = clock.adaptive.as_mut().unwrap();
        config.absolute_tolerance = f64::MAX;
        config.relative_tolerance = f64::MAX;
        let mut session = SimulationSession::initialize(model, scenario, clock).unwrap();
        let before = session.snapshot();
        assert!(
            session
                .advance(1)
                .unwrap_err()
                .to_string()
                .contains("error scale")
        );
        assert_eq!(session.snapshot().trace.timeline, before.trace.timeline);
    }
    #[test]
    fn unused_lookup_machine_does_not_reject_an_adaptive_scenario() {
        let (mut model, mut scenario, clock) = fixture();
        let mut unused = model.machines[0].clone();
        unused.id = "unused".into();
        unused.label = "unused".into();
        unused.states[0].do_behavior = Some(StateDoBehavior::LookupTable { tables: vec![] });
        model.machines.push(unused);
        assert!(
            SimulationSession::initialize(model.clone(), scenario.clone(), clock.clone()).is_ok()
        );
        scenario.subjects[0].machine_id = "unused".into();
        assert!(
            SimulationSession::initialize(model, scenario, clock)
                .err()
                .unwrap()
                .to_string()
                .contains("lookup-table")
        );
    }
}
