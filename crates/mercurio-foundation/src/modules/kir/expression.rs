use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

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
        #[serde(alias = "operand")]
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
    /// A resolved, bounded call with bindings ordered by dependency.
    Invoke {
        function: String,
        bindings: Vec<ExpressionBinding>,
        body: Box<ExpressionIr>,
    },
    /// A declaration or invocation boundary checked by the shared evaluator.
    Checked {
        expression: Box<ExpressionIr>,
        contract: ExpressionContract,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpressionBinding {
    /// Resolved feature identity; names never participate in frame lookup.
    pub feature: String,
    pub expression: ExpressionIr,
    /// Definition defaults and locals use lexical lookup. Caller actual arguments
    /// use the caller context. Missing flags retain the earlier argument contract.
    #[serde(default)]
    pub lexical: bool,
}

/// An expression result sequence, distinct from JSON array data values.
///
/// An empty result is `values: []`; one array value is `values: [[...]]`.
/// The legacy evaluator intentionally collapses this distinction on output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpressionResult {
    pub values: Vec<Value>,
}

impl ExpressionResult {
    pub fn cardinality(&self) -> u64 {
        self.values.len() as u64
    }

    pub fn into_legacy_value(self) -> Value {
        result_value(self.values)
    }
}

/// Supported scalar contract domains. `Any` also accepts opaque JSON data values;
/// it does not imply an implementation of collection or object type semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpressionValueType {
    Any,
    Boolean,
    String,
    Integer,
    Real,
    Natural,
    Positive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpressionValueKind {
    Null,
    Boolean,
    String,
    Integer,
    Real,
    Array,
    Object,
}

impl ExpressionValueKind {
    fn of(value: &Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::Bool(_) => Self::Boolean,
            Value::String(_) => Self::String,
            Value::Number(number) if number.is_i64() || number.is_u64() => Self::Integer,
            Value::Number(_) => Self::Real,
            Value::Array(_) => Self::Array,
            Value::Object(_) => Self::Object,
        }
    }
}

impl ExpressionValueType {
    fn accepts(self, value: &Value) -> bool {
        match self {
            Self::Any => true,
            Self::Boolean => value.is_boolean(),
            Self::String => value.is_string(),
            Self::Integer => value.is_i64() || value.is_u64(),
            Self::Real => value.is_number(),
            Self::Natural => value.as_u64().is_some(),
            Self::Positive => value.as_u64().is_some_and(|number| number > 0),
        }
    }
}

/// Inclusive, literal cardinality bounds. `upper: None` is unlimited.
/// General bound expressions must be resolved before constructing this contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionMultiplicity {
    pub lower: u64,
    pub upper: Option<u64>,
}

impl ExpressionMultiplicity {
    /// Parse the bounded executable profile: nonnegative integer literals and
    /// an upper `*`. Unsupported expressions produce an explicit diagnostic.
    pub fn from_bounds(lower: &str, upper: &str) -> Result<Self, ExpressionContractError> {
        fn bound(text: &str) -> Result<u64, ExpressionContractError> {
            let text = text.trim();
            if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(ExpressionContractError::UnsupportedMultiplicityBound {
                    bound: text.to_string(),
                });
            }
            text.parse()
                .map_err(|_| ExpressionContractError::UnsupportedMultiplicityBound {
                    bound: text.to_string(),
                })
        }
        let multiplicity = Self {
            lower: bound(lower)?,
            upper: if upper.trim() == "*" {
                None
            } else {
                Some(bound(upper)?)
            },
        };
        multiplicity.validate()?;
        Ok(multiplicity)
    }

    pub fn validate(&self) -> Result<(), ExpressionContractError> {
        if let Some(upper) = self.upper {
            if upper < self.lower {
                return Err(ExpressionContractError::InvalidMultiplicityBounds {
                    lower: self.lower,
                    upper,
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionContract {
    pub value_type: ExpressionValueType,
    /// None means unspecified, not an implicit singleton constraint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiplicity: Option<ExpressionMultiplicity>,
}

impl ExpressionContract {
    pub fn validate(&self) -> Result<(), ExpressionContractError> {
        if let Some(multiplicity) = self.multiplicity {
            multiplicity.validate()?;
        }
        Ok(())
    }

    pub fn check(&self, result: &ExpressionResult) -> Result<(), ExpressionContractError> {
        self.check_values(&result.values)
    }

    fn check_values(&self, values: &[Value]) -> Result<(), ExpressionContractError> {
        self.validate()?;
        let actual = values.len() as u64;
        if let Some(multiplicity) = self.multiplicity {
            if actual < multiplicity.lower || multiplicity.upper.is_some_and(|upper| actual > upper)
            {
                return Err(ExpressionContractError::MultiplicityMismatch {
                    lower: multiplicity.lower,
                    upper: multiplicity.upper,
                    actual,
                });
            }
        }
        for (index, value) in values.iter().enumerate() {
            if !self.value_type.accepts(value) {
                return Err(ExpressionContractError::TypeMismatch {
                    expected: self.value_type,
                    actual: ExpressionValueKind::of(value),
                    result_index: index as u64 + 1,
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum ExpressionContractError {
    UnsupportedMultiplicityBound {
        bound: String,
    },
    InvalidMultiplicityBounds {
        lower: u64,
        upper: u64,
    },
    MultiplicityMismatch {
        lower: u64,
        upper: Option<u64>,
        actual: u64,
    },
    TypeMismatch {
        expected: ExpressionValueType,
        actual: ExpressionValueKind,
        /// One-based index in the expression result sequence.
        result_index: u64,
    },
}

impl fmt::Display for ExpressionContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedMultiplicityBound { bound } => {
                write!(
                    f,
                    "unsupported multiplicity bound `{bound}`; expected a nonnegative integer literal or upper *"
                )
            }
            Self::InvalidMultiplicityBounds { lower, upper } => {
                write!(
                    f,
                    "invalid multiplicity bounds: lower {lower} exceeds upper {upper}"
                )
            }
            Self::MultiplicityMismatch {
                lower,
                upper,
                actual,
            } => {
                let upper = upper
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "*".to_string());
                write!(
                    f,
                    "expression cardinality {actual} violates multiplicity {lower}..{upper}"
                )
            }
            Self::TypeMismatch {
                expected,
                actual,
                result_index,
            } => {
                write!(
                    f,
                    "expression result {result_index} has type {actual:?}, expected {expected:?}"
                )
            }
        }
    }
}

impl std::error::Error for ExpressionContractError {}

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
    #[serde(alias = "-")]
    Negate,
    #[serde(alias = "!")]
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BinaryExpressionOp {
    #[serde(alias = "+", alias = "plus")]
    Add,
    #[serde(alias = "-", alias = "sub", alias = "minus")]
    Subtract,
    #[serde(alias = "*", alias = "mul")]
    Multiply,
    #[serde(alias = "/", alias = "div")]
    Divide,
    #[serde(alias = "^", alias = "**")]
    Power,
    #[serde(alias = "==")]
    Equal,
    #[serde(alias = "!=")]
    NotEqual,
    #[serde(alias = "<")]
    Less,
    #[serde(alias = "<=")]
    LessEqual,
    #[serde(alias = ">")]
    Greater,
    #[serde(alias = ">=")]
    GreaterEqual,
    #[serde(alias = "&&")]
    And,
    #[serde(alias = "||")]
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

#[derive(Debug)]
pub enum ExpressionValidationError {
    InvalidContract(ExpressionContractError),
    DuplicateBinding { function: String, feature: String },
    UnsupportedPathRoot(String),
    EmptyPath,
    UnsupportedFunction(String),
    InvalidFunctionArity { function: String, arity: usize },
}

impl fmt::Display for ExpressionValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidContract(error) => write!(f, "{error}"),
            Self::DuplicateBinding { function, feature } => {
                write!(
                    f,
                    "duplicate binding `{feature}` in invocation `{function}`"
                )
            }
            Self::UnsupportedPathRoot(root) => {
                write!(f, "unsupported expression_ir path root `{root}`")
            }
            Self::EmptyPath => write!(f, "expression_ir path must have at least one segment"),
            Self::UnsupportedFunction(function) => {
                write!(f, "unsupported expression_ir function `{function}`")
            }
            Self::InvalidFunctionArity { function, arity } => {
                let expected = builtin_signature(function)
                    .map(|signature| signature.arity_description())
                    .unwrap_or("a supported number of arguments");
                write!(
                    f,
                    "expression_ir function `{function}` expects {expected}, got {arity}"
                )
            }
        }
    }
}

impl std::error::Error for ExpressionValidationError {}

impl ExpressionIr {
    pub fn from_value(value: &Value) -> Result<Self, ExpressionIrError> {
        let kind = value
            .get("kind")
            .and_then(Value::as_str)
            .ok_or(ExpressionIrError::MissingKind)?;
        if !matches!(
            kind,
            "literal"
                | "self"
                | "tuple"
                | "path"
                | "unary"
                | "binary"
                | "call"
                | "checked"
                | "invoke"
        ) {
            return Err(ExpressionIrError::UnsupportedKind(kind.to_string()));
        }

        serde_json::from_value(normalize_expression_ir_value(value))
            .map_err(|err| ExpressionIrError::Invalid(err.to_string()))
    }

    pub fn to_value(&self) -> Result<Value, ExpressionIrError> {
        serde_json::to_value(self).map_err(|err| ExpressionIrError::Invalid(err.to_string()))
    }

    pub fn validate_runtime_supported(&self) -> Result<(), ExpressionValidationError> {
        match self {
            Self::Literal { .. } | Self::SelfRef => Ok(()),
            Self::Invoke {
                function,
                bindings,
                body,
            } => {
                let mut features = BTreeSet::new();
                for binding in bindings {
                    if !features.insert(&binding.feature) {
                        return Err(ExpressionValidationError::DuplicateBinding {
                            function: function.clone(),
                            feature: binding.feature.clone(),
                        });
                    }
                    binding.expression.validate_runtime_supported()?;
                }
                body.validate_runtime_supported()
            }
            Self::Checked {
                expression,
                contract,
            } => {
                contract
                    .validate()
                    .map_err(ExpressionValidationError::InvalidContract)?;
                expression.validate_runtime_supported()
            }
            Self::Tuple { items } => {
                for item in items {
                    item.validate_runtime_supported()?;
                }
                Ok(())
            }
            Self::Path { root, segments } => {
                if *root != ExpressionPathRoot::SelfRef {
                    return Err(ExpressionValidationError::UnsupportedPathRoot(
                        root.as_str().to_string(),
                    ));
                }
                if segments.is_empty() || segments.iter().any(|segment| segment.name().is_empty()) {
                    return Err(ExpressionValidationError::EmptyPath);
                }
                Ok(())
            }
            Self::Unary { expr, .. } => expr.validate_runtime_supported(),
            Self::Binary { left, right, .. } => {
                left.validate_runtime_supported()?;
                right.validate_runtime_supported()
            }
            Self::Call { function, args } => {
                let signature = builtin_signature(function).ok_or_else(|| {
                    ExpressionValidationError::UnsupportedFunction(function.clone())
                })?;
                if !signature.accepts_arity(args.len()) {
                    return Err(ExpressionValidationError::InvalidFunctionArity {
                        function: function.clone(),
                        arity: args.len(),
                    });
                }
                for arg in args {
                    arg.validate_runtime_supported()?;
                }
                Ok(())
            }
        }
    }

    pub fn render_constraint_expression(&self) -> String {
        match self {
            Self::Literal { value } => render_literal_value(value),
            Self::Checked { expression, .. } => expression.render_constraint_expression(),
            Self::Invoke {
                function, bindings, ..
            } => format!(
                "{}({})",
                function,
                bindings
                    .iter()
                    .map(|binding| binding.expression.render_constraint_expression())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::SelfRef => "self".to_string(),
            Self::Tuple { items } => format!(
                "({})",
                items
                    .iter()
                    .map(Self::render_constraint_expression)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Path { segments, .. } => render_path_segments(segments),
            Self::Unary { op, expr } => match op {
                UnaryExpressionOp::Negate => format!("-{}", expr.render_constraint_expression()),
                UnaryExpressionOp::Not => format!("not {}", expr.render_constraint_expression()),
            },
            Self::Binary { left, op, right } => format!(
                "({} {} {})",
                left.render_constraint_expression(),
                op.constraint_symbol(),
                right.render_constraint_expression()
            ),
            Self::Call { function, args } => {
                let rendered = args
                    .iter()
                    .map(Self::render_constraint_expression)
                    .collect::<Vec<_>>();
                match (function.as_str(), rendered.as_slice()) {
                    ("if", [test, yes, no]) => format!("(if {test} ? {yes} else {no})"),
                    ("#", [sequence, index]) => format!("({sequence})#({index})"),
                    ("+" | "-", [operand]) => format!("{function}({operand})"),
                    (
                        "+" | "-" | "%" | ".." | "&" | "|" | "xor" | "implies" | "??",
                        [left, right],
                    ) => {
                        format!("({left} {function} {right})")
                    }
                    _ => format!("{}({})", function.replace('.', "::"), rendered.join(", ")),
                }
            }
        }
    }

    pub fn collect_path_variables(&self, output: &mut BTreeSet<String>) {
        match self {
            Self::Path { segments, .. } => {
                output.insert(render_path_segments(segments));
            }
            Self::Tuple { items } => {
                for item in items {
                    item.collect_path_variables(output);
                }
            }
            Self::Unary { expr, .. } => expr.collect_path_variables(output),
            Self::Checked { expression, .. } => expression.collect_path_variables(output),
            Self::Invoke { bindings, body, .. } => {
                for binding in bindings {
                    binding.expression.collect_path_variables(output);
                }
                body.collect_path_variables(output);
            }
            Self::Binary { left, right, .. } => {
                left.collect_path_variables(output);
                right.collect_path_variables(output);
            }
            Self::Call { args, .. } => {
                for arg in args {
                    arg.collect_path_variables(output);
                }
            }
            Self::Literal { .. } | Self::SelfRef => {}
        }
    }

    pub fn evaluate(
        &self,
        context: &mut impl ExpressionEvaluationContext,
    ) -> Result<Value, ExpressionEvaluationError> {
        self.evaluate_result(context)
            .map(ExpressionResult::into_legacy_value)
    }

    /// Evaluate without collapsing a result sequence into its legacy JSON shape.
    pub fn evaluate_result(
        &self,
        context: &mut impl ExpressionEvaluationContext,
    ) -> Result<ExpressionResult, ExpressionEvaluationError> {
        self.check_evaluation_budget()?;
        // Validate every branch, but evaluate only the branches selected by control operators.
        self.validate_runtime_supported()
            .map_err(|error| match error {
                ExpressionValidationError::InvalidContract(error) => {
                    ExpressionEvaluationError::ContractViolation(error)
                }
                ExpressionValidationError::DuplicateBinding { function, feature } => {
                    ExpressionEvaluationError::DuplicateBinding { function, feature }
                }
                error => ExpressionEvaluationError::InvalidExpression(error.to_string()),
            })?;
        self.evaluate_values(context)
            .map(|values| ExpressionResult { values })
    }

    // Preflight is iterative so malformed externally supplied trees cannot exhaust
    // the call stack before deterministic evaluation limits are checked.
    fn check_evaluation_budget(&self) -> Result<(), ExpressionEvaluationError> {
        let mut pending = vec![(self, 1usize, 0usize)];
        let mut nodes = 0usize;
        while let Some((expression, depth, invocation_depth)) = pending.pop() {
            nodes += 1;
            let invocation_depth =
                invocation_depth + usize::from(matches!(expression, Self::Invoke { .. }));
            for (resource, actual, limit) in [
                ("expression_nodes", nodes, 10_000),
                ("expression_depth", depth, 256),
                ("invocation_depth", invocation_depth, 64),
            ] {
                if actual > limit {
                    return Err(ExpressionEvaluationError::ResourceLimitExceeded {
                        resource: resource.to_string(),
                        limit,
                    });
                }
            }
            let mut push = |child| pending.push((child, depth + 1, invocation_depth));
            match expression {
                Self::Literal { .. } | Self::SelfRef | Self::Path { .. } => {}
                Self::Tuple { items } | Self::Call { args: items, .. } => {
                    for item in items {
                        push(item);
                    }
                }
                Self::Unary { expr, .. }
                | Self::Checked {
                    expression: expr, ..
                } => push(expr),
                Self::Binary { left, right, .. } => {
                    push(left);
                    push(right);
                }
                Self::Invoke { bindings, body, .. } => {
                    for binding in bindings {
                        push(&binding.expression);
                    }
                    push(body);
                }
            }
        }
        Ok(())
    }

    // Result sequences are separate from JSON data values: a literal array is one value,
    // whereas Tuple concatenates the results of its operands. Null denotes no result.
    fn evaluate_values(
        &self,
        context: &mut dyn ExpressionEvaluationContext,
    ) -> Result<Vec<Value>, ExpressionEvaluationError> {
        let values = self.evaluate_values_unchecked(context)?;
        check_result_capacity(0, values.len(), "result_values")?;
        Ok(values)
    }

    fn evaluate_values_unchecked(
        &self,
        context: &mut dyn ExpressionEvaluationContext,
    ) -> Result<Vec<Value>, ExpressionEvaluationError> {
        match self {
            Self::Invoke {
                function,
                bindings,
                body,
            } => {
                let mut frame = InvocationEvaluationContext {
                    outer: context,
                    function,
                    declared: bindings
                        .iter()
                        .map(|binding| binding.feature.clone())
                        .collect(),
                    bindings: BTreeMap::new(),
                    lexical: false,
                };
                let mut frame_values = 0;
                for binding in bindings {
                    frame.lexical = binding.lexical;
                    let values = binding.expression.evaluate_values(&mut frame)?;
                    frame_values =
                        check_result_capacity(frame_values, values.len(), "frame_values")?;
                    frame.bindings.insert(binding.feature.clone(), values);
                }
                frame.lexical = true;
                body.evaluate_values(&mut frame)
            }
            Self::Checked {
                expression,
                contract,
            } => {
                let values = expression.evaluate_values(context)?;
                contract
                    .check_values(&values)
                    .map_err(ExpressionEvaluationError::ContractViolation)?;
                Ok(values)
            }
            Self::Literal { value: Value::Null } => Ok(Vec::new()),
            Self::Literal { value } => Ok(vec![value.clone()]),
            Self::SelfRef => Ok(vec![Value::String(context.owner_id().to_string())]),
            Self::Tuple { items } => {
                let mut values = Vec::new();
                for item in items {
                    let item_values = item.evaluate_values(context)?;
                    check_result_capacity(values.len(), item_values.len(), "result_values")?;
                    values.extend(item_values);
                }
                Ok(values)
            }
            Self::Path { segments, .. } => context.resolve_path(segments),
            Self::Unary { op, expr } => {
                let value = expr.evaluate_scalar(context)?;
                Ok(vec![match op {
                    UnaryExpressionOp::Negate => negate_number(&value, self)?,
                    UnaryExpressionOp::Not => Value::Bool(!value_as_bool(&value, self)?),
                }])
            }
            Self::Binary { left, op, right } => {
                if matches!(op, BinaryExpressionOp::Equal | BinaryExpressionOp::NotEqual) {
                    let left = left.evaluate_values(context)?;
                    let right = right.evaluate_values(context)?;
                    if left.len() > 1 || right.len() > 1 {
                        return Err(invalid(
                            "equality operands must each have zero or one result",
                        ));
                    }
                    let equal = sequence_equal(&left, &right);
                    return Ok(vec![Value::Bool(if *op == BinaryExpressionOp::Equal {
                        equal
                    } else {
                        !equal
                    })]);
                }
                let left = left.evaluate_scalar(context)?;
                if (*op == BinaryExpressionOp::And && !value_as_bool(&left, self)?)
                    || (*op == BinaryExpressionOp::Or && value_as_bool(&left, self)?)
                {
                    return Ok(vec![left]);
                }
                let right = right.evaluate_scalar(context)?;
                Ok(vec![evaluate_binary_expression(*op, &left, &right, self)?])
            }
            Self::Call { function, args } => evaluate_call(function, args, self, context),
        }
    }

    fn evaluate_scalar(
        &self,
        context: &mut dyn ExpressionEvaluationContext,
    ) -> Result<Value, ExpressionEvaluationError> {
        let mut values = self.evaluate_values(context)?;
        if values.len() != 1 {
            return Err(invalid(format!(
                "expected one expression result, got {}",
                values.len()
            )));
        }
        values
            .pop()
            .ok_or_else(|| invalid("missing expression result"))
    }
}

impl BinaryExpressionOp {
    fn constraint_symbol(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Power => "^",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::And => "and",
            Self::Or => "or",
        }
    }
}

fn normalize_expression_ir_value(value: &Value) -> Value {
    let Some(object) = value.as_object() else {
        return value.clone();
    };

    let mut normalized = object.clone();
    match normalized.get("kind").and_then(Value::as_str) {
        Some("path") => {
            normalized
                .entry("root".to_string())
                .or_insert_with(|| Value::String("self".to_string()));
        }
        Some("tuple") => normalize_array_field(&mut normalized, "items"),
        Some("checked") => normalize_object_field(&mut normalized, "expression"),
        Some("invoke") => {
            normalize_object_field(&mut normalized, "body");
            if let Some(bindings) = normalized.get_mut("bindings").and_then(Value::as_array_mut) {
                for binding in bindings {
                    if let Some(binding) = binding.as_object_mut() {
                        normalize_object_field(binding, "expression");
                    }
                }
            }
        }
        Some("unary") => {
            normalize_object_field(&mut normalized, "expr");
            normalize_object_field(&mut normalized, "operand");
        }
        Some("binary") => {
            normalize_object_field(&mut normalized, "left");
            normalize_object_field(&mut normalized, "right");
        }
        Some("call") => normalize_array_field(&mut normalized, "args"),
        _ => {}
    }
    Value::Object(normalized)
}

fn normalize_object_field(object: &mut serde_json::Map<String, Value>, field: &str) {
    if let Some(value) = object.get(field).cloned() {
        object.insert(field.to_string(), normalize_expression_ir_value(&value));
    }
}

fn normalize_array_field(object: &mut serde_json::Map<String, Value>, field: &str) {
    if let Some(values) = object.get(field).and_then(Value::as_array) {
        object.insert(
            field.to_string(),
            Value::Array(values.iter().map(normalize_expression_ir_value).collect()),
        );
    }
}

fn render_literal_value(value: &Value) -> String {
    // Preserve string quoting so a value cannot turn into a feature reference when rendered.
    value.to_string()
}

fn render_path_segments(segments: &[ExpressionPathSegment]) -> String {
    segments
        .iter()
        .map(ExpressionPathSegment::name)
        .collect::<Vec<_>>()
        .join(".")
}

pub trait ExpressionEvaluationContext {
    fn owner_id(&self) -> &str;

    fn resolve_path(
        &mut self,
        segments: &[ExpressionPathSegment],
    ) -> Result<Vec<Value>, ExpressionEvaluationError>;

    /// Resolve a reference originating in a callable definition. Contexts without
    /// a lexical model environment must reject unsupported nonlocal captures.
    fn resolve_lexical_path(
        &mut self,
        segments: &[ExpressionPathSegment],
    ) -> Result<Vec<Value>, ExpressionEvaluationError> {
        self.resolve_path(segments)
    }
}

struct InvocationEvaluationContext<'a> {
    outer: &'a mut dyn ExpressionEvaluationContext,
    function: &'a str,
    declared: BTreeSet<String>,
    bindings: BTreeMap<String, Vec<Value>>,
    lexical: bool,
}

impl ExpressionEvaluationContext for InvocationEvaluationContext<'_> {
    fn owner_id(&self) -> &str {
        self.outer.owner_id()
    }

    fn resolve_path(
        &mut self,
        segments: &[ExpressionPathSegment],
    ) -> Result<Vec<Value>, ExpressionEvaluationError> {
        self.resolve_frame_path(segments, self.lexical)
    }

    fn resolve_lexical_path(
        &mut self,
        segments: &[ExpressionPathSegment],
    ) -> Result<Vec<Value>, ExpressionEvaluationError> {
        self.resolve_frame_path(segments, true)
    }
}

impl InvocationEvaluationContext<'_> {
    fn resolve_frame_path(
        &mut self,
        segments: &[ExpressionPathSegment],
        lexical: bool,
    ) -> Result<Vec<Value>, ExpressionEvaluationError> {
        if let Some(ExpressionPathSegment::Resolved {
            feature: Some(feature),
            ..
        }) = segments.first()
        {
            if let Some(values) = self.bindings.get(feature) {
                if segments.len() != 1 {
                    return Err(invalid(format!(
                        "navigation through parameter `{feature}` in invocation `{}` is not supported",
                        self.function
                    )));
                }
                return Ok(values.clone());
            }
            if self.declared.contains(feature) {
                return Err(ExpressionEvaluationError::UnboundParameter {
                    function: self.function.to_string(),
                    feature: feature.clone(),
                });
            }
        }
        if lexical {
            self.outer.resolve_lexical_path(segments)
        } else {
            self.outer.resolve_path(segments)
        }
    }
}

#[derive(Debug)]
pub enum ExpressionEvaluationError {
    ContractViolation(ExpressionContractError),
    DuplicateBinding {
        function: String,
        feature: String,
    },
    UnboundParameter {
        function: String,
        feature: String,
    },
    ResourceLimitExceeded {
        resource: String,
        limit: usize,
    },
    InvalidExpression(String),
    MissingBinding(String),
    DivisionByZero,
    NonFiniteResult,
    UnsupportedAggregation {
        expression: String,
    },
    UnsupportedFunction {
        function: String,
        expression: String,
    },
    NonNumericValue {
        owner: String,
        feature: String,
    },
}

impl fmt::Display for ExpressionEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContractViolation(error) => write!(f, "{error}"),
            Self::DuplicateBinding { function, feature } => write!(
                f,
                "duplicate binding `{feature}` in invocation `{function}`"
            ),
            Self::UnboundParameter { function, feature } => write!(
                f,
                "unbound or cyclic parameter `{feature}` in invocation `{function}`"
            ),
            Self::ResourceLimitExceeded { resource, limit } => write!(
                f,
                "expression resource limit `{resource}` exceeded ({limit})"
            ),
            Self::InvalidExpression(expression) => write!(f, "invalid expression: {expression}"),
            Self::MissingBinding(path) => write!(f, "unresolved expression path: {path}"),
            Self::DivisionByZero => write!(f, "division by zero"),
            Self::NonFiniteResult => write!(f, "non-finite expression result"),
            Self::UnsupportedAggregation { expression } => {
                write!(f, "unsupported aggregation expression: {expression}")
            }
            Self::UnsupportedFunction {
                function,
                expression,
            } => write!(
                f,
                "unsupported expression_ir function `{function}`: {expression}"
            ),
            Self::NonNumericValue { owner, feature } => {
                write!(
                    f,
                    "non-numeric value encountered while reading {feature} from {owner}"
                )
            }
        }
    }
}

impl std::error::Error for ExpressionEvaluationError {}

// These are bounded evaluator intrinsics, not arbitrary user-function resolution.
// Qualified names are accepted only for explicit standard-library packages.
#[derive(Clone, Copy)]
enum Builtin {
    Count,
    Sum,
    Min,
    Max,
    Avg,
    Plus,
    Minus,
    Binary(BinaryExpressionOp),
    Remainder,
    Range,
    Index,
    EagerAnd,
    EagerOr,
    Xor,
    Implies,
    Coalesce,
    If,
}

struct BuiltinSignature {
    operation: Builtin,
    minimum_arity: usize,
    maximum_arity: usize,
}

impl BuiltinSignature {
    fn accepts_arity(&self, arity: usize) -> bool {
        (self.minimum_arity..=self.maximum_arity).contains(&arity)
    }

    fn arity_description(&self) -> &'static str {
        match (self.minimum_arity, self.maximum_arity) {
            (1, 1) => "one argument",
            (2, 2) => "two arguments",
            (3, 3) => "three arguments",
            (1, 2) => "one or two arguments",
            _ => "a supported number of arguments",
        }
    }
}

fn builtin_signature(function: &str) -> Option<BuiltinSignature> {
    let qualified = function.split_once("::").or_else(|| {
        // The textual frontend normalizes qualified names with a dot separator.
        (!function.starts_with('.'))
            .then(|| function.split_once('.'))
            .flatten()
    });
    let (package, name) = qualified.unwrap_or(("", function));
    let name = name
        .strip_prefix('\'')
        .and_then(|name| name.strip_suffix('\''))
        .unwrap_or(name);
    let numeric_package = matches!(
        package,
        "ScalarFunctions"
            | "NumericalFunctions"
            | "RealFunctions"
            | "RationalFunctions"
            | "IntegerFunctions"
    );
    let (operation, minimum_arity, maximum_arity) = match (package, name) {
        ("", "count" | "size") | ("SequenceFunctions", "size") => (Builtin::Count, 1, 1),
        ("", "sum") => (Builtin::Sum, 1, 1),
        (_, "sum") if numeric_package && package != "ScalarFunctions" => (Builtin::Sum, 1, 1),
        ("", "avg") => (Builtin::Avg, 1, 1),
        // One-sequence min/max are retained as project compatibility functions.
        ("", "min") => (Builtin::Min, 1, 2),
        ("", "max") => (Builtin::Max, 1, 2),
        (_, "min") if numeric_package => (Builtin::Min, 2, 2),
        (_, "max") if numeric_package => (Builtin::Max, 2, 2),
        ("", "+") => (Builtin::Plus, 1, 2),
        ("", "-") => (Builtin::Minus, 1, 2),
        ("", "%") => (Builtin::Remainder, 2, 2),
        ("", "..") => (Builtin::Range, 2, 2),
        ("" | "SequenceFunctions" | "BaseFunctions", "#") => (Builtin::Index, 2, 2),
        ("" | "BooleanFunctions", "&") => (Builtin::EagerAnd, 2, 2),
        ("" | "BooleanFunctions", "|") => (Builtin::EagerOr, 2, 2),
        ("" | "BooleanFunctions", "xor") => (Builtin::Xor, 2, 2),
        ("" | "ControlFunctions", "and") => (Builtin::Binary(BinaryExpressionOp::And), 2, 2),
        ("" | "ControlFunctions", "or") => (Builtin::Binary(BinaryExpressionOp::Or), 2, 2),
        ("" | "ControlFunctions", "implies") => (Builtin::Implies, 2, 2),
        ("" | "ControlFunctions", "??") => (Builtin::Coalesce, 2, 2),
        ("" | "ControlFunctions", "if") => (Builtin::If, 3, 3),
        _ => return None,
    };
    Some(BuiltinSignature {
        operation,
        minimum_arity,
        maximum_arity,
    })
}

fn evaluate_call(
    function: &str,
    args: &[ExpressionIr],
    expression: &ExpressionIr,
    context: &mut dyn ExpressionEvaluationContext,
) -> Result<Vec<Value>, ExpressionEvaluationError> {
    let signature = builtin_signature(function).ok_or_else(|| {
        ExpressionEvaluationError::UnsupportedFunction {
            function: function.to_string(),
            expression: format!("{expression:?}"),
        }
    })?;
    if !signature.accepts_arity(args.len()) {
        return Err(invalid(format!(
            "function `{function}` expects {}",
            signature.arity_description()
        )));
    }
    match signature.operation {
        Builtin::If => {
            let test = args[0].evaluate_scalar(context)?;
            return args[if value_as_bool(&test, expression)? {
                1
            } else {
                2
            }]
            .evaluate_values(context);
        }
        Builtin::Coalesce => {
            let first = args[0].evaluate_values(context)?;
            return if first.is_empty() {
                args[1].evaluate_values(context)
            } else {
                Ok(first)
            };
        }
        Builtin::Implies => {
            let first = args[0].evaluate_scalar(context)?;
            if !value_as_bool(&first, expression)? {
                return Ok(vec![Value::Bool(true)]);
            }
            let second = args[1].evaluate_scalar(context)?;
            return Ok(vec![Value::Bool(value_as_bool(&second, expression)?)]);
        }
        Builtin::Binary(op @ (BinaryExpressionOp::And | BinaryExpressionOp::Or)) => {
            let first = args[0].evaluate_scalar(context)?;
            let boolean = value_as_bool(&first, expression)?;
            if (op == BinaryExpressionOp::And && !boolean)
                || (op == BinaryExpressionOp::Or && boolean)
            {
                return Ok(vec![first]);
            }
            let second = args[1].evaluate_scalar(context)?;
            return Ok(vec![Value::Bool(value_as_bool(&second, expression)?)]);
        }
        Builtin::Index => {
            let sequence = args[0].evaluate_values(context)?;
            let index = args[1].evaluate_scalar(context)?;
            let index = integer_value(&index)
                .filter(|index| *index > 0)
                .ok_or_else(|| invalid("sequence index must be a positive integer"))?;
            return Ok(usize::try_from(index - 1)
                .ok()
                .and_then(|index| sequence.get(index))
                .cloned()
                .into_iter()
                .collect());
        }
        Builtin::Count | Builtin::Sum | Builtin::Min | Builtin::Max | Builtin::Avg => {
            let mut values = args[0].evaluate_values(context)?;
            if args.len() == 2 {
                if values.len() != 1 {
                    return Err(invalid("binary min/max expects two scalar arguments"));
                }
                values.push(args[1].evaluate_scalar(context)?);
            }
            if (function.starts_with("IntegerFunctions::")
                || function.starts_with("IntegerFunctions."))
                && values.iter().any(|value| integer_value(value).is_none())
            {
                return Err(invalid("IntegerFunctions requires Integer arguments"));
            }
            let result = match signature.operation {
                Builtin::Count => Value::Number(Number::from(values.len() as u64)),
                Builtin::Sum => sum_numbers(&values, context.owner_id(), expression)?,
                Builtin::Avg => {
                    if values.is_empty() {
                        return Err(invalid("avg requires at least one value"));
                    }
                    let total = sum_numbers(&values, context.owner_id(), expression)?;
                    evaluate_binary_expression(
                        BinaryExpressionOp::Divide,
                        &total,
                        &Value::Number(Number::from(values.len() as u64)),
                        expression,
                    )?
                }
                Builtin::Min | Builtin::Max => {
                    let mut values = values.into_iter();
                    let first = values
                        .next()
                        .ok_or_else(|| invalid("min/max requires at least one value"))?;
                    if !first.is_number() {
                        return Err(non_numeric(context.owner_id(), expression));
                    }
                    values.try_fold(first, |best, value| {
                        let ordering = numeric_order(&best, &value, expression)?;
                        Ok(
                            if (matches!(signature.operation, Builtin::Min)
                                && ordering == Ordering::Greater)
                                || (matches!(signature.operation, Builtin::Max)
                                    && ordering == Ordering::Less)
                            {
                                value
                            } else {
                                best
                            },
                        )
                    })?
                }
                _ => return Err(invalid("invalid aggregate operation")),
            };
            return Ok(vec![result]);
        }
        _ => {}
    }
    let left = args[0].evaluate_scalar(context)?;
    if args.len() == 1 {
        return Ok(vec![match signature.operation {
            Builtin::Plus if left.is_number() => left,
            Builtin::Minus => negate_number(&left, expression)?,
            _ => return Err(invalid("unary plus requires a numeric value")),
        }]);
    }
    let right = args[1].evaluate_scalar(context)?;
    if matches!(signature.operation, Builtin::Range) {
        return integer_range(&left, &right);
    }
    Ok(vec![match signature.operation {
        Builtin::Plus => {
            evaluate_binary_expression(BinaryExpressionOp::Add, &left, &right, expression)?
        }
        Builtin::Minus => {
            evaluate_binary_expression(BinaryExpressionOp::Subtract, &left, &right, expression)?
        }
        Builtin::Binary(op) => evaluate_binary_expression(op, &left, &right, expression)?,
        Builtin::Remainder => remainder(&left, &right, expression)?,
        Builtin::EagerAnd => boolean_binary(&left, &right, expression, |a, b| a & b)?,
        Builtin::EagerOr => boolean_binary(&left, &right, expression, |a, b| a | b)?,
        Builtin::Xor => boolean_binary(&left, &right, expression, |a, b| a ^ b)?,
        _ => return Err(invalid("invalid intrinsic operation")),
    }])
}

fn check_result_capacity(
    current: usize,
    additional: usize,
    resource: &str,
) -> Result<usize, ExpressionEvaluationError> {
    const LIMIT: usize = 100_000;
    current
        .checked_add(additional)
        .filter(|total| *total <= LIMIT)
        .ok_or_else(|| ExpressionEvaluationError::ResourceLimitExceeded {
            resource: resource.to_string(),
            limit: LIMIT,
        })
}

fn result_value(values: Vec<Value>) -> Value {
    match values.as_slice() {
        [value] => value.clone(),
        _ => Value::Array(values),
    }
}

fn invalid(message: impl Into<String>) -> ExpressionEvaluationError {
    ExpressionEvaluationError::InvalidExpression(message.into())
}

fn non_numeric(owner_id: &str, expression: &ExpressionIr) -> ExpressionEvaluationError {
    ExpressionEvaluationError::NonNumericValue {
        owner: owner_id.to_string(),
        feature: format!("{expression:?}"),
    }
}

fn integer_value(value: &Value) -> Option<i128> {
    let number = value.as_number()?;
    number
        .as_i64()
        .map(i128::from)
        .or_else(|| number.as_u64().map(i128::from))
}

fn integer_result(value: i128) -> Result<Value, ExpressionEvaluationError> {
    if let Ok(value) = i64::try_from(value) {
        Ok(Value::Number(Number::from(value)))
    } else if let Ok(value) = u64::try_from(value) {
        Ok(Value::Number(Number::from(value)))
    } else {
        Err(invalid("integer result exceeds supported i64/u64 range"))
    }
}

fn negate_number(
    value: &Value,
    expression: &ExpressionIr,
) -> Result<Value, ExpressionEvaluationError> {
    if let Some(integer) = integer_value(value) {
        integer_result(-integer)
    } else {
        number_from_f64(-value_as_f64(value, expression)?)
    }
}

fn evaluate_binary_expression(
    op: BinaryExpressionOp,
    left: &Value,
    right: &Value,
    expression: &ExpressionIr,
) -> Result<Value, ExpressionEvaluationError> {
    use BinaryExpressionOp as Op;
    match op {
        Op::Add if left.is_string() && right.is_string() => {
            let (Some(left), Some(right)) = (left.as_str(), right.as_str()) else {
                return Err(invalid("invalid string operands"));
            };
            Ok(Value::String(format!("{left}{right}")))
        }
        Op::Add | Op::Subtract | Op::Multiply | Op::Power => {
            if let (Some(left), Some(right)) = (integer_value(left), integer_value(right)) {
                let result = match op {
                    Op::Add => left.checked_add(right),
                    Op::Subtract => left.checked_sub(right),
                    Op::Multiply => left.checked_mul(right),
                    Op::Power if right >= 0 => {
                        // Handle arbitrarily large exponents of the fixed points without looping.
                        if left == 0 || left == 1 {
                            Some(if right == 0 { 1 } else { left })
                        } else if left == -1 {
                            Some(if right % 2 == 0 { 1 } else { -1 })
                        } else {
                            u32::try_from(right)
                                .ok()
                                .and_then(|exponent| left.checked_pow(exponent))
                        }
                    }
                    Op::Power => {
                        return number_from_f64(
                            value_as_f64(&integer_result(left)?, expression)?
                                .powf(value_as_f64(&integer_result(right)?, expression)?),
                        );
                    }
                    _ => None,
                }
                .ok_or_else(|| invalid("integer arithmetic overflow"))?;
                return integer_result(result);
            }
            let left = value_as_f64(left, expression)?;
            let right = value_as_f64(right, expression)?;
            number_from_f64(match op {
                Op::Add => left + right,
                Op::Subtract => left - right,
                Op::Multiply => left * right,
                Op::Power => left.powf(right),
                _ => return Err(invalid("invalid arithmetic operation")),
            })
        }
        Op::Divide => {
            let left = value_as_f64(left, expression)?;
            let right = value_as_f64(right, expression)?;
            if right == 0.0 {
                return Err(ExpressionEvaluationError::DivisionByZero);
            }
            number_from_f64(left / right)
        }
        Op::Less | Op::LessEqual | Op::Greater | Op::GreaterEqual => {
            let ordering = match (left.as_str(), right.as_str()) {
                (Some(left), Some(right)) => left.cmp(right),
                _ => numeric_order(left, right, expression)?,
            };
            Ok(Value::Bool(match op {
                Op::Less => ordering == Ordering::Less,
                Op::LessEqual => ordering != Ordering::Greater,
                Op::Greater => ordering == Ordering::Greater,
                Op::GreaterEqual => ordering != Ordering::Less,
                _ => false,
            }))
        }
        Op::Equal => Ok(Value::Bool(value_equal(left, right))),
        Op::NotEqual => Ok(Value::Bool(!value_equal(left, right))),
        Op::And => boolean_binary(left, right, expression, |a, b| a && b),
        Op::Or => boolean_binary(left, right, expression, |a, b| a || b),
    }
}

fn sequence_equal(left: &[Value], right: &[Value]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| value_equal(left, right))
}

fn value_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            number_order(left, right) == Some(Ordering::Equal)
        }
        // Literal arrays and objects remain data values rather than expression sequences.
        _ => left == right,
    }
}

fn number_order(left: &Number, right: &Number) -> Option<Ordering> {
    let integer = |number: &Number| {
        number
            .as_i64()
            .map(i128::from)
            .or_else(|| number.as_u64().map(i128::from))
    };
    match (integer(left), integer(right)) {
        (Some(left), Some(right)) => Some(left.cmp(&right)),
        (Some(left), None) => Some(integer_real_order(left, right.as_f64()?)),
        (None, Some(right)) => Some(integer_real_order(right, left.as_f64()?).reverse()),
        (None, None) => left.as_f64()?.partial_cmp(&right.as_f64()?),
    }
}

fn integer_real_order(integer: i128, real: f64) -> Ordering {
    // JSON numbers are finite. Check bounds before the truncating cast so large integers
    // never first round to f64, e.g. 9007199254740993 != 9007199254740992.0.
    if real >= i128::MAX as f64 {
        return Ordering::Less;
    }
    if real <= i128::MIN as f64 {
        return Ordering::Greater;
    }
    match integer.cmp(&(real as i128)) {
        Ordering::Equal if real.fract() > 0.0 => Ordering::Less,
        Ordering::Equal if real.fract() < 0.0 => Ordering::Greater,
        other => other,
    }
}

fn numeric_order(
    left: &Value,
    right: &Value,
    expression: &ExpressionIr,
) -> Result<Ordering, ExpressionEvaluationError> {
    match (left.as_number(), right.as_number()) {
        (Some(left), Some(right)) => number_order(left, right)
            .ok_or_else(|| invalid("numeric comparison exceeds the supported value domain")),
        _ => Err(invalid(format!(
            "expected numeric operands: {expression:?}"
        ))),
    }
}

fn sum_numbers(
    values: &[Value],
    owner_id: &str,
    expression: &ExpressionIr,
) -> Result<Value, ExpressionEvaluationError> {
    values
        .iter()
        .try_fold(Value::Number(Number::from(0)), |total, value| {
            if !value.is_number() {
                return Err(non_numeric(owner_id, expression));
            }
            evaluate_binary_expression(BinaryExpressionOp::Add, &total, value, expression)
        })
}

fn remainder(
    left: &Value,
    right: &Value,
    expression: &ExpressionIr,
) -> Result<Value, ExpressionEvaluationError> {
    if let (Some(left), Some(right)) = (integer_value(left), integer_value(right)) {
        if right == 0 {
            return Err(ExpressionEvaluationError::DivisionByZero);
        }
        return integer_result(left % right);
    }
    let left = value_as_f64(left, expression)?;
    let right = value_as_f64(right, expression)?;
    if right == 0.0 {
        return Err(ExpressionEvaluationError::DivisionByZero);
    }
    number_from_f64(left % right)
}

fn integer_range(left: &Value, right: &Value) -> Result<Vec<Value>, ExpressionEvaluationError> {
    const MAX_RANGE_RESULTS: i128 = 100_000;
    let (Some(lower), Some(upper)) = (integer_value(left), integer_value(right)) else {
        return Err(invalid("range bounds must be integers"));
    };
    if upper < lower {
        return Ok(Vec::new());
    }
    let count = upper - lower + 1;
    if count > MAX_RANGE_RESULTS {
        return Err(invalid(format!(
            "range exceeds {MAX_RANGE_RESULTS} result execution limit"
        )));
    }
    (lower..=upper).map(integer_result).collect()
}

fn number_from_f64(value: f64) -> Result<Value, ExpressionEvaluationError> {
    Number::from_f64(value)
        .map(Value::Number)
        .ok_or(ExpressionEvaluationError::NonFiniteResult)
}

fn boolean_binary(
    left: &Value,
    right: &Value,
    expression: &ExpressionIr,
    op: impl FnOnce(bool, bool) -> bool,
) -> Result<Value, ExpressionEvaluationError> {
    Ok(Value::Bool(op(
        value_as_bool(left, expression)?,
        value_as_bool(right, expression)?,
    )))
}

fn value_as_bool(
    value: &Value,
    expression: &ExpressionIr,
) -> Result<bool, ExpressionEvaluationError> {
    value
        .as_bool()
        .ok_or_else(|| invalid(format!("expected Boolean operand: {expression:?}")))
}

fn value_as_f64(
    value: &Value,
    expression: &ExpressionIr,
) -> Result<f64, ExpressionEvaluationError> {
    let real = value
        .as_f64()
        .ok_or_else(|| invalid(format!("expected numeric operand: {expression:?}")))?;
    if let Some(integer) = integer_value(value) {
        if real as i128 != integer {
            return Err(invalid(
                "integer cannot be converted to real without losing precision",
            ));
        }
    }
    Ok(real)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::{Value, json};

    use super::{
        BinaryExpressionOp, ExpressionBinding, ExpressionContract, ExpressionContractError,
        ExpressionEvaluationContext, ExpressionEvaluationError, ExpressionIr,
        ExpressionMultiplicity, ExpressionPathRoot, ExpressionPathSegment, ExpressionValueKind,
        ExpressionValueType, UnaryExpressionOp,
    };

    #[derive(Default)]
    struct TestEvaluationContext {
        owner_id: String,
        paths: BTreeMap<Vec<String>, Vec<Value>>,
    }

    impl ExpressionEvaluationContext for TestEvaluationContext {
        fn owner_id(&self) -> &str {
            &self.owner_id
        }

        fn resolve_path(
            &mut self,
            segments: &[ExpressionPathSegment],
        ) -> Result<Vec<Value>, ExpressionEvaluationError> {
            let key = segments
                .iter()
                .map(ExpressionPathSegment::name)
                .map(str::to_string)
                .collect::<Vec<_>>();
            self.paths
                .get(&key)
                .cloned()
                .ok_or_else(|| ExpressionEvaluationError::InvalidExpression(key.join(".")))
        }
    }

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

    #[test]
    fn evaluates_pure_expression_ir_with_path_callback() {
        let mut context = TestEvaluationContext {
            owner_id: "assembly.Vehicle".to_string(),
            paths: [(
                vec!["parts".to_string(), "mass".to_string()],
                vec![json!(4.0), json!(6.5)],
            )]
            .into_iter()
            .collect(),
        };
        let expression = ExpressionIr::Binary {
            left: Box::new(ExpressionIr::Call {
                function: "sum".to_string(),
                args: vec![ExpressionIr::Path {
                    root: ExpressionPathRoot::SelfRef,
                    segments: vec![
                        ExpressionPathSegment::Name("parts".to_string()),
                        ExpressionPathSegment::Name("mass".to_string()),
                    ],
                }],
            }),
            op: BinaryExpressionOp::Greater,
            right: Box::new(ExpressionIr::Literal { value: json!(10) }),
        };

        assert_eq!(expression.evaluate(&mut context).unwrap(), json!(true));
    }

    #[test]
    fn reports_nonnumeric_sum_values_from_shared_evaluator() {
        let mut context = TestEvaluationContext {
            owner_id: "assembly.Vehicle".to_string(),
            paths: [(
                vec!["parts".to_string(), "mass".to_string()],
                vec![json!(4.0), json!("heavy")],
            )]
            .into_iter()
            .collect(),
        };
        let expression = ExpressionIr::Call {
            function: "sum".to_string(),
            args: vec![ExpressionIr::Path {
                root: ExpressionPathRoot::SelfRef,
                segments: vec![
                    ExpressionPathSegment::Name("parts".to_string()),
                    ExpressionPathSegment::Name("mass".to_string()),
                ],
            }],
        };

        let error = expression.evaluate(&mut context).unwrap_err();
        assert!(matches!(
            error,
            ExpressionEvaluationError::NonNumericValue {
                owner,
                ..
            } if owner == "assembly.Vehicle"
        ));
    }

    #[test]
    fn evaluates_numeric_aggregate_functions() {
        let mut context = TestEvaluationContext {
            owner_id: "assembly.Vehicle".to_string(),
            paths: [(
                vec!["parts".to_string(), "mass".to_string()],
                vec![json!(4.0), json!(6.0), json!(11.0)],
            )]
            .into_iter()
            .collect(),
        };
        let aggregate_arg = ExpressionIr::Path {
            root: ExpressionPathRoot::SelfRef,
            segments: vec![
                ExpressionPathSegment::Name("parts".to_string()),
                ExpressionPathSegment::Name("mass".to_string()),
            ],
        };

        for (function, expected) in [
            ("sum", json!(21.0)),
            ("min", json!(4.0)),
            ("max", json!(11.0)),
            ("avg", json!(7.0)),
        ] {
            let expression = ExpressionIr::Call {
                function: function.to_string(),
                args: vec![aggregate_arg.clone()],
            };
            assert_eq!(expression.evaluate(&mut context).unwrap(), expected);
        }
    }

    #[test]
    fn validates_runtime_supported_function_policy() {
        let expression = ExpressionIr::Call {
            function: "median".to_string(),
            args: vec![ExpressionIr::Literal { value: json!(1) }],
        };

        let error = expression.validate_runtime_supported().unwrap_err();
        assert_eq!(
            error.to_string(),
            "unsupported expression_ir function `median`"
        );
    }

    #[test]
    fn validates_runtime_supported_call_arity() {
        let expression = ExpressionIr::Call {
            function: "sum".to_string(),
            args: vec![
                ExpressionIr::Literal { value: json!(1) },
                ExpressionIr::Literal { value: json!(2) },
            ],
        };

        let error = expression.validate_runtime_supported().unwrap_err();
        assert_eq!(
            error.to_string(),
            "expression_ir function `sum` expects one argument, got 2"
        );
    }

    fn literal(value: Value) -> ExpressionIr {
        ExpressionIr::Literal { value }
    }

    fn binary(op: BinaryExpressionOp, left: ExpressionIr, right: ExpressionIr) -> ExpressionIr {
        ExpressionIr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    fn call(function: &str, args: Vec<ExpressionIr>) -> ExpressionIr {
        ExpressionIr::Call {
            function: function.to_string(),
            args,
        }
    }

    fn evaluate(expression: ExpressionIr) -> Value {
        expression
            .evaluate(&mut TestEvaluationContext::default())
            .unwrap()
    }

    fn division_by_zero() -> ExpressionIr {
        binary(
            BinaryExpressionOp::Divide,
            literal(json!(1)),
            literal(json!(0)),
        )
    }

    // KerML 9.4.11 / 9.4.2: arithmetic results and numeric equality must not depend
    // on whether serde_json stores the same number as an integer or a float.
    #[test]
    fn numeric_equality_and_order_preserve_integer_precision() {
        assert_eq!(
            evaluate(binary(
                BinaryExpressionOp::Equal,
                binary(
                    BinaryExpressionOp::Add,
                    literal(json!(1)),
                    literal(json!(1))
                ),
                literal(json!(2))
            )),
            json!(true)
        );
        for (left, right, equal) in [
            (json!(1), json!(1.0), true),
            (json!(0), json!(-0.0), true),
            (
                json!(9007199254740993_u64),
                json!(9007199254740992.0),
                false,
            ),
            (json!(u64::MAX), json!(18446744073709551616.0), false),
            (json!(i64::MIN), json!(-9223372036854775808.0), true),
            (json!(-1), json!(-1.5), false),
        ] {
            assert_eq!(
                evaluate(binary(
                    BinaryExpressionOp::Equal,
                    literal(left.clone()),
                    literal(right.clone())
                )),
                json!(equal)
            );
            assert_eq!(
                evaluate(binary(
                    BinaryExpressionOp::NotEqual,
                    literal(left),
                    literal(right)
                )),
                json!(!equal)
            );
        }
        for (left, right) in [
            (json!(9007199254740993_u64), json!(9007199254740992.0)),
            (json!(u64::MAX), json!(-1)),
            (json!(-1), json!(-1.5)),
            (json!(0), json!(-0.5)),
        ] {
            assert_eq!(
                evaluate(binary(
                    BinaryExpressionOp::Greater,
                    literal(left.clone()),
                    literal(right.clone())
                )),
                json!(true)
            );
            assert_eq!(
                evaluate(binary(
                    BinaryExpressionOp::Less,
                    literal(right),
                    literal(left)
                )),
                json!(true)
            );
        }
    }

    #[test]
    fn integer_arithmetic_is_checked_and_exact() {
        for (op, left, right, expected) in [
            (
                BinaryExpressionOp::Subtract,
                json!(9007199254740993_u64),
                json!(9007199254740992_u64),
                json!(1),
            ),
            (
                BinaryExpressionOp::Add,
                json!(i64::MAX),
                json!(1),
                json!(9223372036854775808_u64),
            ),
            (
                BinaryExpressionOp::Multiply,
                json!(9007199254740993_u64),
                json!(2),
                json!(18014398509481986_u64),
            ),
            (
                BinaryExpressionOp::Power,
                json!(3),
                json!(34),
                json!(16677181699666569_u64),
            ),
            (
                BinaryExpressionOp::Power,
                json!(-1),
                json!(u64::MAX),
                json!(-1),
            ),
        ] {
            assert_eq!(
                evaluate(binary(op, literal(left), literal(right))),
                expected
            );
        }
        for (op, left, right) in [
            (BinaryExpressionOp::Add, json!(u64::MAX), json!(1)),
            (BinaryExpressionOp::Subtract, json!(i64::MIN), json!(1)),
            (
                BinaryExpressionOp::Multiply,
                json!(u64::MAX),
                json!(u64::MAX),
            ),
            (BinaryExpressionOp::Power, json!(2), json!(64)),
            (
                BinaryExpressionOp::Add,
                json!(9007199254740993_u64),
                json!(0.0),
            ),
        ] {
            assert!(
                binary(op, literal(left), literal(right))
                    .evaluate(&mut TestEvaluationContext::default())
                    .is_err()
            );
        }
        assert_eq!(
            evaluate(ExpressionIr::Unary {
                op: UnaryExpressionOp::Negate,
                expr: Box::new(literal(json!(i64::MIN)))
            }),
            json!(9223372036854775808_u64)
        );
        assert_eq!(
            evaluate(binary(
                BinaryExpressionOp::Divide,
                literal(json!(3)),
                literal(json!(2))
            )),
            json!(1.5)
        );
        assert_eq!(
            evaluate(binary(
                BinaryExpressionOp::Power,
                literal(json!(2)),
                literal(json!(-1))
            )),
            json!(0.5)
        );
    }

    // KerML 9.4.17 control operators evaluate only the selected operands.
    #[test]
    fn controls_are_lazy_but_structurally_validate_all_branches() {
        assert_eq!(
            evaluate(binary(
                BinaryExpressionOp::And,
                literal(json!(false)),
                division_by_zero()
            )),
            json!(false)
        );
        assert_eq!(
            evaluate(binary(
                BinaryExpressionOp::Or,
                literal(json!(true)),
                division_by_zero()
            )),
            json!(true)
        );
        assert_eq!(
            evaluate(call(
                "implies",
                vec![literal(json!(false)), division_by_zero()]
            )),
            json!(true)
        );
        assert_eq!(
            evaluate(call(
                "if",
                vec![literal(json!(true)), literal(json!(7)), division_by_zero()]
            )),
            json!(7)
        );
        assert_eq!(
            evaluate(call(
                "if",
                vec![literal(json!(false)), division_by_zero(), literal(json!(9))]
            )),
            json!(9)
        );
        assert_eq!(
            evaluate(call("??", vec![literal(json!(false)), division_by_zero()])),
            json!(false)
        );
        assert_eq!(
            evaluate(call("??", vec![literal(Value::Null), literal(json!(2))])),
            json!(2)
        );
        assert_eq!(
            evaluate(call(
                "ControlFunctions::'and'",
                vec![literal(json!(false)), division_by_zero()]
            )),
            json!(false)
        );
        assert!(
            call(
                "if",
                vec![
                    literal(json!(true)),
                    literal(json!(1)),
                    call("unknown", vec![])
                ]
            )
            .evaluate(&mut TestEvaluationContext::default())
            .is_err()
        );
        assert!(
            call(
                "if",
                vec![literal(json!(0)), literal(json!(1)), literal(json!(2))]
            )
            .evaluate(&mut TestEvaluationContext::default())
            .is_err()
        );
        for expression in [
            binary(
                BinaryExpressionOp::And,
                literal(json!(true)),
                division_by_zero(),
            ),
            binary(
                BinaryExpressionOp::Or,
                literal(json!(false)),
                division_by_zero(),
            ),
            call("implies", vec![literal(json!(true)), division_by_zero()]),
            call("??", vec![literal(Value::Null), division_by_zero()]),
        ] {
            assert!(matches!(
                expression.evaluate(&mut TestEvaluationContext::default()),
                Err(ExpressionEvaluationError::DivisionByZero)
            ));
        }
    }

    #[test]
    fn eager_boolean_operators_evaluate_both_operands() {
        for (function, left, right, expected) in [
            ("&", true, false, false),
            ("|", false, true, true),
            ("xor", true, true, false),
            ("xor", true, false, true),
        ] {
            assert_eq!(
                evaluate(call(
                    function,
                    vec![literal(json!(left)), literal(json!(right))]
                )),
                json!(expected)
            );
            assert!(matches!(
                call(function, vec![literal(json!(left)), division_by_zero()])
                    .evaluate(&mut TestEvaluationContext::default()),
                Err(ExpressionEvaluationError::DivisionByZero)
            ));
        }
    }

    // KerML 7.4.9.3: concatenate result sequences, not arbitrary data payloads.
    #[test]
    fn sequences_flatten_results_and_null_is_empty() {
        let sequence = ExpressionIr::Tuple {
            items: vec![
                ExpressionIr::Tuple {
                    items: vec![literal(json!(1)), literal(json!(2))],
                },
                literal(Value::Null),
                literal(json!(3)),
            ],
        };
        assert_eq!(evaluate(sequence.clone()), json!([1, 2, 3]));
        assert_eq!(evaluate(call("count", vec![sequence.clone()])), json!(3));
        assert_eq!(evaluate(call("size", vec![sequence.clone()])), json!(3));
        assert_eq!(evaluate(call("sum", vec![sequence])), json!(6));
        assert_eq!(evaluate(literal(Value::Null)), json!([]));
        assert_eq!(evaluate(call("size", vec![literal(Value::Null)])), json!(0));
        assert_eq!(evaluate(call("sum", vec![literal(Value::Null)])), json!(0));
        assert_eq!(
            evaluate(binary(
                BinaryExpressionOp::Equal,
                literal(Value::Null),
                ExpressionIr::Tuple { items: vec![] }
            )),
            json!(true)
        );
        assert_eq!(
            evaluate(call("??", vec![literal(json!([])), division_by_zero()])),
            json!([])
        );
        assert_eq!(
            evaluate(call(
                "??",
                vec![ExpressionIr::Tuple { items: vec![] }, literal(json!(2))]
            )),
            json!(2)
        );
        let data = literal(json!([1, [2, 3]]));
        assert_eq!(evaluate(call("size", vec![data.clone()])), json!(1));
        assert_eq!(
            evaluate(ExpressionIr::Tuple {
                items: vec![data, literal(json!(4))]
            }),
            json!([[1, [2, 3]], 4])
        );
        let object = json!({"items": [1, [2]], "nullable": null});
        assert_eq!(evaluate(literal(object.clone())), object);
        assert!(
            binary(
                BinaryExpressionOp::Equal,
                ExpressionIr::Tuple {
                    items: vec![literal(json!(1)), literal(json!(2))]
                },
                ExpressionIr::Tuple {
                    items: vec![literal(json!(1)), literal(json!(2))]
                }
            )
            .evaluate(&mut TestEvaluationContext::default())
            .is_err()
        );
    }

    #[test]
    fn standard_builtins_use_exact_qualification_and_correct_arities() {
        let sequence = ExpressionIr::Tuple {
            items: vec![literal(json!(1)), literal(json!(2))],
        };
        for name in [
            "sum",
            "NumericalFunctions::sum",
            "NumericalFunctions.sum",
            "IntegerFunctions::sum",
        ] {
            assert_eq!(evaluate(call(name, vec![sequence.clone()])), json!(3));
        }
        assert_eq!(
            evaluate(call("SequenceFunctions::size", vec![sequence.clone()])),
            json!(2)
        );
        for name in ["min", "NumericalFunctions::min", "IntegerFunctions.min"] {
            assert_eq!(
                evaluate(call(name, vec![literal(json!(2)), literal(json!(1))])),
                json!(1)
            );
        }
        assert_eq!(evaluate(call("max", vec![sequence])), json!(2));
        for expression in [
            call("NumericalFunctions::min", vec![literal(json!(1))]),
            call(
                "min",
                vec![literal(json!(1)), literal(json!(2)), literal(json!(3))],
            ),
            call("sum", vec![literal(json!(1)), literal(json!(2))]),
            call("UserFunctions::sum", vec![literal(json!(1))]),
            call("ScalarFunctions::sum", vec![literal(json!(1))]),
            call("SequenceFunctions::avg", vec![literal(json!(1))]),
            call("if", vec![literal(json!(true)), literal(json!(1))]),
            call("IntegerFunctions::sum", vec![literal(json!(1.5))]),
        ] {
            assert!(
                expression
                    .evaluate(&mut TestEvaluationContext::default())
                    .is_err(),
                "{expression:?}"
            );
        }
    }

    #[test]
    fn sequence_index_is_one_based_and_range_is_bounded() {
        assert_eq!(
            evaluate(call("..", vec![literal(json!(1)), literal(json!(3))])),
            json!([1, 2, 3])
        );
        assert_eq!(
            evaluate(call("..", vec![literal(json!(3)), literal(json!(1))])),
            json!([])
        );
        let sequence = call("..", vec![literal(json!(1)), literal(json!(3))]);
        assert_eq!(
            evaluate(call("#", vec![sequence.clone(), literal(json!(1))])),
            json!(1)
        );
        assert_eq!(
            evaluate(call("#", vec![sequence.clone(), literal(json!(3))])),
            json!(3)
        );
        assert_eq!(
            evaluate(call("#", vec![sequence.clone(), literal(json!(4))])),
            json!([])
        );
        assert_eq!(
            evaluate(call("#", vec![sequence.clone(), literal(json!(u64::MAX))])),
            json!([])
        );
        for expression in [
            call("#", vec![sequence.clone(), literal(json!(0))]),
            call("#", vec![sequence.clone(), literal(json!(-1))]),
            call("#", vec![sequence, literal(json!(1.0))]),
            call("..", vec![literal(json!(1)), literal(json!(100001))]),
            call("..", vec![literal(json!(1.0)), literal(json!(3))]),
        ] {
            assert!(
                expression
                    .evaluate(&mut TestEvaluationContext::default())
                    .is_err()
            );
        }
    }

    #[test]
    fn remainder_unary_plus_and_string_operators_match_scalar_domains() {
        assert_eq!(
            evaluate(call("%", vec![literal(json!(7)), literal(json!(3))])),
            json!(1)
        );
        assert_eq!(
            evaluate(call("%", vec![literal(json!(-7)), literal(json!(3))])),
            json!(-1)
        );
        assert_eq!(
            evaluate(call("+", vec![literal(json!(9007199254740993_u64))])),
            json!(9007199254740993_u64)
        );
        assert_eq!(
            evaluate(binary(
                BinaryExpressionOp::Add,
                literal(json!("hello ")),
                literal(json!("world"))
            )),
            json!("hello world")
        );
        assert_eq!(
            evaluate(binary(
                BinaryExpressionOp::Less,
                literal(json!("alpha")),
                literal(json!("beta"))
            )),
            json!(true)
        );
        assert_eq!(
            evaluate(binary(
                BinaryExpressionOp::Greater,
                literal(json!("\u{e9}")),
                literal(json!("e"))
            )),
            json!(true)
        );
        assert!(
            call("%", vec![literal(json!(1)), literal(json!(0))])
                .evaluate(&mut TestEvaluationContext::default())
                .is_err()
        );
        assert!(
            call("+", vec![literal(json!("one"))])
                .evaluate(&mut TestEvaluationContext::default())
                .is_err()
        );
        assert!(
            binary(
                BinaryExpressionOp::Add,
                literal(json!("one")),
                literal(json!(1))
            )
            .evaluate(&mut TestEvaluationContext::default())
            .is_err()
        );
    }

    #[test]
    fn renders_intrinsics_without_losing_operator_syntax() {
        let cases = [
            (
                call(
                    "if",
                    vec![literal(json!(true)), literal(json!(1)), literal(json!(2))],
                ),
                "(if true ? 1 else 2)",
            ),
            (
                call(
                    "#",
                    vec![
                        call("..", vec![literal(json!(1)), literal(json!(3))]),
                        literal(json!(2)),
                    ],
                ),
                "((1 .. 3))#(2)",
            ),
            (call("+", vec![literal(json!(1))]), "+(1)"),
            (
                call("??", vec![literal(Value::Null), literal(json!(2))]),
                "(null ?? 2)",
            ),
            (
                call("NumericalFunctions.sum", vec![literal(json!(1))]),
                "NumericalFunctions::sum(1)",
            ),
            (literal(json!("a")), "\"a\""),
        ];
        for (expression, expected) in cases {
            assert_eq!(expression.render_constraint_expression(), expected);
        }
    }
    fn checked(
        expression: ExpressionIr,
        value_type: ExpressionValueType,
        lower: u64,
        upper: Option<u64>,
    ) -> ExpressionIr {
        ExpressionIr::Checked {
            expression: Box::new(expression),
            contract: ExpressionContract {
                value_type,
                multiplicity: Some(ExpressionMultiplicity { lower, upper }),
            },
        }
    }

    fn parameter(feature: &str, name: &str) -> ExpressionIr {
        ExpressionIr::Path {
            root: ExpressionPathRoot::SelfRef,
            segments: vec![ExpressionPathSegment::Resolved {
                name: name.to_string(),
                feature: Some(feature.to_string()),
            }],
        }
    }

    #[test]
    fn explicit_results_preserve_array_data_and_empty_cardinality() {
        let mut context = TestEvaluationContext::default();
        let array = literal(json!([1, 2]))
            .evaluate_result(&mut context)
            .unwrap();
        let sequence = ExpressionIr::Tuple {
            items: vec![literal(json!(1)), literal(json!(2))],
        }
        .evaluate_result(&mut context)
        .unwrap();
        assert_eq!(array.cardinality(), 1);
        assert_eq!(sequence.cardinality(), 2);
        assert_eq!(
            serde_json::to_value(&array).unwrap(),
            json!({"values": [[1, 2]]})
        );
        assert_eq!(array.into_legacy_value(), sequence.into_legacy_value());
        assert_eq!(
            literal(Value::Null)
                .evaluate_result(&mut context)
                .unwrap()
                .cardinality(),
            0
        );
        assert_eq!(
            literal(json!([]))
                .evaluate_result(&mut context)
                .unwrap()
                .cardinality(),
            1
        );
    }

    #[test]
    fn scalar_contracts_check_every_value_without_numeric_coercion() {
        for (value_type, value, accepted) in [
            (ExpressionValueType::Integer, json!(u64::MAX), true),
            (ExpressionValueType::Integer, json!(i64::MIN), true),
            (ExpressionValueType::Integer, json!(1.0), false),
            (ExpressionValueType::Natural, json!(0), true),
            (ExpressionValueType::Natural, json!(-1), false),
            (ExpressionValueType::Natural, json!(1.0), false),
            (ExpressionValueType::Positive, json!(1), true),
            (ExpressionValueType::Positive, json!(0), false),
            (ExpressionValueType::Real, json!(u64::MAX), true),
            (ExpressionValueType::Real, json!(1.5), true),
            (ExpressionValueType::Real, json!("1"), false),
            (ExpressionValueType::Boolean, json!(false), true),
            (ExpressionValueType::Boolean, json!(0), false),
            (ExpressionValueType::String, json!("value"), true),
            (ExpressionValueType::String, json!(["value"]), false),
            (ExpressionValueType::Any, json!({"data": [1, 2]}), true),
        ] {
            let result = checked(literal(value.clone()), value_type, 1, Some(1))
                .evaluate(&mut TestEvaluationContext::default());
            assert_eq!(result.is_ok(), accepted, "{value_type:?}: {value}");
            if accepted {
                assert_eq!(result.unwrap(), value);
            }
        }
        let result = checked(
            ExpressionIr::Tuple {
                items: vec![literal(json!(1)), literal(json!("bad"))],
            },
            ExpressionValueType::Integer,
            2,
            Some(2),
        )
        .evaluate(&mut TestEvaluationContext::default());
        assert!(matches!(
            result,
            Err(ExpressionEvaluationError::ContractViolation(
                ExpressionContractError::TypeMismatch {
                    expected: ExpressionValueType::Integer,
                    actual: ExpressionValueKind::String,
                    result_index: 2
                }
            ))
        ));
    }

    #[test]
    fn multiplicity_contracts_check_empty_fixed_bounded_and_unlimited_results() {
        for (count, lower, upper, accepted) in [
            (0, 0, Some(0), true),
            (0, 1, Some(1), false),
            (1, 1, Some(1), true),
            (2, 1, Some(1), false),
            (2, 2, Some(4), true),
            (4, 2, Some(4), true),
            (5, 2, Some(4), false),
            (20, 0, None, true),
        ] {
            let expression = ExpressionIr::Tuple {
                items: vec![literal(json!(5)); count],
            };
            let result = checked(expression, ExpressionValueType::Integer, lower, upper)
                .evaluate(&mut TestEvaluationContext::default());
            assert_eq!(
                result.is_ok(),
                accepted,
                "count={count}, range={lower}..{upper:?}"
            );
            if !accepted {
                assert!(
                    matches!(result, Err(ExpressionEvaluationError::ContractViolation(
                    ExpressionContractError::MultiplicityMismatch { actual, .. }
                )) if actual == count as u64)
                );
            }
        }
        let unspecified = ExpressionIr::Checked {
            expression: Box::new(ExpressionIr::Tuple {
                items: vec![literal(json!(1)), literal(json!(2))],
            }),
            contract: ExpressionContract {
                value_type: ExpressionValueType::Integer,
                multiplicity: None,
            },
        };
        assert_eq!(evaluate(unspecified), json!([1, 2]));
        assert!(
            checked(literal(json!([1, 2])), ExpressionValueType::Any, 2, Some(2))
                .evaluate(&mut TestEvaluationContext::default())
                .is_err()
        );
    }

    #[test]
    fn multiplicity_bound_errors_are_explicit_and_structured() {
        assert_eq!(
            ExpressionMultiplicity::from_bounds(" 2 ", " * ").unwrap(),
            ExpressionMultiplicity {
                lower: 2,
                upper: None
            }
        );
        assert!(matches!(
            ExpressionMultiplicity::from_bounds("3", "2"),
            Err(ExpressionContractError::InvalidMultiplicityBounds { lower: 3, upper: 2 })
        ));
        for (lower, upper) in [
            ("n", "4"),
            ("0", "n + 1"),
            ("*", "*"),
            ("-1", "2"),
            ("0", "1.0"),
            ("0", "18446744073709551616"),
        ] {
            let error = ExpressionMultiplicity::from_bounds(lower, upper).unwrap_err();
            assert!(matches!(
                error,
                ExpressionContractError::UnsupportedMultiplicityBound { .. }
            ));
            assert_eq!(
                serde_json::to_value(error).unwrap()["code"],
                "unsupported_multiplicity_bound"
            );
        }
        let invalid = checked(literal(json!(1)), ExpressionValueType::Integer, 2, Some(1));
        assert!(matches!(
            invalid.evaluate(&mut TestEvaluationContext::default()),
            Err(ExpressionEvaluationError::ContractViolation(
                ExpressionContractError::InvalidMultiplicityBounds { .. }
            ))
        ));
    }

    #[test]
    fn checked_boundaries_preserve_lazy_controls() {
        let wrong_type = checked(
            literal(json!("wrong")),
            ExpressionValueType::Integer,
            1,
            Some(1),
        );
        assert_eq!(
            evaluate(call(
                "if",
                vec![literal(json!(true)), literal(json!(5)), wrong_type.clone()]
            )),
            json!(5)
        );
        assert!(matches!(
            call(
                "if",
                vec![literal(json!(false)), literal(json!(5)), wrong_type]
            )
            .evaluate(&mut TestEvaluationContext::default()),
            Err(ExpressionEvaluationError::ContractViolation(_))
        ));
    }

    #[test]
    fn invocation_frames_use_feature_identity_and_preserve_result_sequences() {
        let expression = ExpressionIr::Invoke {
            function: "type.F".to_string(),
            bindings: vec![
                ExpressionBinding {
                    lexical: false,
                    feature: "feature.F.x".to_string(),
                    expression: literal(json!(3)),
                },
                ExpressionBinding {
                    lexical: false,
                    feature: "feature.F.y".to_string(),
                    expression: binary(
                        BinaryExpressionOp::Add,
                        parameter("feature.F.x", "x"),
                        literal(json!(2)),
                    ),
                },
            ],
            body: Box::new(ExpressionIr::Invoke {
                function: "type.G".to_string(),
                bindings: vec![ExpressionBinding {
                    lexical: false,
                    feature: "feature.G.x".to_string(),
                    expression: parameter("feature.F.y", "y"),
                }],
                body: Box::new(ExpressionIr::Tuple {
                    items: vec![
                        parameter("feature.F.x", "sameName"),
                        parameter("feature.G.x", "sameName"),
                    ],
                }),
            }),
        };
        assert_eq!(evaluate(expression), json!([3, 5]));
    }

    #[test]
    fn invocation_arguments_evaluate_once_and_do_not_shadow_unrelated_ids() {
        #[derive(Default)]
        struct CountingContext {
            reads: usize,
        }
        impl ExpressionEvaluationContext for CountingContext {
            fn owner_id(&self) -> &str {
                "owner"
            }
            fn resolve_path(
                &mut self,
                _segments: &[ExpressionPathSegment],
            ) -> Result<Vec<Value>, ExpressionEvaluationError> {
                self.reads += 1;
                Ok(vec![json!(7)])
            }
        }
        let expression = ExpressionIr::Invoke {
            function: "type.F".to_string(),
            bindings: vec![ExpressionBinding {
                lexical: false,
                feature: "feature.F.x".to_string(),
                expression: parameter("feature.Outer.x", "x"),
            }],
            body: Box::new(binary(
                BinaryExpressionOp::Add,
                parameter("feature.F.x", "x"),
                parameter("feature.F.x", "x"),
            )),
        };
        let mut context = CountingContext::default();
        assert_eq!(expression.evaluate(&mut context).unwrap(), json!(14));
        assert_eq!(context.reads, 1);
    }

    #[test]
    fn invocation_rejects_duplicate_forward_and_cyclic_bindings() {
        for (bindings, duplicate) in [
            (
                vec![
                    ExpressionBinding {
                        lexical: false,
                        feature: "x".to_string(),
                        expression: literal(json!(1)),
                    },
                    ExpressionBinding {
                        lexical: false,
                        feature: "x".to_string(),
                        expression: literal(json!(2)),
                    },
                ],
                true,
            ),
            (
                vec![
                    ExpressionBinding {
                        lexical: false,
                        feature: "x".to_string(),
                        expression: parameter("y", "y"),
                    },
                    ExpressionBinding {
                        lexical: false,
                        feature: "y".to_string(),
                        expression: literal(json!(2)),
                    },
                ],
                false,
            ),
            (
                vec![ExpressionBinding {
                    lexical: false,
                    feature: "x".to_string(),
                    expression: parameter("x", "x"),
                }],
                false,
            ),
        ] {
            let expression = ExpressionIr::Invoke {
                function: "type.F".to_string(),
                bindings,
                body: Box::new(parameter("x", "x")),
            };
            let error = expression
                .evaluate(&mut TestEvaluationContext::default())
                .unwrap_err();
            assert!(if duplicate {
                matches!(error, ExpressionEvaluationError::DuplicateBinding { .. })
            } else {
                matches!(error, ExpressionEvaluationError::UnboundParameter { .. })
            });
        }
    }

    #[test]
    fn invocation_and_expression_resource_limits_are_deterministic() {
        let mut expression = literal(json!(1));
        for _ in 0..65 {
            expression = ExpressionIr::Invoke {
                function: "type.F".to_string(),
                bindings: vec![],
                body: Box::new(expression),
            };
        }
        assert!(
            matches!(expression.evaluate(&mut TestEvaluationContext::default()), Err(ExpressionEvaluationError::ResourceLimitExceeded { resource, limit: 64 }) if resource == "invocation_depth")
        );
        let expression = ExpressionIr::Tuple {
            items: vec![literal(json!(1)); 10_000],
        };
        assert!(
            matches!(expression.evaluate(&mut TestEvaluationContext::default()), Err(ExpressionEvaluationError::ResourceLimitExceeded { resource, limit: 10_000 }) if resource == "expression_nodes")
        );
    }

    #[test]
    fn checked_invocation_ir_round_trips_and_normalizes_nested_paths() {
        let expression = ExpressionIr::from_value(&json!({
            "kind": "invoke", "function": "type.F",
            "bindings": [{"feature": "feature.F.x", "expression": {"kind": "literal", "value": 8}}],
            "body": {"kind": "checked", "contract": {"value_type": "integer", "multiplicity": {"lower": 1, "upper": 1}},
                "expression": {"kind": "path", "segments": [{"name": "x", "feature": "feature.F.x"}]}}
        })).unwrap();
        assert_eq!(evaluate(expression.clone()), json!(8));
        assert_eq!(
            ExpressionIr::from_value(&expression.to_value().unwrap()).unwrap(),
            expression
        );
    }
    #[test]
    fn result_and_frame_caps_prevent_unbounded_sequence_accumulation() {
        let range = |count| call("..", vec![literal(json!(1)), literal(json!(count))]);
        let expression = ExpressionIr::Tuple {
            items: vec![range(50_000), range(50_001)],
        };
        assert!(
            matches!(expression.evaluate(&mut TestEvaluationContext::default()),
            Err(ExpressionEvaluationError::ResourceLimitExceeded { resource, limit: 100_000 }) if resource == "result_values")
        );
        let expression = ExpressionIr::Invoke {
            function: "type.F".to_string(),
            bindings: vec![
                ExpressionBinding {
                    lexical: false,
                    feature: "x".to_string(),
                    expression: range(50_000),
                },
                ExpressionBinding {
                    lexical: false,
                    feature: "y".to_string(),
                    expression: range(50_001),
                },
            ],
            body: Box::new(literal(json!(1))),
        };
        assert!(
            matches!(expression.evaluate(&mut TestEvaluationContext::default()),
            Err(ExpressionEvaluationError::ResourceLimitExceeded { resource, limit: 100_000 }) if resource == "frame_values")
        );
        assert_eq!(
            range(100_000)
                .evaluate_result(&mut TestEvaluationContext::default())
                .unwrap()
                .cardinality(),
            100_000
        );
    }
}
