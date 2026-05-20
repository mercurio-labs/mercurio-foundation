use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ExpressionIr {
    Literal {
        value: Value,
    },
    #[serde(rename = "self")]
    SelfRef,
    Tuple {
        items: Vec<ExpressionIr>,
    },
    Path {
        root: ExpressionPathRoot,
        segments: Vec<ExpressionPathSegment>,
    },
    Unary {
        op: UnaryExpressionOp,
        expr: Box<ExpressionIr>,
    },
    Binary {
        left: Box<ExpressionIr>,
        op: BinaryExpressionOp,
        right: Box<ExpressionIr>,
    },
    Call {
        function: String,
        args: Vec<ExpressionIr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpressionPathRoot {
    #[serde(rename = "self")]
    SelfRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExpressionPathSegment {
    Resolved {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        feature: Option<String>,
    },
    Name(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnaryExpressionOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BinaryExpressionOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

impl ExpressionPathRoot {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SelfRef => "self",
        }
    }
}

impl ExpressionPathSegment {
    pub fn name(&self) -> &str {
        match self {
            Self::Resolved { name, .. } | Self::Name(name) => name,
        }
    }
}

#[derive(Debug)]
pub enum ExpressionIrError {
    MissingKind,
    UnsupportedKind(String),
    Invalid(String),
}

impl fmt::Display for ExpressionIrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingKind => write!(f, "invalid expression_ir missing string kind"),
            Self::UnsupportedKind(kind) => {
                write!(f, "unsupported expression_ir kind `{kind}`")
            }
            Self::Invalid(message) => write!(f, "invalid expression_ir: {message}"),
        }
    }
}

impl std::error::Error for ExpressionIrError {}

impl ExpressionIr {
    pub fn from_value(value: &Value) -> Result<Self, ExpressionIrError> {
        let kind = value
            .get("kind")
            .and_then(Value::as_str)
            .ok_or(ExpressionIrError::MissingKind)?;
        if !matches!(
            kind,
            "literal" | "self" | "tuple" | "path" | "unary" | "binary" | "call"
        ) {
            return Err(ExpressionIrError::UnsupportedKind(kind.to_string()));
        }

        serde_json::from_value(value.clone())
            .map_err(|err| ExpressionIrError::Invalid(err.to_string()))
    }

    pub fn to_value(&self) -> Result<Value, ExpressionIrError> {
        serde_json::to_value(self).map_err(|err| ExpressionIrError::Invalid(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        BinaryExpressionOp, ExpressionIr, ExpressionPathRoot, ExpressionPathSegment,
        UnaryExpressionOp,
    };

    #[test]
    fn serializes_expression_ir_contract_shapes() {
        let expression = ExpressionIr::Binary {
            left: Box::new(ExpressionIr::Call {
                function: "sum".to_string(),
                args: vec![ExpressionIr::Path {
                    root: ExpressionPathRoot::SelfRef,
                    segments: vec![
                        ExpressionPathSegment::Resolved {
                            name: "parts".to_string(),
                            feature: Some("feature.Demo.vehicle.parts".to_string()),
                        },
                        ExpressionPathSegment::Resolved {
                            name: "mass".to_string(),
                            feature: Some("feature.Demo.Engine.mass".to_string()),
                        },
                    ],
                }],
            }),
            op: BinaryExpressionOp::Greater,
            right: Box::new(ExpressionIr::Unary {
                op: UnaryExpressionOp::Negate,
                expr: Box::new(ExpressionIr::Literal { value: json!(0) }),
            }),
        };

        assert_eq!(
            serde_json::to_value(expression).unwrap(),
            json!({
                "kind": "binary",
                "op": "greater",
                "left": {
                    "kind": "call",
                    "function": "sum",
                    "args": [{
                        "kind": "path",
                        "root": "self",
                        "segments": [
                            {"name": "parts", "feature": "feature.Demo.vehicle.parts"},
                            {"name": "mass", "feature": "feature.Demo.Engine.mass"}
                        ]
                    }]
                },
                "right": {
                    "kind": "unary",
                    "op": "negate",
                    "expr": {"kind": "literal", "value": 0}
                }
            })
        );
    }

    #[test]
    fn deserializes_legacy_string_path_segments() {
        let expression: ExpressionIr = serde_json::from_value(json!({
            "kind": "path",
            "root": "self",
            "segments": ["parts", "mass"]
        }))
        .unwrap();

        let ExpressionIr::Path { segments, .. } = expression else {
            panic!("expected path expression");
        };
        assert_eq!(
            segments
                .iter()
                .map(ExpressionPathSegment::name)
                .collect::<Vec<_>>(),
            vec!["parts", "mass"]
        );
    }

    #[test]
    fn rejects_unknown_expression_ir_kind() {
        let error = ExpressionIr::from_value(&json!({
            "kind": "select",
            "source": {"kind": "self"}
        }))
        .unwrap_err();

        assert_eq!(error.to_string(), "unsupported expression_ir kind `select`");
    }
}
