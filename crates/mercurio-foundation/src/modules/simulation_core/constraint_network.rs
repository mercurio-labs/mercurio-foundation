//! Deterministic, language-neutral scalar constraint networks over shared KIR expressions.
//!
//! This bounded solver accepts affine equalities and explicit given/unknown bindings.
//! It does not parse source, infer variables, solve nonlinear equations, or assign units.
//! Given-only subexpressions and final residuals use the shared expression evaluator.
//! Unknown-dependent expressions are lowered structurally, never by numeric probing.
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::kir::{
    BinaryExpressionOp, ExpressionEvaluationContext, ExpressionEvaluationError, ExpressionIr,
    ExpressionPathSegment, UnaryExpressionOp,
};

/// Binding keys are the dot-joined path segment names in the input expressions.
/// Names and equation IDs must be nonempty and unique in their respective lists.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstraintNetworkRequest {
    #[serde(default)]
    pub knowns: BTreeMap<String, f64>,
    #[serde(default)]
    pub unknowns: Vec<String>,
    pub equations: Vec<ConstraintEquation>,
    #[serde(default = "default_tolerance")]
    pub tolerance: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstraintEquation {
    pub id: String,
    pub left: ExpressionIr,
    pub right: ExpressionIr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintNetworkStatus {
    Solved,
    Underdetermined,
    Inconsistent,
    Unsupported,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintNetworkDiagnosticCode {
    InvalidInput,
    MissingBinding,
    InvalidExpression,
    NonlinearExpression,
    UnsupportedExpression,
    NumericalFailure,
    Underdetermined,
    Inconsistent,
    ResidualExceeded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstraintNetworkDiagnostic {
    pub code: ConstraintNetworkDiagnosticCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equation_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstraintEquationResidual {
    pub equation_id: String,
    pub left: f64,
    pub right: f64,
    /// Signed left minus right.
    pub residual: f64,
    pub within_tolerance: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstraintNetworkResult {
    pub status: ConstraintNetworkStatus,
    /// Given values plus inferred values only when the whole network is solved.
    /// No guessed or partial assignments escape an unsuccessful solve.
    pub values: BTreeMap<String, f64>,
    pub residuals: Vec<ConstraintEquationResidual>,
    pub diagnostics: Vec<ConstraintNetworkDiagnostic>,
    pub rank: usize,
    pub unknown_count: usize,
    /// Invalid input tolerances are reported as None to keep the result JSON-safe.
    pub tolerance: Option<f64>,
}

fn default_tolerance() -> f64 {
    1.0e-9
}

fn diagnostic(
    code: ConstraintNetworkDiagnosticCode,
    message: impl Into<String>,
    equation_id: Option<&str>,
) -> ConstraintNetworkDiagnostic {
    ConstraintNetworkDiagnostic {
        code,
        message: message.into(),
        equation_id: equation_id.map(str::to_owned),
    }
}

/// Solve simultaneous affine equalities using deterministic scaled partial pivoting.
///
/// Equation IDs and unknown names are sorted before elimination; changing request
/// order therefore does not change the result. Coefficients are normalized per row
/// before pivoting. `tolerance` must be finite and strictly between zero and one;
/// it is the pivot threshold and the absolute + relative residual bound:
/// `abs(left - right) <= tolerance * (1 + max(abs(left), abs(right)))`.
/// Near-singular systems can consequently be reported as underdetermined at the
/// requested tolerance. Dimensions are bounded to 256 unknowns and 2048 equations.
/// The function is pure, has no host dependencies, and is safe for WASM callers.
pub fn solve_constraint_network(request: &ConstraintNetworkRequest) -> ConstraintNetworkResult {
    let tolerance_valid =
        request.tolerance.is_finite() && request.tolerance > 0.0 && request.tolerance < 1.0;
    let mut result = ConstraintNetworkResult {
        status: ConstraintNetworkStatus::Invalid,
        values: request
            .knowns
            .iter()
            .filter(|(_, value)| value.is_finite())
            .map(|(name, value)| (name.clone(), *value))
            .collect(),
        residuals: Vec::new(),
        diagnostics: Vec::new(),
        rank: 0,
        unknown_count: request.unknowns.len(),
        tolerance: tolerance_valid.then_some(request.tolerance),
    };
    let mut invalid = |message: String| {
        result.diagnostics.push(diagnostic(
            ConstraintNetworkDiagnosticCode::InvalidInput,
            message,
            None,
        ));
    };
    if !tolerance_valid {
        invalid("tolerance must be finite and strictly between zero and one".into());
    }
    if request.unknowns.len() > 256 || request.equations.len() > 2048 {
        invalid("constraint network exceeds 256 unknowns or 2048 equations".into());
    }
    let mut names = BTreeSet::new();
    for name in &request.unknowns {
        if name.trim().is_empty() || !names.insert(name.as_str()) {
            invalid(format!("unknown binding `{name}` is empty or duplicated"));
        }
        if request.knowns.contains_key(name) {
            invalid(format!("binding `{name}` is both given and unknown"));
        }
    }
    for (name, value) in &request.knowns {
        if name.trim().is_empty() || !value.is_finite() {
            invalid(format!(
                "given binding `{name}` must have a name and finite scalar value"
            ));
        }
    }
    let mut equation_ids = BTreeSet::new();
    for equation in &request.equations {
        if equation.id.trim().is_empty() || !equation_ids.insert(equation.id.as_str()) {
            invalid(format!(
                "equation ID `{}` is empty or duplicated",
                equation.id
            ));
        }
    }
    if !result.diagnostics.is_empty() {
        return result;
    }
    let unknowns = names.into_iter().collect::<Vec<_>>();
    let indices = unknowns
        .iter()
        .enumerate()
        .map(|(index, name)| (*name, index))
        .collect::<BTreeMap<_, _>>();
    let mut equations = request.equations.iter().collect::<Vec<_>>();
    equations.sort_by(|left, right| left.id.cmp(&right.id));
    let mut rows = Vec::new();
    for equation in &equations {
        for expression in [&equation.left, &equation.right] {
            if let Err(error) = expression.validate_runtime_supported() {
                result.diagnostics.push(diagnostic(
                    ConstraintNetworkDiagnosticCode::UnsupportedExpression,
                    error.to_string(),
                    Some(&equation.id),
                ));
            }
        }
        if !result.diagnostics.is_empty() {
            result.status = ConstraintNetworkStatus::Unsupported;
            return result;
        }
        let lower = || -> Result<Affine, ConstraintNetworkDiagnostic> {
            let left = lower_affine(&equation.left, &request.knowns, &indices)?;
            let right = lower_affine(&equation.right, &request.knowns, &indices)?;
            left.combine(right, -1.0)
        };
        match lower() {
            Ok(affine) => rows.push(Row {
                coefficients: affine.coefficients,
                rhs: -affine.constant,
                rhs_scale: 0.0,
            }),
            Err(mut error) => {
                error.equation_id = Some(equation.id.clone());
                result.status = match error.code {
                    ConstraintNetworkDiagnosticCode::NonlinearExpression
                    | ConstraintNetworkDiagnosticCode::UnsupportedExpression => {
                        ConstraintNetworkStatus::Unsupported
                    }
                    _ => ConstraintNetworkStatus::Invalid,
                };
                result.diagnostics.push(error);
                return result;
            }
        }
    }
    let (rank, pivots) = match eliminate(&mut rows, unknowns.len(), request.tolerance) {
        Ok(value) => value,
        Err(error) => {
            result.diagnostics.push(error);
            return result;
        }
    };
    result.rank = rank;
    if rows.iter().any(|row| {
        row.coefficients
            .iter()
            .all(|value| value.abs() <= request.tolerance)
            && !within_scaled_tolerance(row.rhs, row.rhs_scale, request.tolerance)
    }) {
        result.status = ConstraintNetworkStatus::Inconsistent;
        result.diagnostics.push(diagnostic(
            ConstraintNetworkDiagnosticCode::Inconsistent,
            "equalities conflict at the requested tolerance",
            None,
        ));
        return result;
    }
    if rank < unknowns.len() {
        result.status = ConstraintNetworkStatus::Underdetermined;
        result.diagnostics.push(diagnostic(
            ConstraintNetworkDiagnosticCode::Underdetermined,
            format!("network rank {rank} is less than {} unknowns; {} independent equation(s) are missing", unknowns.len(), unknowns.len() - rank),
            None,
        ));
        return result;
    }
    let mut candidate = request.knowns.clone();
    for (row_index, column) in pivots.iter().enumerate() {
        candidate.insert(unknowns[*column].to_owned(), rows[row_index].rhs);
    }
    for equation in equations {
        let evaluate = || -> Result<(f64, f64), ConstraintNetworkDiagnostic> {
            Ok((
                evaluate_number(&equation.left, &candidate)?,
                evaluate_number(&equation.right, &candidate)?,
            ))
        };
        match evaluate() {
            Ok((left, right)) => {
                let residual = left - right;
                if !residual.is_finite() {
                    result.diagnostics.push(diagnostic(
                        ConstraintNetworkDiagnosticCode::NumericalFailure,
                        "nonfinite residual while verifying the candidate solution",
                        Some(&equation.id),
                    ));
                    return result;
                }
                let within_tolerance = within_scaled_tolerance(
                    residual,
                    left.abs().max(right.abs()),
                    request.tolerance,
                );
                result.residuals.push(ConstraintEquationResidual {
                    equation_id: equation.id.clone(),
                    left,
                    right,
                    residual,
                    within_tolerance,
                });
                if !within_tolerance {
                    result.diagnostics.push(diagnostic(
                        ConstraintNetworkDiagnosticCode::ResidualExceeded,
                        "candidate solution does not satisfy the original expression equality at the requested tolerance",
                        Some(&equation.id),
                    ));
                }
            }
            Err(mut error) => {
                error.equation_id = Some(equation.id.clone());
                result.diagnostics.push(error);
                return result;
            }
        }
    }
    if !result.diagnostics.is_empty() {
        return result;
    }
    result.status = ConstraintNetworkStatus::Solved;
    result.values = candidate;
    result
}

// Distributing the tolerance avoids overflow in `1 + scale`.
fn within_scaled_tolerance(residual: f64, scale: f64, tolerance: f64) -> bool {
    residual.abs() <= tolerance + tolerance * scale
}

struct ScalarBindings<'a> {
    values: &'a BTreeMap<String, f64>,
}

fn path_name(segments: &[ExpressionPathSegment]) -> String {
    segments
        .iter()
        .map(ExpressionPathSegment::name)
        .collect::<Vec<_>>()
        .join(".")
}

impl ExpressionEvaluationContext for ScalarBindings<'_> {
    fn owner_id(&self) -> &str {
        "constraint_network"
    }

    fn resolve_path(
        &mut self,
        segments: &[ExpressionPathSegment],
    ) -> Result<Vec<Value>, ExpressionEvaluationError> {
        let name = path_name(segments);
        let value = self
            .values
            .get(&name)
            .ok_or_else(|| ExpressionEvaluationError::MissingBinding(name))?;
        Ok(vec![Value::from(*value)])
    }
}

fn evaluation_diagnostic(error: ExpressionEvaluationError) -> ConstraintNetworkDiagnostic {
    let code = match error {
        ExpressionEvaluationError::MissingBinding(_) => {
            ConstraintNetworkDiagnosticCode::MissingBinding
        }
        ExpressionEvaluationError::NonFiniteResult => {
            ConstraintNetworkDiagnosticCode::NumericalFailure
        }
        _ => ConstraintNetworkDiagnosticCode::InvalidExpression,
    };
    diagnostic(code, error.to_string(), None)
}

fn scalar_number(value: Value) -> Result<f64, ConstraintNetworkDiagnostic> {
    value
        .as_f64()
        .filter(|value| value.is_finite())
        .ok_or_else(|| {
            diagnostic(
                ConstraintNetworkDiagnosticCode::InvalidExpression,
                "constraint expressions must produce one finite numeric scalar",
                None,
            )
        })
}

fn evaluate_number(
    expression: &ExpressionIr,
    values: &BTreeMap<String, f64>,
) -> Result<f64, ConstraintNetworkDiagnostic> {
    scalar_number(
        expression
            .evaluate(&mut ScalarBindings { values })
            .map_err(evaluation_diagnostic)?,
    )
}

#[derive(Clone)]
struct Affine {
    constant: f64,
    coefficients: Vec<f64>,
}

impl Affine {
    fn constant(value: f64, dimensions: usize) -> Self {
        Self {
            constant: value,
            coefficients: vec![0.0; dimensions],
        }
    }

    fn is_constant(&self) -> bool {
        self.coefficients.iter().all(|value| *value == 0.0)
    }

    fn finite(self) -> Result<Self, ConstraintNetworkDiagnostic> {
        if self.constant.is_finite() && self.coefficients.iter().all(|value| value.is_finite()) {
            Ok(self)
        } else {
            Err(diagnostic(
                ConstraintNetworkDiagnosticCode::NumericalFailure,
                "nonfinite value while constructing affine coefficients",
                None,
            ))
        }
    }

    fn combine(
        mut self,
        other: Self,
        multiplier: f64,
    ) -> Result<Self, ConstraintNetworkDiagnostic> {
        self.constant += multiplier * other.constant;
        for (left, right) in self.coefficients.iter_mut().zip(other.coefficients) {
            *left += multiplier * right;
        }
        self.finite()
    }

    fn scale(mut self, multiplier: f64) -> Result<Self, ConstraintNetworkDiagnostic> {
        self.constant *= multiplier;
        for value in &mut self.coefficients {
            *value *= multiplier;
        }
        self.finite()
    }
}

fn lower_affine(
    expression: &ExpressionIr,
    knowns: &BTreeMap<String, f64>,
    unknowns: &BTreeMap<&str, usize>,
) -> Result<Affine, ConstraintNetworkDiagnostic> {
    // Evaluation is solely for bound subexpressions. Missing declared unknowns
    // trigger structural lowering, never substituted trial values.
    match expression.evaluate(&mut ScalarBindings { values: knowns }) {
        Ok(value) => return Ok(Affine::constant(scalar_number(value)?, unknowns.len())),
        Err(ExpressionEvaluationError::MissingBinding(name))
            if unknowns.contains_key(name.as_str()) => {}
        Err(error) => return Err(evaluation_diagnostic(error)),
    }
    let nonlinear = |message: &str| {
        diagnostic(
            ConstraintNetworkDiagnosticCode::NonlinearExpression,
            message,
            None,
        )
    };
    match expression {
        ExpressionIr::Path { segments, .. } => {
            let name = path_name(segments);
            let Some(index) = unknowns.get(name.as_str()) else {
                return Err(diagnostic(
                    ConstraintNetworkDiagnosticCode::MissingBinding,
                    format!("missing binding `{name}`"),
                    None,
                ));
            };
            let mut affine = Affine::constant(0.0, unknowns.len());
            affine.coefficients[*index] = 1.0;
            Ok(affine)
        }
        ExpressionIr::Unary {
            op: UnaryExpressionOp::Negate,
            expr,
        } => lower_affine(expr, knowns, unknowns)?.scale(-1.0),
        ExpressionIr::Binary { left, op, right } => {
            let left = lower_affine(left, knowns, unknowns)?;
            let right = lower_affine(right, knowns, unknowns)?;
            match op {
                BinaryExpressionOp::Add => left.combine(right, 1.0),
                BinaryExpressionOp::Subtract => left.combine(right, -1.0),
                BinaryExpressionOp::Multiply if left.is_constant() => right.scale(left.constant),
                BinaryExpressionOp::Multiply if right.is_constant() => left.scale(right.constant),
                BinaryExpressionOp::Multiply => Err(nonlinear(
                    "a product of unknown-dependent expressions is nonlinear",
                )),
                BinaryExpressionOp::Divide if right.is_constant() && right.constant != 0.0 => {
                    left.scale(1.0 / right.constant)
                }
                BinaryExpressionOp::Divide if right.is_constant() => Err(evaluation_diagnostic(
                    ExpressionEvaluationError::DivisionByZero,
                )),
                BinaryExpressionOp::Divide => Err(nonlinear(
                    "an unknown-dependent denominator is outside the affine solver profile",
                )),
                BinaryExpressionOp::Power => Err(nonlinear(
                    "unknown-dependent powers are outside the affine solver profile",
                )),
                _ => Err(diagnostic(
                    ConstraintNetworkDiagnosticCode::UnsupportedExpression,
                    "unknown-dependent comparisons and Boolean operators cannot form a scalar affine equality",
                    None,
                )),
            }
        }
        // The shared evaluator checks contracts again against the complete final
        // candidate, so a symbolic term never bypasses an expression contract.
        ExpressionIr::Checked { expression, .. } => lower_affine(expression, knowns, unknowns),
        ExpressionIr::Call { .. } | ExpressionIr::Invoke { .. } => Err(diagnostic(
            ConstraintNetworkDiagnosticCode::UnsupportedExpression,
            "unknown-dependent calls must be lowered to affine operators by the language adapter",
            None,
        )),
        _ => Err(diagnostic(
            ConstraintNetworkDiagnosticCode::UnsupportedExpression,
            "expression is outside the scalar affine solver profile",
            None,
        )),
    }
}

struct Row {
    coefficients: Vec<f64>,
    rhs: f64,
    // Tracks right-hand-side scale through elimination to assess dependent rows.
    rhs_scale: f64,
}

fn eliminate(
    rows: &mut [Row],
    dimensions: usize,
    tolerance: f64,
) -> Result<(usize, Vec<usize>), ConstraintNetworkDiagnostic> {
    let failure = || {
        diagnostic(
            ConstraintNetworkDiagnosticCode::NumericalFailure,
            "nonfinite value during equation elimination",
            None,
        )
    };
    for row in rows.iter_mut() {
        let scale = row
            .coefficients
            .iter()
            .map(|value| value.abs())
            .fold(0.0_f64, f64::max);
        if scale > 0.0 {
            for value in &mut row.coefficients {
                *value /= scale;
            }
            row.rhs /= scale;
        }
        row.rhs_scale = row.rhs.abs();
        if !row.rhs.is_finite() {
            return Err(failure());
        }
    }
    let mut pivots = Vec::new();
    for column in 0..dimensions {
        let rank = pivots.len();
        let mut pivot_row = None;
        let mut maximum = tolerance;
        for (index, row) in rows.iter().enumerate().skip(rank) {
            let magnitude = row.coefficients[column].abs();
            if magnitude > maximum {
                maximum = magnitude;
                pivot_row = Some(index);
            }
        }
        let Some(pivot_row) = pivot_row else {
            continue;
        };
        rows.swap(rank, pivot_row);
        let pivot = rows[rank].coefficients[column];
        for value in &mut rows[rank].coefficients {
            *value /= pivot;
        }
        rows[rank].rhs /= pivot;
        rows[rank].rhs_scale /= pivot.abs();
        let pivot_coefficients = rows[rank].coefficients.clone();
        let pivot_rhs = rows[rank].rhs;
        let pivot_scale = rows[rank].rhs_scale;
        for (index, row) in rows.iter_mut().enumerate() {
            if index != rank {
                let factor = row.coefficients[column];
                for (value, pivot_value) in row.coefficients.iter_mut().zip(&pivot_coefficients) {
                    *value -= factor * pivot_value;
                }
                row.coefficients[column] = 0.0;
                row.rhs -= factor * pivot_rhs;
                row.rhs_scale += factor.abs() * pivot_scale;
            }
            if !row.rhs.is_finite()
                || !row.rhs_scale.is_finite()
                || row.coefficients.iter().any(|value| !value.is_finite())
            {
                return Err(failure());
            }
        }
        pivots.push(column);
    }
    Ok((pivots.len(), pivots))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kir::{
        ExpressionContract, ExpressionMultiplicity, ExpressionPathRoot, ExpressionValueType,
    };
    use serde_json::json;

    fn number(value: f64) -> ExpressionIr {
        ExpressionIr::Literal {
            value: json!(value),
        }
    }
    fn symbol(name: &str) -> ExpressionIr {
        ExpressionIr::Path {
            root: ExpressionPathRoot::SelfRef,
            segments: name
                .split('.')
                .map(|part| ExpressionPathSegment::Name(part.into()))
                .collect(),
        }
    }
    fn binary(left: ExpressionIr, op: BinaryExpressionOp, right: ExpressionIr) -> ExpressionIr {
        ExpressionIr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
    fn add(left: ExpressionIr, right: ExpressionIr) -> ExpressionIr {
        binary(left, BinaryExpressionOp::Add, right)
    }
    fn subtract(left: ExpressionIr, right: ExpressionIr) -> ExpressionIr {
        binary(left, BinaryExpressionOp::Subtract, right)
    }
    fn multiply(left: ExpressionIr, right: ExpressionIr) -> ExpressionIr {
        binary(left, BinaryExpressionOp::Multiply, right)
    }
    fn equation(id: &str, left: ExpressionIr, right: ExpressionIr) -> ConstraintEquation {
        ConstraintEquation {
            id: id.into(),
            left,
            right,
        }
    }
    fn request(unknowns: &[&str], equations: Vec<ConstraintEquation>) -> ConstraintNetworkRequest {
        ConstraintNetworkRequest {
            knowns: BTreeMap::new(),
            unknowns: unknowns.iter().map(|name| (*name).into()).collect(),
            equations,
            tolerance: default_tolerance(),
        }
    }
    fn coupled() -> ConstraintNetworkRequest {
        request(
            &["y", "x"],
            vec![
                equation("sum", add(symbol("x"), symbol("y")), number(10.0)),
                equation(
                    "difference",
                    subtract(symbol("x"), symbol("y")),
                    number(2.0),
                ),
            ],
        )
    }

    #[test]
    fn simultaneous_equations_produce_a_unique_solution_and_original_residuals() {
        let result = solve_constraint_network(&coupled());
        assert_eq!(result.status, ConstraintNetworkStatus::Solved);
        assert_eq!(result.rank, 2);
        assert_eq!(
            result.values,
            BTreeMap::from([("x".into(), 6.0), ("y".into(), 4.0)])
        );
        assert_eq!(result.residuals.len(), 2);
        assert!(
            result
                .residuals
                .iter()
                .all(|row| row.within_tolerance && row.residual == 0.0)
        );
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn pivoting_avoids_tiny_leading_coefficients_and_checks_tolerance_near_zero() {
        let input = request(
            &["x", "y"],
            vec![
                equation(
                    "a",
                    add(multiply(number(1.0e-12), symbol("x")), symbol("y")),
                    number(1.0),
                ),
                equation("b", add(symbol("x"), symbol("y")), number(2.0)),
            ],
        );
        let result = solve_constraint_network(&input);
        assert_eq!(result.status, ConstraintNetworkStatus::Solved);
        assert!((result.values["x"] - 1.0).abs() < 1.0e-9);
        assert!(result.residuals.iter().all(|row| row.within_tolerance));
        let accepted = request(
            &[],
            vec![equation("near_zero", number(0.0), number(0.9e-9))],
        );
        assert_eq!(
            solve_constraint_network(&accepted).status,
            ConstraintNetworkStatus::Solved
        );
        let rejected = request(
            &[],
            vec![equation("near_zero", number(0.0), number(1.1e-9))],
        );
        assert_eq!(
            solve_constraint_network(&rejected).status,
            ConstraintNetworkStatus::Inconsistent
        );
    }

    #[test]
    fn reverse_solving_rebinds_any_scalar_as_unknown() {
        let mut input = request(
            &["vehicle.mass"],
            vec![equation(
                "force",
                symbol("force"),
                multiply(symbol("vehicle.mass"), symbol("acceleration")),
            )],
        );
        input.knowns = BTreeMap::from([("force".into(), 60.0), ("acceleration".into(), 3.0)]);
        let result = solve_constraint_network(&input);
        assert_eq!(result.status, ConstraintNetworkStatus::Solved);
        assert_eq!(result.values["vehicle.mass"], 20.0);
        input.unknowns = vec!["force".into()];
        input.knowns.remove("force");
        input.knowns.insert("vehicle.mass".into(), 20.0);
        assert_eq!(solve_constraint_network(&input).values["force"], 60.0);
    }

    #[test]
    fn canonical_order_is_independent_of_equation_and_unknown_order() {
        let mut input = coupled();
        let first = solve_constraint_network(&input);
        input.equations.reverse();
        input.unknowns.reverse();
        assert_eq!(first, solve_constraint_network(&input));
        let serialized = serde_json::to_value(&input).unwrap();
        let restored: ConstraintNetworkRequest = serde_json::from_value(serialized).unwrap();
        assert_eq!(first, solve_constraint_network(&restored));
    }

    #[test]
    fn redundant_equations_and_small_coefficients_do_not_block_unique_solution() {
        let mut input = coupled();
        input.equations.push(equation(
            "redundant",
            multiply(number(2.0), add(symbol("x"), symbol("y"))),
            number(20.0),
        ));
        assert_eq!(
            solve_constraint_network(&input).status,
            ConstraintNetworkStatus::Solved
        );
        let tiny = request(
            &["x"],
            vec![equation(
                "tiny",
                multiply(number(1.0e-20), symbol("x")),
                number(1.0),
            )],
        );
        let result = solve_constraint_network(&tiny);
        assert_eq!(result.status, ConstraintNetworkStatus::Solved);
        assert_eq!(result.values["x"], 1.0e20);
    }

    #[test]
    fn inconsistent_and_rank_deficient_networks_never_leak_partial_assignments() {
        let mut input = request(
            &["x", "y"],
            vec![equation("sum", add(symbol("x"), symbol("y")), number(10.0))],
        );
        let result = solve_constraint_network(&input);
        assert_eq!(result.status, ConstraintNetworkStatus::Underdetermined);
        assert_eq!(result.rank, 1);
        assert!(result.values.is_empty());
        input.equations.push(equation(
            "conflict",
            add(symbol("x"), symbol("y")),
            number(12.0),
        ));
        let result = solve_constraint_network(&input);
        assert_eq!(result.status, ConstraintNetworkStatus::Inconsistent);
        assert!(result.values.is_empty());
        assert_eq!(
            result.diagnostics[0].code,
            ConstraintNetworkDiagnosticCode::Inconsistent
        );
    }

    #[test]
    fn nonlinear_polynomials_cannot_masquerade_as_affine_from_sample_values() {
        // This polynomial is zero at 0, 1, and 2, defeating sample-based detection.
        let polynomial = multiply(
            multiply(symbol("x"), subtract(symbol("x"), number(1.0))),
            subtract(symbol("x"), number(2.0)),
        );
        let input = request(
            &["x"],
            vec![equation("polynomial", polynomial, number(0.0))],
        );
        let result = solve_constraint_network(&input);
        assert_eq!(result.status, ConstraintNetworkStatus::Unsupported);
        assert_eq!(
            result.diagnostics[0].code,
            ConstraintNetworkDiagnosticCode::NonlinearExpression
        );
        assert_eq!(
            result.diagnostics[0].equation_id.as_deref(),
            Some("polynomial")
        );
    }

    #[test]
    fn unknown_denominators_and_powers_are_explicitly_unsupported() {
        for expr in [
            binary(number(1.0), BinaryExpressionOp::Divide, symbol("x")),
            binary(symbol("x"), BinaryExpressionOp::Power, number(2.0)),
        ] {
            let result = solve_constraint_network(&request(
                &["x"],
                vec![equation("nonlinear", expr, number(2.0))],
            ));
            assert_eq!(result.status, ConstraintNetworkStatus::Unsupported);
            assert_eq!(
                result.diagnostics[0].code,
                ConstraintNetworkDiagnosticCode::NonlinearExpression
            );
        }
    }

    #[test]
    fn given_only_calls_and_powers_use_shared_expression_semantics() {
        let coefficient = ExpressionIr::Call {
            function: "max".into(),
            args: vec![
                symbol("a"),
                binary(number(2.0), BinaryExpressionOp::Power, number(3.0)),
            ],
        };
        let mut input = request(
            &["x"],
            vec![equation(
                "shared",
                multiply(coefficient, symbol("x")),
                number(32.0),
            )],
        );
        input.knowns.insert("a".into(), 3.0);
        assert_eq!(solve_constraint_network(&input).values["x"], 4.0);
    }

    #[test]
    fn missing_bindings_invalid_arithmetic_and_nonscalars_are_diagnostic() {
        for (expr, expected) in [
            (
                add(symbol("x"), symbol("missing")),
                ConstraintNetworkDiagnosticCode::MissingBinding,
            ),
            (
                binary(symbol("x"), BinaryExpressionOp::Divide, number(0.0)),
                ConstraintNetworkDiagnosticCode::InvalidExpression,
            ),
            (
                ExpressionIr::Literal {
                    value: json!("string"),
                },
                ConstraintNetworkDiagnosticCode::InvalidExpression,
            ),
            (
                ExpressionIr::Tuple {
                    items: vec![number(1.0), number(2.0)],
                },
                ConstraintNetworkDiagnosticCode::InvalidExpression,
            ),
            (
                multiply(number(1.0e308), number(1.0e308)),
                ConstraintNetworkDiagnosticCode::NumericalFailure,
            ),
        ] {
            let result = solve_constraint_network(&request(
                &["x"],
                vec![equation("bad", expr, number(0.0))],
            ));
            assert_eq!(result.status, ConstraintNetworkStatus::Invalid);
            assert_eq!(result.diagnostics[0].code, expected);
        }
    }

    #[test]
    fn candidate_verification_enforces_checked_contracts() {
        let checked = ExpressionIr::Checked {
            expression: Box::new(symbol("x")),
            contract: ExpressionContract {
                value_type: ExpressionValueType::Boolean,
                multiplicity: Some(ExpressionMultiplicity {
                    lower: 1,
                    upper: Some(1),
                }),
            },
        };
        let result = solve_constraint_network(&request(
            &["x"],
            vec![equation("contract", checked, number(1.0))],
        ));
        assert_eq!(result.status, ConstraintNetworkStatus::Invalid);
        assert_eq!(
            result.diagnostics[0].code,
            ConstraintNetworkDiagnosticCode::InvalidExpression
        );
        assert!(!result.values.contains_key("x"));
    }

    #[test]
    fn invalid_inputs_remain_json_serializable_and_report_invalid_status() {
        for tolerance in [0.0, -1.0, f64::NAN, f64::INFINITY, 1.0] {
            let mut input = coupled();
            input.tolerance = tolerance;
            let result = solve_constraint_network(&input);
            assert_eq!(result.status, ConstraintNetworkStatus::Invalid);
            assert_eq!(result.tolerance, None);
            assert!(serde_json::to_string(&result).is_ok());
        }
        let mut input = coupled();
        input.knowns.insert("bad".into(), f64::INFINITY);
        let result = solve_constraint_network(&input);
        assert_eq!(result.status, ConstraintNetworkStatus::Invalid);
        assert!(!result.values.contains_key("bad"));
        let mut input = coupled();
        input.unknowns.push("x".into());
        input.knowns.insert("y".into(), 1.0);
        input.equations[1].id = input.equations[0].id.clone();
        let result = solve_constraint_network(&input);
        assert_eq!(result.diagnostics.len(), 3);
    }

    #[test]
    fn empty_and_fully_bound_networks_have_explicit_verdicts() {
        assert_eq!(
            solve_constraint_network(&request(&[], vec![])).status,
            ConstraintNetworkStatus::Solved
        );
        assert_eq!(
            solve_constraint_network(&request(&["x"], vec![])).status,
            ConstraintNetworkStatus::Underdetermined
        );
        assert_eq!(
            solve_constraint_network(&request(
                &[],
                vec![equation("valid", number(2.0), number(2.0))]
            ))
            .status,
            ConstraintNetworkStatus::Solved
        );
        assert_eq!(
            solve_constraint_network(&request(
                &[],
                vec![equation("invalid", number(2.0), number(3.0))]
            ))
            .status,
            ConstraintNetworkStatus::Inconsistent
        );
    }

    #[test]
    fn cancelling_linear_terms_and_division_by_given_scalar_are_supported() {
        let expr = binary(
            add(subtract(symbol("x"), symbol("x")), symbol("y")),
            BinaryExpressionOp::Divide,
            number(2.0),
        );
        let input = request(
            &["x", "y"],
            vec![
                equation("cancel", expr, number(3.0)),
                equation("x", symbol("x"), number(9.0)),
            ],
        );
        let result = solve_constraint_network(&input);
        assert_eq!(result.status, ConstraintNetworkStatus::Solved);
        assert_eq!(result.values["y"], 6.0);
    }
}
