//! Deterministic neutral operators share the same scalar/sequence evaluator as
//! standard function calls. Language-specific names remain in the frontends.
use super::{ExpressionIr, ExpressionValidationError};

pub(super) fn validate(
    operator: &str,
    operands: &[ExpressionIr],
) -> Result<(), ExpressionValidationError> {
    let signature = super::builtin_signature(operator)
        .ok_or_else(|| ExpressionValidationError::UnsupportedConstruct(operator.into()))?;
    if !signature.accepts_arity(operands.len()) {
        return Err(ExpressionValidationError::InvalidFunctionArity {
            function: operator.into(),
            arity: operands.len(),
        });
    }
    for operand in operands {
        operand.validate_runtime_supported()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::ExpressionPathSegment;
    use super::super::{ExpressionEvaluationContext, ExpressionEvaluationError};
    use super::*;
    use serde_json::Value;
    use serde_json::json;

    struct MissingContext;
    impl ExpressionEvaluationContext for MissingContext {
        fn owner_id(&self) -> &str {
            "test"
        }
        fn resolve_path(
            &mut self,
            _: &[ExpressionPathSegment],
        ) -> Result<Vec<Value>, ExpressionEvaluationError> {
            Err(ExpressionEvaluationError::MissingBinding("missing".into()))
        }
    }
    fn op(operator: &str, values: &[Value]) -> ExpressionIr {
        ExpressionIr::Operation {
            operator: operator.into(),
            operands: values
                .iter()
                .map(|value| match value {
                    Value::Array(values) => ExpressionIr::Tuple {
                        items: values
                            .iter()
                            .map(|value| ExpressionIr::Literal {
                                value: value.clone(),
                            })
                            .collect(),
                    },
                    value => ExpressionIr::Literal {
                        value: value.clone(),
                    },
                })
                .collect(),
        }
    }
    fn missing() -> ExpressionIr {
        ExpressionIr::from_value(&json!({"kind":"path","segments":["missing"]})).unwrap()
    }
    #[test]
    fn control_operations_evaluate_only_selected_branches() {
        for (operator, operands, expected) in [
            (
                "if",
                vec![
                    ExpressionIr::Literal { value: json!(true) },
                    ExpressionIr::Literal { value: json!(7) },
                    missing(),
                ],
                json!(7),
            ),
            (
                "if",
                vec![
                    ExpressionIr::Literal {
                        value: json!(false),
                    },
                    missing(),
                    ExpressionIr::Literal { value: json!(8) },
                ],
                json!(8),
            ),
            (
                "??",
                vec![
                    ExpressionIr::Literal {
                        value: json!(false),
                    },
                    missing(),
                ],
                json!(false),
            ),
            (
                "implies",
                vec![
                    ExpressionIr::Literal {
                        value: json!(false),
                    },
                    missing(),
                ],
                json!(true),
            ),
        ] {
            let expression = ExpressionIr::Operation {
                operator: operator.into(),
                operands,
            };
            assert_eq!(expression.evaluate(&mut MissingContext).unwrap(), expected);
        }
        for first in [Value::Null, json!([])] {
            assert_eq!(
                op("??", &[first, json!(9)])
                    .evaluate(&mut MissingContext)
                    .unwrap(),
                json!(9)
            );
        }
        for operator in ["&", "|", "xor"] {
            let expression = ExpressionIr::Operation {
                operator: operator.into(),
                operands: vec![
                    ExpressionIr::Literal {
                        value: json!(false),
                    },
                    missing(),
                ],
            };
            assert!(matches!(
                expression.evaluate(&mut MissingContext),
                Err(ExpressionEvaluationError::MissingBinding(_))
            ));
        }
    }
    #[test]
    fn sequence_indexing_preserves_one_based_order_and_empty_results() {
        for (index, expected) in [
            (1, json!(11)),
            (2, json!(22)),
            (0, json!([])),
            (-1, json!([])),
            (3, json!([])),
        ] {
            assert_eq!(
                op("#", &[json!([11, 22]), json!(index)])
                    .evaluate(&mut MissingContext)
                    .unwrap(),
                expected
            );
        }
        assert_eq!(
            op("#", &[json!(7), json!(1)])
                .evaluate(&mut MissingContext)
                .unwrap(),
            json!(7)
        );
        assert_eq!(
            op("#", &[Value::Null, json!(1)])
                .evaluate(&mut MissingContext)
                .unwrap(),
            json!([])
        );
        assert!(
            op("#", &[json!([11]), json!(1.5)])
                .evaluate(&mut MissingContext)
                .is_err()
        );
    }
    #[test]
    fn operations_reject_bad_arity_types_and_unresolved_selected_values() {
        for expression in [
            op("if", &[json!(true)]),
            op("xor", &[json!(true), json!(1)]),
            op("implies", &[json!(1), json!(false)]),
            op("if", &[json!(1), json!(2), json!(3)]),
        ] {
            assert!(expression.evaluate(&mut MissingContext).is_err());
        }
        for (operator, first) in [("??", Value::Null), ("implies", json!(true))] {
            let expression = ExpressionIr::Operation {
                operator: operator.into(),
                operands: vec![ExpressionIr::Literal { value: first }, missing()],
            };
            assert!(matches!(
                expression.evaluate(&mut MissingContext),
                Err(ExpressionEvaluationError::MissingBinding(_))
            ));
        }
        for operator in ["&", "|", "xor", "implies"] {
            for first in [false, true] {
                for second in [false, true] {
                    let expected = match operator {
                        "&" => first & second,
                        "|" => first | second,
                        "xor" => first ^ second,
                        _ => !first || second,
                    };
                    assert_eq!(
                        op(operator, &[json!(first), json!(second)])
                            .evaluate(&mut MissingContext)
                            .unwrap(),
                        json!(expected)
                    );
                }
            }
        }
    }
}
