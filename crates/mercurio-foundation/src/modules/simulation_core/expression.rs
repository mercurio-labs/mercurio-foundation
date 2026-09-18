//! Prepared expression evaluation over simulation snapshots. Operators belong to KIR.
use super::*;
use crate::kir::{
    ExpressionEvaluationContext, ExpressionEvaluationError, ExpressionIr, ExpressionPathSegment,
};
use std::cell::RefCell;

#[derive(Default)]
pub(super) struct ExpressionEvaluator {
    // Per-run cache: preparation is independent of changing snapshot bindings.
    prepared: RefCell<BTreeMap<String, Result<ExpressionIr, String>>>,
}

impl ExpressionEvaluator {
    pub(super) fn evaluate(
        &self,
        expression: &Value,
        subject: &str,
        values: &BTreeMap<(String, String), Value>,
    ) -> Result<Value, CoreSimulationError> {
        let key = expression.to_string();
        let mut prepared = self.prepared.borrow_mut();
        let ir = prepared
            .entry(key)
            .or_insert_with(|| {
                let ir = if expression.is_object() {
                    ExpressionIr::from_value(expression).map_err(|error| error.to_string())?
                } else {
                    ExpressionIr::Literal {
                        value: expression.clone(),
                    }
                };
                ir.validate_runtime_supported()
                    .map_err(|error| error.to_string())?;
                Ok(ir)
            })
            .as_ref()
            .map_err(|error| CoreSimulationError::InvalidExpression(error.clone()))?;
        ir.evaluate(&mut SnapshotContext { subject, values })
            .map_err(|error| CoreSimulationError::InvalidExpression(format!("{subject}: {error}")))
    }
}

pub(super) struct SnapshotContext<'a> {
    pub subject: &'a str,
    pub values: &'a BTreeMap<(String, String), Value>,
}

impl ExpressionEvaluationContext for SnapshotContext<'_> {
    fn owner_id(&self) -> &str {
        self.subject
    }

    fn resolve_lexical_path(
        &mut self,
        segments: &[ExpressionPathSegment],
    ) -> Result<Vec<Value>, ExpressionEvaluationError> {
        let path = segments
            .iter()
            .map(|segment| match segment {
                ExpressionPathSegment::Resolved {
                    feature: Some(feature),
                    ..
                } => feature.as_str(),
                _ => segment.name(),
            })
            .collect::<Vec<_>>()
            .join(".");
        Err(ExpressionEvaluationError::InvalidExpression(format!(
            "simulation snapshots do not support nonlocal lexical capture `{path}`"
        )))
    }

    fn resolve_path(
        &mut self,
        segments: &[ExpressionPathSegment],
    ) -> Result<Vec<Value>, ExpressionEvaluationError> {
        let path = segments
            .iter()
            .map(ExpressionPathSegment::name)
            .collect::<Vec<_>>()
            .join(".");
        let value = resolve_feature_path(&path, self.subject, self.values).ok_or_else(|| {
            ExpressionEvaluationError::MissingBinding(format!("{}.{}", self.subject, path))
        })?;
        // Snapshot array values represent collection bindings, just as graph paths do.
        match value {
            Value::Null => Err(ExpressionEvaluationError::MissingBinding(path)),
            Value::Array(values) => Ok(values),
            value => Ok(vec![value]),
        }
    }
}

// Compatibility grammar for the pre-IR core profile: one comparison, a Boolean,
// or a feature path. This only translates input; KIR evaluates every operator.
// SysML expressions with arithmetic must arrive as IR from the language adapter.
pub(super) fn legacy_guard_ir(text: &str) -> Result<Value, CoreSimulationError> {
    fn operand(text: &str) -> Result<Value, CoreSimulationError> {
        let text = text.trim();
        if let Ok(value) = text.parse::<f64>() {
            if !value.is_finite() {
                return Err(CoreSimulationError::InvalidExpression(text.into()));
            }
            return Ok(serde_json::json!({"kind":"literal","value":value}));
        }
        if text == "true" || text == "false" {
            return Ok(serde_json::json!({"kind":"literal","value":text == "true"}));
        }
        if text.is_empty()
            || text.split('.').any(|part| {
                part.is_empty() || !part.chars().all(|c| c.is_alphanumeric() || c == '_')
            })
        {
            return Err(CoreSimulationError::InvalidExpression(format!(
                "unsupported legacy guard operand `{text}`"
            )));
        }
        let text = text.strip_prefix("self.").unwrap_or(text);
        Ok(serde_json::json!({"kind":"path","segments":text.split('.').collect::<Vec<_>>()}))
    }
    for op in [">=", "<=", "==", "!=", ">", "<"] {
        if let Some((left, right)) = text.split_once(op) {
            return Ok(
                serde_json::json!({"kind":"binary","op":op,"left":operand(left)?,"right":operand(right)?}),
            );
        }
    }
    operand(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn shared_expression_aliases_contexts_and_errors() {
        let evaluator = ExpressionEvaluator::default();
        let mut values = BTreeMap::from([
            (("chamber".into(), "x".into()), json!(8.0)),
            (("ambient".into(), "x".into()), json!(2.0)),
        ]);
        for op in ["subtract", "sub", "minus", "-"] {
            let expression = json!({"kind":"binary","op":op,
                "left":{"kind":"path","segments":[{"name":"x","feature":"attribute.Chamber.x"}]},
                "right":{"kind":"path","segments":["ambient","x"]}});
            assert_eq!(
                evaluator.evaluate(&expression, "chamber", &values).unwrap(),
                json!(6.0)
            );
            let canonical = crate::kir::ExpressionIr::from_value(&expression).unwrap();
            assert_eq!(
                canonical
                    .evaluate(&mut SnapshotContext {
                        subject: "chamber",
                        values: &values
                    })
                    .unwrap(),
                json!(6.0)
            );
        }
        let negation = json!({"kind":"unary","op":"-","operand":{"kind":"path","segments":["x"]}});
        assert_eq!(
            evaluator.evaluate(&negation, "chamber", &values).unwrap(),
            json!(-8.0)
        );
        let cache_size = evaluator.prepared.borrow().len();
        values.insert(("chamber".into(), "x".into()), json!(3.0));
        assert_eq!(
            evaluator.evaluate(&negation, "chamber", &values).unwrap(),
            json!(-3.0)
        );
        assert_eq!(evaluator.prepared.borrow().len(), cache_size);
        for expression in [
            json!({"kind":"binary","op":"divide","left":{"kind":"literal","value":1},"right":{"kind":"literal","value":0}}),
            json!({"kind":"binary","op":"multiply","left":{"kind":"literal","value":1e308},"right":{"kind":"literal","value":1e308}}),
            json!({"kind":"path","segments":[""]}),
            json!({"kind":"binary","op":"subtract","left":{"kind":"literal","value":"bad"},"right":{"kind":"literal","value":1}}),
            json!({"kind":"call","function":"unsupported","args":[]}),
        ] {
            assert!(
                evaluator.evaluate(&expression, "chamber", &values).is_err(),
                "{expression}"
            );
        }
    }

    #[test]
    fn conditional_operators_skip_unused_runtime_values_but_validate_structure() {
        let evaluator = ExpressionEvaluator::default();
        let values = BTreeMap::new();
        let missing = json!({"kind":"path","segments":["missing"]});
        for (op, left, expected) in [("or", true, true), ("and", false, false)] {
            let expression = json!({"kind":"binary","op":op,
                "left":{"kind":"literal","value":left},"right":missing});
            assert_eq!(
                evaluator.evaluate(&expression, "owner", &values).unwrap(),
                json!(expected)
            );
        }
        // Required values remain errors, never null sentinels that compare equal.
        let required = json!({"kind":"binary","op":"equal","left":missing,"right":missing});
        assert!(evaluator.evaluate(&required, "owner", &values).is_err());
        // Capability validation covers the whole IR, including unselected branches.
        let unsupported = json!({"kind":"binary","op":"or","left":{"kind":"literal","value":true},
            "right":{"kind":"call","function":"unknown","args":[]}});
        assert!(evaluator.evaluate(&unsupported, "owner", &values).is_err());
    }

    #[test]
    fn shared_expression_assignments_are_sequential_and_fail_before_writing() {
        let evaluator = ExpressionEvaluator::default();
        let mut values = BTreeMap::from([(("chamber".into(), "x".into()), json!(8.0))]);
        let mut signals = VecDeque::new();
        let difference = json!({"kind":"binary","op":"subtract", "left":{"kind":"path","segments":["x"]}, "right":{"kind":"literal","value":2}});
        let assignment = SimulationActionNode::Effect(SimulationEffect::AssignExpression {
            feature: "x".into(),
            expression: difference,
        });
        apply_action_sequence(
            &evaluator,
            &SimulationActionSequence {
                actions: vec![assignment.clone(), assignment],
            },
            "chamber",
            &mut values,
            &mut signals,
        )
        .unwrap();
        assert_eq!(values[&("chamber".into(), "x".into())], json!(4.0));
        let bad = SimulationActionSequence {
            actions: vec![
                SimulationActionNode::Effect(SimulationEffect::AssignExpression {
                    feature: "x".into(),
                    expression: json!({"kind":"path","segments":["missing"]}),
                }),
                SimulationActionNode::Effect(SimulationEffect::Assign(AssignEffect {
                    feature: "later".into(),
                    value: json!(true),
                })),
            ],
        };
        assert!(
            apply_action_sequence(&evaluator, &bad, "chamber", &mut values, &mut signals).is_err()
        );
        assert_eq!(values[&("chamber".into(), "x".into())], json!(4.0));
        assert!(!values.contains_key(&("chamber".into(), "later".into())));
        let decision = SimulationActionSequence {
            actions: vec![SimulationActionNode::Decision {
                guard: SimulationGuard::RuntimeFeature("missing".into()),
                then_branch: SimulationActionSequence { actions: vec![] },
                else_branch: Some(SimulationActionSequence {
                    actions: vec![SimulationActionNode::Effect(SimulationEffect::Assign(
                        AssignEffect {
                            feature: "else".into(),
                            value: json!(true),
                        },
                    ))],
                }),
            }],
        };
        assert!(
            apply_action_sequence(&evaluator, &decision, "chamber", &mut values, &mut signals)
                .is_err()
        );
        assert!(!values.contains_key(&("chamber".into(), "else".into())));
        assert!(
            apply_effect(
                &evaluator,
                &SimulationEffect::AssignExpression {
                    feature: "typo".into(),
                    expression: json!({"kind":"literal","value":5}),
                },
                "chamber",
                &mut values,
                &mut signals,
            )
            .is_err()
        );
        assert!(!values.contains_key(&("chamber".into(), "typo".into())));
        // JSON objects assigned by the legacy literal variant remain data.
        let literal = json!({"kind":"path","segments":["missing"]});
        apply_effect(
            &evaluator,
            &SimulationEffect::Assign(AssignEffect {
                feature: "data".into(),
                value: literal.clone(),
            }),
            "chamber",
            &mut values,
            &mut signals,
        )
        .unwrap();
        assert_eq!(values[&("chamber".into(), "data".into())], literal);
    }
    fn resolved_test_path(feature: &str, name: &str) -> Value {
        json!({"kind": "path", "segments": [{"name": name, "feature": feature}]})
    }

    #[test]
    fn simulation_invocations_reject_definition_body_and_default_captures() {
        let evaluator = ExpressionEvaluator::default();
        let values = BTreeMap::from([(("caller".into(), "bias".into()), json!(99))]);
        let lexical_bias = resolved_test_path("feature.Library.bias", "bias");
        for expression in [
            json!({"kind": "invoke", "function": "type.Library.F", "bindings": [], "body": lexical_bias}),
            json!({"kind": "invoke", "function": "type.Library.F", "bindings": [
                {"feature": "feature.Library.F.x", "lexical": true, "expression": lexical_bias}
            ], "body": resolved_test_path("feature.Library.F.x", "x")}),
        ] {
            let error = evaluator
                .evaluate(&expression, "caller", &values)
                .unwrap_err()
                .to_string();
            assert!(error.contains("nonlocal lexical capture"), "{error}");
            assert!(error.contains("feature.Library.bias"), "{error}");
        }
    }

    #[test]
    fn simulation_invocation_caller_arguments_and_local_defaults_remain_executable() {
        let evaluator = ExpressionEvaluator::default();
        let values = BTreeMap::from([(("caller".into(), "bias".into()), json!(7))]);
        let expression = json!({"kind": "invoke", "function": "type.Library.F", "bindings": [
            {"feature": "feature.Library.F.x", "expression": resolved_test_path("feature.Caller.bias", "bias")},
            {"feature": "feature.Library.F.y", "lexical": true, "expression": {
                "kind": "binary", "op": "add", "left": resolved_test_path("feature.Library.F.x", "x"),
                "right": {"kind": "literal", "value": 2}
            }}
        ], "body": resolved_test_path("feature.Library.F.y", "y")});
        assert_eq!(
            evaluator.evaluate(&expression, "caller", &values).unwrap(),
            json!(9)
        );
    }

    #[test]
    fn nested_simulation_frames_preserve_lexical_capture_signal() {
        let evaluator = ExpressionEvaluator::default();
        let values = BTreeMap::from([(("caller".into(), "bias".into()), json!(99))]);
        for nested in [
            json!({"kind": "invoke", "function": "type.Inner", "bindings": [],
                "body": resolved_test_path("feature.Library.bias", "bias")}),
            json!({"kind": "invoke", "function": "type.Inner", "bindings": [
                {"feature": "feature.Inner.x", "lexical": false,
                    "expression": resolved_test_path("feature.Library.bias", "bias")}
            ], "body": resolved_test_path("feature.Inner.x", "x")}),
        ] {
            let expression =
                json!({"kind": "invoke", "function": "type.Outer", "bindings": [], "body": nested});
            let error = evaluator
                .evaluate(&expression, "caller", &values)
                .unwrap_err()
                .to_string();
            assert!(error.contains("nonlocal lexical capture"), "{error}");
        }
        // An inner argument may still refer by identity to a bound outer parameter.
        let expression = json!({"kind": "invoke", "function": "type.Outer", "bindings": [
            {"feature": "feature.Outer.x", "expression": resolved_test_path("feature.Caller.bias", "bias")}
        ], "body": {"kind": "invoke", "function": "type.Inner", "bindings": [
            {"feature": "feature.Inner.x", "expression": resolved_test_path("feature.Outer.x", "x")}
        ], "body": resolved_test_path("feature.Inner.x", "x")}});
        assert_eq!(
            evaluator.evaluate(&expression, "caller", &values).unwrap(),
            json!(99)
        );
    }
}
