//! Bounded sampled-trace eventuality: a Boolean requirement holds at a sample by its deadline.
use super::*;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequirementOutcome {
    pub requirement_id: String,
    pub status: String,
    pub reason_code: String,
    pub evidence_strength: String,
    pub deadline_s: Option<f64>,
    pub witness_time_s: Option<f64>,
}
pub fn evaluate_deadline_requirements(trace: &SimulationTrace) -> Vec<RequirementOutcome> {
    evaluate_samples(
        &ExpressionEvaluator::default(),
        &trace.subject_id,
        trace.status,
        &trace.requirements,
        &trace.timeline,
    )
}
pub(super) fn evaluate_samples(
    evaluator: &ExpressionEvaluator,
    subject_id: &str,
    status: SimulationStatus,
    requirements: &[SimulationRequirement],
    timeline: &[SimTraceEntry],
) -> Vec<RequirementOutcome> {
    requirements
        .iter()
        .map(|requirement| {
            let mut out = RequirementOutcome {
                requirement_id: requirement.id.clone(),
                status: "unevaluated".into(),
                reason_code: "missing_deadline".into(),
                evidence_strength: "sampled_simulation".into(),
                deadline_s: requirement.deadline_s,
                witness_time_s: None,
            };
            let Some(deadline) = requirement.deadline_s else {
                return out;
            };
            if !deadline.is_finite() || deadline < 0.0 {
                out.reason_code = "invalid_deadline".into();
                return out;
            }
            if status != SimulationStatus::Completed {
                out.reason_code = "execution_not_completed".into();
                return out;
            }
            let Some(expression) = &requirement.expression else {
                out.reason_code = "missing_expression".into();
                return out;
            };
            if timeline.is_empty()
                || timeline.iter().any(|f| !f.t.is_finite() || f.t < 0.0)
                || timeline.windows(2).any(|w| w[0].t > w[1].t)
            {
                out.reason_code = "invalid_or_empty_trace".into();
                return out;
            }
            let mut witness = None;
            let mut observed_deadline = false;
            for frame in timeline.iter().filter(|f| f.t <= deadline) {
                let value = match eval_bool(evaluator, expression, subject_id, &frame.values) {
                    Ok(value) => value,
                    Err(error) => {
                        out.reason_code = if error.to_string().contains("expected Boolean") {
                            "non_boolean_or_invalid_expression"
                        } else {
                            "unsupported_or_unresolved_expression"
                        }
                        .into();
                        return out;
                    }
                };
                if value && witness.is_none() {
                    witness = Some(frame.t);
                }
                observed_deadline |= frame.t == deadline;
            }
            if !observed_deadline {
                out.reason_code = "deadline_not_observed".into();
                return out;
            }
            if let Some(t) = witness {
                out.status = "satisfied".into();
                out.reason_code = "witness_by_deadline".into();
                out.witness_time_s = Some(t);
            } else if observed_deadline {
                out.status = "violated".into();
                out.reason_code = "no_witness_by_deadline".into();
            } else {
                out.reason_code = "deadline_not_observed".into();
            }
            out
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn trace() -> SimulationTrace {
        SimulationTrace {
            configuration: None,
            termination: None,
            scenario_id: "case".into(),
            subject_id: "chamber".into(),
            channels: vec![],
            status: SimulationStatus::Completed,
            objectives: vec![],
            requirements: vec![SimulationRequirement {
                id: "req".into(),
                label: "deadline".into(),
                deadline_s: Some(5.0),
                expression: Some(
                    json!({"kind":"binary","op":"greater_equal","left":{"kind":"path","segments":["temperature"]},"right":{"kind":"literal","value":80}}),
                ),
            }],
            timeline: (0..=6)
                .map(|t| SimTraceEntry {
                    t: t as f64,
                    states: BTreeMap::new(),
                    events: vec![],
                    values: BTreeMap::from([(
                        ("chamber".into(), "temperature".into()),
                        json!(20 + 10 * t),
                    )]),
                })
                .collect(),
        }
    }
    #[test]
    fn shared_expression_arithmetic_requirement() {
        let mut t = trace();
        let difference = json!({"kind":"binary","op":"subtract",
            "left":{"kind":"path","segments":["temperature"]},
            "right":{"kind":"literal","value":20}});
        t.requirements[0].expression = Some(json!({"kind":"binary","op":"greater_equal",
            "left":difference,"right":{"kind":"literal","value":40}}));
        let outcome = evaluate_deadline_requirements(&t);
        assert_eq!(outcome[0].status, "satisfied");
        assert_eq!(outcome[0].witness_time_s, Some(4.0));
    }

    #[test]
    fn shared_expression_strict_guard_and_negation() {
        let values = BTreeMap::new();
        assert!(
            eval_bool(
                &ExpressionEvaluator::default(),
                &json!({"kind":"literal","value":1}),
                "chamber",
                &values
            )
            .is_err()
        );
        assert_eq!(
            eval_number(
                &ExpressionEvaluator::default(),
                &json!({"kind":"unary","op":"negate",
            "expr":{"kind":"literal","value":3}}),
                "chamber",
                &values
            )
            .unwrap(),
            -3.0
        );
        let missing = json!({"kind":"path","segments":["missing"]});
        assert!(
            eval_value(
                &ExpressionEvaluator::default(),
                &json!({"kind":"binary","op":"equal", "left":missing,"right":missing}),
                "chamber",
                &values
            )
            .is_err()
        );
    }

    #[test]
    fn stopping_policy_requires_evaluated_evidence_and_respects_blocked_health() {
        let mut trace = trace();
        let mut scenario = ConcurrentSimulationScenario {
            termination_policy: SimulationTerminationPolicy {
                on_all_satisfied: true,
                on_any_violated: true,
                on_blocked: true,
            },
            id: "case".into(),
            subjects: vec![ConcurrentSubjectScenario {
                subject_id: "chamber".into(),
                machine_id: "machine".into(),
                initial_state_id: None,
                events: vec![],
            }],
            max_steps: 100,
            step_duration_s: 1.0,
            clock_config: None,
            initial_values: BTreeMap::new(),
            requirements: trace.requirements.clone(),
            objectives: vec![],
        };
        assert_eq!(
            policy_stop_reason(
                &ExpressionEvaluator::default(),
                &scenario,
                SimulationStatus::Completed,
                &trace.timeline[..5]
            ),
            None
        );
        assert_eq!(
            policy_stop_reason(
                &ExpressionEvaluator::default(),
                &scenario,
                SimulationStatus::Completed,
                &trace.timeline
            ),
            Some(SimulationTermination::RequirementViolated)
        );
        scenario.requirements[0].deadline_s = Some(6.0);
        assert_eq!(
            policy_stop_reason(
                &ExpressionEvaluator::default(),
                &scenario,
                SimulationStatus::Completed,
                &trace.timeline
            ),
            Some(SimulationTermination::RequirementsSatisfied)
        );
        assert_eq!(
            policy_stop_reason(
                &ExpressionEvaluator::default(),
                &scenario,
                SimulationStatus::Blocked,
                &trace.timeline
            ),
            Some(SimulationTermination::Blocked)
        );
        scenario.termination_policy.on_blocked = false;
        assert_eq!(
            policy_stop_reason(
                &ExpressionEvaluator::default(),
                &scenario,
                SimulationStatus::Blocked,
                &trace.timeline
            ),
            None
        );
        trace.timeline[0].values.clear();
        assert_eq!(
            policy_stop_reason(
                &ExpressionEvaluator::default(),
                &scenario,
                SimulationStatus::Completed,
                &trace.timeline
            ),
            None
        );
        scenario.requirements.clear();
        assert_eq!(
            policy_stop_reason(
                &ExpressionEvaluator::default(),
                &scenario,
                SimulationStatus::Completed,
                &trace.timeline
            ),
            None
        );
        let mut legacy = serde_json::to_value(&scenario).unwrap();
        legacy.as_object_mut().unwrap().remove("termination_policy");
        assert_eq!(
            serde_json::from_value::<ConcurrentSimulationScenario>(legacy)
                .unwrap()
                .termination_policy,
            SimulationTerminationPolicy::default()
        );
    }

    #[test]
    fn deadline_boundaries_and_incomplete_runs() {
        let mut t = trace();
        assert_eq!(evaluate_deadline_requirements(&t)[0].status, "violated");
        t.requirements[0].deadline_s = Some(6.0);
        let out = evaluate_deadline_requirements(&t);
        assert_eq!(out[0].status, "satisfied");
        assert_eq!(out[0].witness_time_s, Some(6.0));
        t.status = SimulationStatus::Blocked;
        assert_eq!(
            evaluate_deadline_requirements(&t)[0].reason_code,
            "execution_not_completed"
        );
        t.status = SimulationStatus::Completed;
        t.requirements[0].deadline_s = Some(7.0);
        assert_eq!(
            evaluate_deadline_requirements(&t)[0].reason_code,
            "deadline_not_observed"
        );
        t.requirements[0].deadline_s = Some(5.5);
        assert_eq!(evaluate_deadline_requirements(&t)[0].status, "unevaluated");
        t.requirements[0].deadline_s = None;
        assert_eq!(
            evaluate_deadline_requirements(&t)[0].reason_code,
            "missing_deadline"
        );
    }
    #[test]
    fn required_missing_values_and_unsupported_programs_remain_unevaluated() {
        let mut t = trace();
        let missing = json!({"kind":"path","segments":["missing"]});
        for expression in [
            json!({"kind":"binary","op":"equal","left":missing,"right":missing}),
            json!({"kind":"binary","op":"or","left":{"kind":"literal","value":true},"right":{"kind":"call","name":"unknown"}}),
            json!({"kind":"literal","value":1}),
        ] {
            t.requirements[0].expression = Some(expression);
            assert_eq!(evaluate_deadline_requirements(&t)[0].status, "unevaluated");
        }
    }
}
