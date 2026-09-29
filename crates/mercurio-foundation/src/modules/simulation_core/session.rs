//! Resumable deterministic execution. A session owns its model, event queues,
//! logical clock and state history; resuming never reruns an earlier prefix.
use super::*;

/// Engine lifecycle, independent of requirement acceptance and trace health.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SimulationSessionLifecycle {
    Paused,
    Completed,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationSessionSnapshot {
    pub lifecycle: SimulationSessionLifecycle,
    pub logical_time_s: f64,
    /// Existing scenario budget units: fired transitions and integration intervals.
    pub execution_steps: usize,
    /// Atomic scheduling cycles, including each bounded immediate transition closure.
    pub advance_cycles: usize,
    pub trace: SimulationTrace,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// An owned, in-memory session usable by native and WASM hosts.
///
/// Each advance cycle executes the same atomic scheduling turn as the batch API:
/// immediate transitions, one queued event per subject, due timers, and (when
/// nothing fired) one continuous integration interval. A closure can fire more
/// than one transition, bounded by the change-loop and scenario step limits.
/// The host regains control at a consistent boundary between these cycles.
pub struct SimulationSession {
    model: SimulationModel,
    scenario: ConcurrentSimulationScenario,
    clock: SimulationClockConfig,
    evaluator: ExpressionEvaluator,
    subjects: Vec<CoreSubjectRunState>,
    values: BTreeMap<(String, String), Value>,
    pending_signals: VecDeque<CorePendingSignal>,
    history: BTreeMap<(String, String), String>,
    elapsed: BTreeMap<(String, String), f64>,
    t: f64,
    step: usize,
    max_steps: usize,
    cycles: usize,
    status: SimulationStatus,
    timeline: Vec<SimTraceEntry>,
    lifecycle: SimulationSessionLifecycle,
    termination: Option<SimulationTermination>,
    stop_fallback: SimulationTermination,
    error: Option<String>,
}

impl SimulationSession {
    pub fn initialize(
        mut model: SimulationModel,
        scenario: ConcurrentSimulationScenario,
        clock: SimulationClockConfig,
    ) -> Result<Self, CoreSimulationError> {
        let evaluator = ExpressionEvaluator::default();
        validate_simulation_model(&model)?;
        validate_session_inputs(&scenario, &clock)?;
        numerical::validate(&model, &scenario, &clock)?;
        model.constraint_networks.retain(|network| {
            scenario
                .subjects
                .iter()
                .any(|subject| subject.subject_id == network.subject_id)
        });
        dynamic_networks::validate(&model, &scenario)?;
        let mut subjects = Vec::<CoreSubjectRunState>::new();
        for subject in &scenario.subjects {
            if subject.subject_id.is_empty() {
                return Err(CoreSimulationError::MissingSubject(
                    subject.subject_id.clone(),
                ));
            }
            let machine = model
                .machines
                .iter()
                .find(|machine| {
                    machine.id == subject.machine_id || machine.label == subject.machine_id
                })
                .ok_or_else(|| {
                    CoreSimulationError::MissingStateMachine(subject.machine_id.clone())
                })?;
            let active = initial_configuration(machine, subject.initial_state_id.as_deref())
                .ok_or_else(|| {
                    CoreSimulationError::MissingInitialState(subject.machine_id.clone())
                })?;
            subjects.push(CoreSubjectRunState {
                subject_id: subject.subject_id.clone(),
                machine: Arc::new(machine.clone()),
                active,
                event_index: 0,
                events: subject.events.clone(),
            });
        }

        let mut values = scenario.initial_values.clone();
        dynamic_networks::solve(&model, &mut values)?;
        let mut pending_signals = VecDeque::<CorePendingSignal>::new();
        let history = BTreeMap::<(String, String), String>::new();
        let mut elapsed = BTreeMap::<(String, String), f64>::new();
        let t = 0.0;
        let step = 0usize;
        let status = SimulationStatus::Completed;
        for subject in &subjects {
            for state_id in &subject.active {
                if !model.constraint_networks.is_empty() {
                    propagate_model_values(&evaluator, &model, &subjects, &mut values)?;
                }
                elapsed.insert((subject.subject_id.clone(), state_id.clone()), 0.0);
                apply_state_behavior(
                    &evaluator,
                    &subject.machine,
                    state_id,
                    &subject.subject_id,
                    &mut values,
                    &mut pending_signals,
                )?;
                apply_state_lookup_tables(
                    &subject.machine,
                    state_id,
                    &subject.subject_id,
                    &mut values,
                    0.0,
                )?;
            }
        }
        propagate_model_values(&evaluator, &model, &subjects, &mut values)?;

        let timeline = vec![make_core_entry(&model, t, &subjects, &values, Vec::new())?];
        let max_steps = scenario.max_steps.max(1);

        let mut session = Self {
            model,
            scenario,
            clock,
            evaluator,
            subjects,
            values,
            pending_signals,
            history,
            elapsed,
            t,
            step,
            max_steps,
            cycles: 0,
            status,
            timeline,
            lifecycle: SimulationSessionLifecycle::Paused,
            termination: None,
            stop_fallback: SimulationTermination::TimeBudgetExhausted,
            error: None,
        };
        session.finish_at_boundary();
        Ok(session)
    }

    /// Advance at most `cycles` atomic scheduling turns. Zero is read-only.
    /// Completed/cancelled sessions return their unchanged terminal snapshot.
    pub fn advance(
        &mut self,
        cycles: usize,
    ) -> Result<SimulationSessionSnapshot, CoreSimulationError> {
        for _ in 0..cycles {
            if self.lifecycle != SimulationSessionLifecycle::Paused {
                break;
            }
            // Preserve the last committed boundary if expression evaluation fails.
            let checkpoint = (
                self.subjects.clone(),
                self.values.clone(),
                self.pending_signals.clone(),
                self.history.clone(),
                self.elapsed.clone(),
                self.t,
                self.step,
                self.status,
                self.timeline.len(),
                self.stop_fallback,
            );
            let fired = match self.advance_cycle() {
                Ok(fired) => fired,
                Err(error) => {
                    let timeline_len = checkpoint.8;
                    (
                        self.subjects,
                        self.values,
                        self.pending_signals,
                        self.history,
                        self.elapsed,
                        self.t,
                        self.step,
                        self.status,
                        _,
                        self.stop_fallback,
                    ) = checkpoint;
                    self.timeline.truncate(timeline_len);
                    self.status = SimulationStatus::Failed;
                    self.lifecycle = SimulationSessionLifecycle::Failed;
                    self.error = Some(error.to_string());
                    return Err(error);
                }
            };
            self.cycles += 1;
            self.finish_at_boundary();
            if !fired && self.lifecycle == SimulationSessionLifecycle::Paused {
                self.finish(self.stop_fallback);
            }
        }
        Ok(self.snapshot())
    }

    /// Append an external event to one subject's FIFO at a paused boundary.
    /// Names need not match a transition: unmatched events are recorded as dropped
    /// by the same engine path as scripted events. Event identities must be unique
    /// within that subject for the lifetime of the session.
    pub fn inject_event(
        &mut self,
        subject_id: &str,
        event: SimulationEvent,
    ) -> Result<(), CoreSimulationError> {
        if self.lifecycle != SimulationSessionLifecycle::Paused {
            return Err(CoreSimulationError::InvalidSessionOperation(
                "events require a paused session".into(),
            ));
        }
        if event.id.trim().is_empty() || event.trigger.trim().is_empty() {
            return Err(CoreSimulationError::InvalidSessionOperation(
                "event id and trigger must be nonempty".into(),
            ));
        }
        let subject = self
            .subjects
            .iter_mut()
            .find(|subject| subject.subject_id == subject_id)
            .ok_or_else(|| CoreSimulationError::MissingSubject(subject_id.into()))?;
        if configuration_is_final(&subject.machine, &subject.active) {
            return Err(CoreSimulationError::InvalidSessionOperation(format!(
                "subject `{subject_id}` has reached its final state"
            )));
        }
        if subject
            .events
            .iter()
            .any(|existing| existing.id == event.id)
        {
            return Err(CoreSimulationError::InvalidSessionOperation(format!(
                "duplicate event id `{}` for subject `{subject_id}`",
                event.id
            )));
        }
        subject.events.push(event);
        Ok(())
    }

    /// Cancellation is idempotent and preserves the last committed trace.
    pub fn cancel(&mut self) -> SimulationSessionSnapshot {
        if self.lifecycle == SimulationSessionLifecycle::Paused {
            self.lifecycle = SimulationSessionLifecycle::Cancelled;
            self.termination = Some(SimulationTermination::Cancelled);
        }
        self.snapshot()
    }

    pub fn snapshot(&self) -> SimulationSessionSnapshot {
        SimulationSessionSnapshot {
            lifecycle: self.lifecycle,
            logical_time_s: self.t,
            execution_steps: self.step,
            advance_cycles: self.cycles,
            trace: self.trace(),
            error: self.error.clone(),
        }
    }

    /// Drain the same scheduler used by `advance` without cloning intermediate traces.
    pub fn run_to_completion(&mut self) -> Result<SimulationTrace, CoreSimulationError> {
        Ok(self.advance(usize::MAX)?.trace)
    }

    fn finish(&mut self, termination: SimulationTermination) {
        self.lifecycle = SimulationSessionLifecycle::Completed;
        self.termination = Some(termination);
    }

    fn finish_at_boundary(&mut self) {
        if all_subjects_final(&self.subjects) {
            self.finish(SimulationTermination::FinalState);
        } else if let Some(reason) =
            policy_stop_reason(&self.evaluator, &self.scenario, self.status, &self.timeline)
        {
            self.finish(reason);
        } else if self.step >= self.max_steps {
            self.finish(SimulationTermination::StepBudgetExhausted);
        } else if self.t > self.clock.max_time_s {
            self.finish(SimulationTermination::TimeBudgetExhausted);
        }
    }

    fn trace(&self) -> SimulationTrace {
        let generated_channels = generated_continuous_channels(&self.subjects);
        let channels = self
            .values
            .keys()
            .map(|(subject, feature)| SimTraceChannel {
                id: format!("{subject}.{feature}"),
                unit: None,
                source: generated_channels
                    .get(&(subject.clone(), feature.clone()))
                    .cloned()
                    .unwrap_or(SimTraceChannelSource::AssignEffect),
            })
            .collect();
        SimulationTrace {
            configuration: Some(SimulationRunConfiguration {
                schema_version: 1,
                max_steps: self.max_steps,
                clock: self.clock.clone(),
                termination_policy: self.scenario.termination_policy.clone(),
            }),
            scenario_id: self.scenario.id.clone(),
            subject_id: self
                .scenario
                .subjects
                .first()
                .map(|subject| subject.subject_id.clone())
                .unwrap_or_default(),
            channels,
            timeline: self.timeline.clone(),
            status: self.status,
            termination: self.termination,
            requirements: self.scenario.requirements.clone(),
            objectives: self.scenario.objectives.clone(),
        }
    }

    fn advance_cycle(&mut self) -> Result<bool, CoreSimulationError> {
        let evaluator = &self.evaluator;
        let mut integration = None;
        let mut fired = false;
        let mut events = Vec::<SimTraceEvent>::new();

        if fire_immediate_transitions(
            &self.model,
            evaluator,
            &mut self.subjects,
            &mut self.values,
            &mut self.pending_signals,
            &mut self.history,
            &mut self.elapsed,
            &mut self.step,
            self.max_steps,
            self.clock.change_loop_limit,
            &mut events,
        )? {
            propagate_model_values(evaluator, &self.model, &self.subjects, &mut self.values)?;
            fired = true;
        }

        let mut scripted_event_fired = false;
        for subject_index in 0..self.subjects.len() {
            if !self.model.constraint_networks.is_empty() {
                propagate_model_values(evaluator, &self.model, &self.subjects, &mut self.values)?;
            }
            let subject = &mut self.subjects[subject_index];
            if configuration_is_final(&subject.machine, &subject.active)
                || self.step >= self.max_steps
                || subject.event_index >= subject.events.len()
            {
                continue;
            }
            let event = subject.events[subject.event_index].clone();
            subject.event_index += 1;
            let Some(transition) = select_transition(
                evaluator,
                &subject.machine,
                &subject.active,
                SimulationTriggerKind::Event,
                &event.trigger,
                &subject.subject_id,
                &self.values,
            )?
            .cloned() else {
                events.push(SimTraceEvent {
                    kind: "event.dropped".to_string(),
                    subject_id: Some(subject.subject_id.clone()),
                    transition_id: None,
                    trigger: Some(event.trigger),
                    reason: Some("no enabled transition matched event trigger".to_string()),
                });
                self.status = SimulationStatus::Blocked;
                fired = true;
                continue;
            };
            self.step += 1;
            let before = subject.active.clone();
            apply_effects(
                evaluator,
                &transition.effects,
                &subject.subject_id,
                &mut self.values,
                &mut self.pending_signals,
            )?;
            subject.active = apply_state_change(
                evaluator,
                &subject.machine,
                &subject.subject_id,
                &before,
                &transition.source,
                &transition.target,
                &mut self.values,
                &mut self.pending_signals,
                &mut self.history,
                &mut self.elapsed,
            )?;
            events.push(SimTraceEvent {
                kind: "transition".to_string(),
                subject_id: Some(subject.subject_id.clone()),
                transition_id: Some(transition.id),
                trigger: Some(event.trigger),
                reason: None,
            });
            scripted_event_fired = true;
            fired = true;
        }
        if scripted_event_fired {
            propagate_model_values(evaluator, &self.model, &self.subjects, &mut self.values)?;
        }

        if fire_immediate_transitions(
            &self.model,
            evaluator,
            &mut self.subjects,
            &mut self.values,
            &mut self.pending_signals,
            &mut self.history,
            &mut self.elapsed,
            &mut self.step,
            self.max_steps,
            self.clock.change_loop_limit,
            &mut events,
        )? {
            propagate_model_values(evaluator, &self.model, &self.subjects, &mut self.values)?;
            fired = true;
        }

        // Process already-due absolute times and zero-duration after triggers
        // before advancing the clock; scripted events retain their priority.
        if fire_after_transitions(
            &self.model,
            evaluator,
            &mut self.subjects,
            &mut self.values,
            &mut self.pending_signals,
            &mut self.history,
            &mut self.elapsed,
            &mut self.step,
            self.max_steps,
            &mut events,
            self.t,
        )? {
            propagate_model_values(evaluator, &self.model, &self.subjects, &mut self.values)?;
            fired = true;
        }

        if !fired && self.step < self.max_steps && !all_subjects_final(&self.subjects) {
            let next_after = next_after_duration(
                evaluator,
                &self.subjects,
                &self.elapsed,
                &self.values,
                self.t,
            )?;
            let next_change = if self.clock.adaptive.is_some() {
                None
            } else {
                next_change_crossing_duration(evaluator, &self.subjects, &self.values)?
            };
            let fixed_step = self.clock.fixed_step_s.max(0.0);
            let mut duration = [Some(fixed_step), next_after, next_change]
                .into_iter()
                .flatten()
                .filter(|duration| duration.is_finite() && *duration >= 0.0)
                .min_by(|left, right| left.total_cmp(right))
                .unwrap_or(fixed_step);
            if fixed_step > 0.0 {
                duration = duration.min(fixed_step);
            }
            if duration <= 0.0 {
                duration = fixed_step;
            }
            if self.clock.adaptive.is_some() {
                duration = duration.min((self.clock.max_time_s - self.t).max(0.0));
                let interval = self.clock.sample_interval_s;
                if interval > 0.0 {
                    let mut next = ((self.t / interval).floor() + 1.0) * interval;
                    if next <= self.t {
                        next += interval;
                    }
                    duration = duration.min(next - self.t);
                }
            }
            self.stop_fallback = if duration > 0.0
                || (self.clock.adaptive.is_some() && self.t >= self.clock.max_time_s)
            {
                SimulationTermination::TimeBudgetExhausted
            } else {
                SimulationTermination::Quiescent
            };
            if duration > 0.0 && self.t + duration <= self.clock.max_time_s {
                if let Some(config) = &self.clock.adaptive {
                    let accepted = numerical::advance(
                        &self.model,
                        evaluator,
                        &self.subjects,
                        &self.values,
                        duration,
                        config,
                    )?;
                    duration = accepted.evidence.accepted_step_s;
                    if self.t + duration <= self.t {
                        return Err(CoreSimulationError::InvalidSessionOperation(
                            "simulation.numerics: accepted step cannot advance logical time".into(),
                        ));
                    }
                    self.values = accepted.values;
                    for subject in &self.subjects {
                        for state_id in &subject.active {
                            *self
                                .elapsed
                                .entry((subject.subject_id.clone(), state_id.clone()))
                                .or_default() += duration;
                        }
                    }
                    integration = Some(accepted.evidence);
                } else {
                    integrate_active_state_behaviors(
                        &self.model,
                        evaluator,
                        &self.subjects,
                        &mut self.values,
                        &mut self.elapsed,
                        duration,
                        self.clock.sample_interval_s,
                        &mut self.timeline,
                        self.t,
                    )?;
                }
                propagate_model_values(evaluator, &self.model, &self.subjects, &mut self.values)?;
                self.t += duration;
                self.step += 1;
                fired = true;
                fire_after_transitions(
                    &self.model,
                    evaluator,
                    &mut self.subjects,
                    &mut self.values,
                    &mut self.pending_signals,
                    &mut self.history,
                    &mut self.elapsed,
                    &mut self.step,
                    self.max_steps,
                    &mut events,
                    self.t,
                )?;
                propagate_model_values(evaluator, &self.model, &self.subjects, &mut self.values)?;
                fire_immediate_transitions(
                    &self.model,
                    evaluator,
                    &mut self.subjects,
                    &mut self.values,
                    &mut self.pending_signals,
                    &mut self.history,
                    &mut self.elapsed,
                    &mut self.step,
                    self.max_steps,
                    self.clock.change_loop_limit,
                    &mut events,
                )?;
                propagate_model_values(evaluator, &self.model, &self.subjects, &mut self.values)?;
            }
        }

        if fired {
            let mut entry =
                make_core_entry(&self.model, self.t, &self.subjects, &self.values, events)?;
            entry.integration = integration;
            self.timeline.push(entry);
        }
        Ok(fired)
    }
}

fn validate_session_inputs(
    scenario: &ConcurrentSimulationScenario,
    clock: &SimulationClockConfig,
) -> Result<(), CoreSimulationError> {
    for (name, value) in [
        ("max_time_s", clock.max_time_s),
        ("fixed_step_s", clock.fixed_step_s),
        ("sample_interval_s", clock.sample_interval_s),
    ] {
        if !value.is_finite() || value < 0.0 {
            return Err(CoreSimulationError::InvalidSessionOperation(format!(
                "{name} must be finite and nonnegative"
            )));
        }
    }
    let mut subject_ids = BTreeSet::new();
    for subject in &scenario.subjects {
        if !subject_ids.insert(&subject.subject_id) {
            return Err(CoreSimulationError::InvalidSessionOperation(format!(
                "duplicate subject id `{}`",
                subject.subject_id
            )));
        }
    }
    Ok(())
}
