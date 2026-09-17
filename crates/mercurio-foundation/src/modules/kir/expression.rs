use std::cmp::Ordering;
use std::collections::BTreeSet;
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
    UnsupportedPathRoot(String),
    EmptyPath,
    UnsupportedFunction(String),
    InvalidFunctionArity { function: String, arity: usize },
}

impl fmt::Display for ExpressionValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            "literal" | "self" | "tuple" | "path" | "unary" | "binary" | "call"
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
        // Validate every branch, but evaluate only the branches selected by control operators.
        self.validate_runtime_supported()
            .map_err(|err| ExpressionEvaluationError::InvalidExpression(err.to_string()))?;
        self.evaluate_values(context).map(result_value)
    }

    // Result sequences are separate from JSON data values: a literal array is one value,
    // whereas Tuple concatenates the results of its operands. Null denotes no result.
    fn evaluate_values(
        &self,
        context: &mut impl ExpressionEvaluationContext,
    ) -> Result<Vec<Value>, ExpressionEvaluationError> {
        match self {
            Self::Literal { value: Value::Null } => Ok(Vec::new()),
            Self::Literal { value } => Ok(vec![value.clone()]),
            Self::SelfRef => Ok(vec![Value::String(context.owner_id().to_string())]),
            Self::Tuple { items } => {
                let mut values = Vec::new();
                for item in items {
                    values.extend(item.evaluate_values(context)?);
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
        context: &mut impl ExpressionEvaluationContext,
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
}

#[derive(Debug)]
pub enum ExpressionEvaluationError {
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
    context: &mut impl ExpressionEvaluationContext,
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
        BinaryExpressionOp, ExpressionEvaluationContext, ExpressionEvaluationError, ExpressionIr,
        ExpressionPathRoot, ExpressionPathSegment, UnaryExpressionOp,
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
}
