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
// Validate all operands before evaluation: unresolved paths and unsupported branches
// must not disappear through Boolean short circuiting or null equality.
fn supported(expr: &Value, subject: &str, values: &BTreeMap<(String, String), Value>) -> bool {
    match expr.get("kind").and_then(Value::as_str) {
        Some("literal") => expr
            .get("value")
            .is_some_and(|v| v.is_boolean() || v.as_f64().is_some_and(f64::is_finite)),
        Some("path") => {
            expr.get("segments")
                .and_then(Value::as_array)
                .is_some_and(|parts| {
                    !parts.is_empty()
                        && parts.iter().all(|p| {
                            p.as_str()
                                .or_else(|| p.get("name").and_then(Value::as_str))
                                .is_some_and(|name| !name.is_empty())
                        })
                })
                && expression_path(expr)
                    .and_then(|p| resolve_feature_path(&p, subject, values))
                    .is_some_and(|v| v.is_boolean() || v.as_f64().is_some_and(f64::is_finite))
        }
        Some("binary") => {
            matches!(
                expr["op"].as_str(),
                Some(
                    "and"
                        | "or"
                        | "&&"
                        | "||"
                        | "equal"
                        | "=="
                        | "not_equal"
                        | "!="
                        | "greater"
                        | ">"
                        | "greater_equal"
                        | ">="
                        | "less"
                        | "<"
                        | "less_equal"
                        | "<="
                )
            ) && supported(&expr["left"], subject, values)
                && supported(&expr["right"], subject, values)
        }
        _ => false,
    }
}
pub fn evaluate_deadline_requirements(trace: &SimulationTrace) -> Vec<RequirementOutcome> {
    trace
        .requirements
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
            if trace.status != SimulationStatus::Completed {
                out.reason_code = "execution_not_completed".into();
                return out;
            }
            let Some(expression) = &requirement.expression else {
                out.reason_code = "missing_expression".into();
                return out;
            };
            if trace.timeline.is_empty()
                || trace.timeline.iter().any(|f| !f.t.is_finite() || f.t < 0.0)
                || trace.timeline.windows(2).any(|w| w[0].t > w[1].t)
            {
                out.reason_code = "invalid_or_empty_trace".into();
                return out;
            }
            let mut witness = None;
            let mut observed_deadline = false;
            for frame in trace.timeline.iter().filter(|f| f.t <= deadline) {
                if !supported(expression, &trace.subject_id, &frame.values) {
                    out.reason_code = "unsupported_or_unresolved_expression".into();
                    return out;
                }
                let Ok(Value::Bool(value)) =
                    eval_value(expression, &trace.subject_id, &frame.values)
                else {
                    out.reason_code = "non_boolean_or_invalid_expression".into();
                    return out;
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
    fn unknown_operands_cannot_satisfy_by_null_equality_or_short_circuit() {
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
