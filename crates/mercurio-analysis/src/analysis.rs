use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::capability::SemanticElementRef;
use mercurio_model::{Element, Graph};

pub type AnalysisElementRef = SemanticElementRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisTechniqueKind {
    Query,
    Calculation,
    ConstraintEvaluation,
    Simulation,
    Verification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisWorkflowStepKind {
    ScopeSubject,
    BindAssumptions,
    ResolveInputs,
    ExecuteTechnique,
    EvaluateConstraints,
    EvaluateRequirements,
    ProduceViews,
    RecordResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisWorkflowStep {
    pub kind: AnalysisWorkflowStepKind,
    pub label: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub elements: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub techniques: Vec<AnalysisTechniqueKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisWorkflow {
    pub case_id: String,
    pub steps: Vec<AnalysisWorkflowStep>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisCaseModel {
    pub element: AnalysisElementRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assumptions: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub calculations: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirements: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verification_cases: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub simulations: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub views: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concerns: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub techniques: Vec<AnalysisTechniqueKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequirementEvaluationModel {
    pub requirement: AnalysisElementRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub formal_constraints: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verification_cases: Vec<AnalysisElementRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisInventory {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub analysis_cases: Vec<AnalysisCaseModel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub calculation_definitions: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub calculation_usages: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraint_definitions: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraint_usages: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirement_evaluations: Vec<RequirementEvaluationModel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verification_cases: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub views: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concerns: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub simulations: Vec<AnalysisElementRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisOpportunityKind {
    AnalysisCase,
    ConstraintEvaluation,
    RequirementEvaluation,
    Simulation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalysisOpportunity {
    pub id: String,
    pub kind: AnalysisOpportunityKind,
    pub label: String,
    pub description: String,
    pub runnable: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub elements: Vec<AnalysisElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub techniques: Vec<AnalysisTechniqueKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_hint: Option<String>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub metadata: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalysisOpportunityReport {
    pub schema: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub opportunities: Vec<AnalysisOpportunity>,
}

impl AnalysisInventory {
    pub fn from_graph(graph: &Graph) -> Self {
        let calculation_definitions =
            collect_by_kind(graph, |kind| kind_contains(kind, "calculationdefinition"));
        let calculation_usages =
            collect_by_kind(graph, |kind| kind_contains(kind, "calculationusage"));
        let constraint_definitions =
            collect_by_kind(graph, |kind| kind_contains(kind, "constraintdefinition"));
        let constraint_usages = collect_by_kind(graph, is_constraint_usage_kind);
        let verification_cases = collect_by_kind(graph, is_verification_case_kind);
        let views = collect_by_kind(graph, is_view_kind);
        let concerns = collect_by_kind(graph, is_concern_or_viewpoint_kind);
        let simulations = collect_by_kind(graph, is_simulation_kind);

        let analysis_cases = graph
            .elements()
            .iter()
            .filter(|element| is_authored_model_element(element))
            .filter(|element| is_analysis_case_kind(element.kind.as_ref()))
            .map(|element| AnalysisCaseModel::from_element(graph, element))
            .collect();

        let requirement_evaluations = graph
            .elements()
            .iter()
            .filter(|element| is_authored_model_element(element))
            .filter(|element| is_requirement_kind(element.kind.as_ref()))
            .map(|element| requirement_evaluation_from_element(graph, element))
            .collect();

        Self {
            analysis_cases,
            calculation_definitions,
            calculation_usages,
            constraint_definitions,
            constraint_usages,
            requirement_evaluations,
            verification_cases,
            views,
            concerns,
            simulations,
        }
    }

    pub fn opportunities(&self) -> AnalysisOpportunityReport {
        let mut opportunities = Vec::new();

        for case in &self.analysis_cases {
            opportunities.push(AnalysisOpportunity {
                id: format!("analysis_case.{}", case.element.element_id),
                kind: AnalysisOpportunityKind::AnalysisCase,
                label: format!(
                    "Run analysis case {}",
                    case.element
                        .label
                        .as_deref()
                        .unwrap_or(case.element.element_id.as_str())
                ),
                description: "Authored analysis case with a semantic execution workflow."
                    .to_string(),
                runnable: true,
                elements: vec![case.element.clone()],
                techniques: case.techniques.clone(),
                capability_id: Some("sysml.analysis.case".to_string()),
                action_id: Some("run_analysis_case".to_string()),
                route_hint: Some("/api/analysis/cases/run".to_string()),
                metadata: serde_json::Map::from_iter([
                    ("caseId".to_string(), json!(case.element.element_id)),
                    ("workflow".to_string(), json!(case.workflow())),
                    ("subjectCount".to_string(), json!(case.subjects.len())),
                    ("constraintCount".to_string(), json!(case.constraints.len())),
                    (
                        "requirementCount".to_string(),
                        json!(case.requirements.len()),
                    ),
                    ("simulationCount".to_string(), json!(case.simulations.len())),
                ]),
            });
        }

        if !self.constraint_usages.is_empty() || !self.constraint_definitions.is_empty() {
            let elements = if self.constraint_usages.is_empty() {
                self.constraint_definitions.clone()
            } else {
                self.constraint_usages.clone()
            };
            opportunities.push(AnalysisOpportunity {
                id: "constraint_evaluation.workspace".to_string(),
                kind: AnalysisOpportunityKind::ConstraintEvaluation,
                label: "Evaluate model constraints".to_string(),
                description: "Constraint definitions or usages are present and can be evaluated against bound model values.".to_string(),
                runnable: !self.constraint_usages.is_empty(),
                elements,
                techniques: vec![AnalysisTechniqueKind::ConstraintEvaluation],
                capability_id: Some("sysml.constraint.analysis".to_string()),
                action_id: Some("solve_constraints".to_string()),
                route_hint: Some("/api/constraints/solve".to_string()),
                metadata: serde_json::Map::from_iter([
                    (
                        "constraintDefinitionCount".to_string(),
                        json!(self.constraint_definitions.len()),
                    ),
                    (
                        "constraintUsageCount".to_string(),
                        json!(self.constraint_usages.len()),
                    ),
                ]),
            });
        }

        let requirement_evaluations = self
            .requirement_evaluations
            .iter()
            .filter(|evaluation| {
                !evaluation.formal_constraints.is_empty()
                    || !evaluation.verification_cases.is_empty()
            })
            .collect::<Vec<_>>();
        if !requirement_evaluations.is_empty() {
            opportunities.push(AnalysisOpportunity {
                id: "requirement_evaluation.workspace".to_string(),
                kind: AnalysisOpportunityKind::RequirementEvaluation,
                label: "Evaluate requirements".to_string(),
                description: "Requirements have formal constraints or verification cases that can produce satisfaction evidence.".to_string(),
                runnable: true,
                elements: requirement_evaluations
                    .iter()
                    .map(|evaluation| evaluation.requirement.clone())
                    .collect(),
                techniques: vec![AnalysisTechniqueKind::Verification],
                capability_id: Some("sysml.requirement.analysis".to_string()),
                action_id: Some("analyze_requirement_coverage".to_string()),
                route_hint: Some("/api/reasoning/capabilities/sysml.requirement.analysis/run".to_string()),
                metadata: serde_json::Map::from_iter([(
                    "requirementCount".to_string(),
                    json!(requirement_evaluations.len()),
                )]),
            });
        }

        if !self.simulations.is_empty() {
            opportunities.push(AnalysisOpportunity {
                id: "simulation.workspace".to_string(),
                kind: AnalysisOpportunityKind::Simulation,
                label: "Explore executable behavior".to_string(),
                description: "State or behavior elements are present and may support dynamic behavior analysis or simulation.".to_string(),
                runnable: true,
                elements: self.simulations.clone(),
                techniques: vec![AnalysisTechniqueKind::Simulation],
                capability_id: Some("sysml.behavior.dynamic".to_string()),
                action_id: Some("analyze_state_machine".to_string()),
                route_hint: Some("/api/reasoning/capabilities/sysml.behavior.dynamic/run".to_string()),
                metadata: serde_json::Map::from_iter([(
                    "simulationElementCount".to_string(),
                    json!(self.simulations.len()),
                )]),
            });
        }

        AnalysisOpportunityReport {
            schema: "mercurio.analysis.opportunities.v1".to_string(),
            opportunities,
        }
    }
}

impl AnalysisCaseModel {
    fn from_element(graph: &Graph, element: &Element) -> Self {
        let calculations = related_elements(graph, element, &["calculation", "calculations"])
            .into_iter()
            .chain(owned_elements_matching(
                graph,
                element,
                is_calculation_usage_kind,
            ))
            .collect::<Vec<_>>();
        let constraints = related_elements(graph, element, &["constraint", "constraints"])
            .into_iter()
            .chain(owned_elements_matching(
                graph,
                element,
                is_constraint_usage_kind,
            ))
            .collect::<Vec<_>>();
        let requirements = related_elements(graph, element, &["requirement", "requirements"]);
        let verification_cases = related_elements(
            graph,
            element,
            &["verification_case", "verification_cases", "verification"],
        );
        let simulations = related_elements(graph, element, &["simulation", "simulations"])
            .into_iter()
            .chain(owned_elements_matching(graph, element, is_simulation_kind))
            .collect::<Vec<_>>();
        let views = related_elements(graph, element, &["view", "views"]);
        let concerns = related_elements(
            graph,
            element,
            &["concern", "concerns", "viewpoint", "viewpoints"],
        );

        let mut techniques = BTreeSet::new();
        if !calculations.is_empty() {
            techniques.insert(AnalysisTechniqueKind::Calculation);
        }
        if !constraints.is_empty() {
            techniques.insert(AnalysisTechniqueKind::ConstraintEvaluation);
        }
        if !simulations.is_empty() {
            techniques.insert(AnalysisTechniqueKind::Simulation);
        }
        if !requirements.is_empty() || !verification_cases.is_empty() {
            techniques.insert(AnalysisTechniqueKind::Verification);
        }
        if has_text_property(element, &["query", "dsl", "script"]) {
            techniques.insert(AnalysisTechniqueKind::Query);
        }

        Self {
            element: element_ref(element),
            subjects: related_elements(graph, element, &["subject", "subjects"]),
            assumptions: related_elements(graph, element, &["assumption", "assumptions"]),
            inputs: related_elements(graph, element, &["input", "inputs"]),
            outputs: related_elements(graph, element, &["output", "outputs"]),
            calculations: dedup_refs(calculations),
            constraints: dedup_refs(constraints),
            requirements,
            verification_cases,
            simulations: dedup_refs(simulations),
            views,
            concerns,
            techniques: techniques.into_iter().collect(),
        }
    }

    pub fn workflow(&self) -> AnalysisWorkflow {
        let mut steps = Vec::new();
        steps.push(AnalysisWorkflowStep {
            kind: AnalysisWorkflowStepKind::ScopeSubject,
            label: "Scope analysis subject".to_string(),
            elements: self.subjects.clone(),
            techniques: Vec::new(),
        });

        if !self.assumptions.is_empty() {
            steps.push(AnalysisWorkflowStep {
                kind: AnalysisWorkflowStepKind::BindAssumptions,
                label: "Bind assumptions".to_string(),
                elements: self.assumptions.clone(),
                techniques: Vec::new(),
            });
        }

        if !(self.inputs.is_empty() && self.outputs.is_empty()) {
            let mut elements = self.inputs.clone();
            elements.extend(self.outputs.clone());
            steps.push(AnalysisWorkflowStep {
                kind: AnalysisWorkflowStepKind::ResolveInputs,
                label: "Resolve inputs and expected outputs".to_string(),
                elements,
                techniques: Vec::new(),
            });
        }

        let execution_elements = self
            .calculations
            .iter()
            .chain(&self.simulations)
            .cloned()
            .collect::<Vec<_>>();
        if !execution_elements.is_empty() {
            steps.push(AnalysisWorkflowStep {
                kind: AnalysisWorkflowStepKind::ExecuteTechnique,
                label: "Execute calculations and simulations".to_string(),
                elements: execution_elements,
                techniques: self
                    .techniques
                    .iter()
                    .copied()
                    .filter(|technique| {
                        matches!(
                            technique,
                            AnalysisTechniqueKind::Calculation
                                | AnalysisTechniqueKind::Simulation
                                | AnalysisTechniqueKind::Query
                        )
                    })
                    .collect(),
            });
        }

        if !self.constraints.is_empty() {
            steps.push(AnalysisWorkflowStep {
                kind: AnalysisWorkflowStepKind::EvaluateConstraints,
                label: "Evaluate constraints".to_string(),
                elements: self.constraints.clone(),
                techniques: vec![AnalysisTechniqueKind::ConstraintEvaluation],
            });
        }

        if !(self.requirements.is_empty() && self.verification_cases.is_empty()) {
            let mut elements = self.requirements.clone();
            elements.extend(self.verification_cases.clone());
            steps.push(AnalysisWorkflowStep {
                kind: AnalysisWorkflowStepKind::EvaluateRequirements,
                label: "Evaluate requirements and verification cases".to_string(),
                elements,
                techniques: vec![AnalysisTechniqueKind::Verification],
            });
        }

        if !(self.views.is_empty() && self.concerns.is_empty()) {
            let mut elements = self.views.clone();
            elements.extend(self.concerns.clone());
            steps.push(AnalysisWorkflowStep {
                kind: AnalysisWorkflowStepKind::ProduceViews,
                label: "Produce stakeholder views".to_string(),
                elements,
                techniques: Vec::new(),
            });
        }

        steps.push(AnalysisWorkflowStep {
            kind: AnalysisWorkflowStepKind::RecordResult,
            label: "Record analysis result and evidence".to_string(),
            elements: vec![self.element.clone()],
            techniques: self.techniques.clone(),
        });

        AnalysisWorkflow {
            case_id: self.element.element_id.clone(),
            steps,
        }
    }
}

fn requirement_evaluation_from_element(
    graph: &Graph,
    element: &Element,
) -> RequirementEvaluationModel {
    let formal_constraints = related_elements(
        graph,
        element,
        &[
            "constraint",
            "constraints",
            "formal_constraint",
            "formal_constraints",
        ],
    )
    .into_iter()
    .chain(owned_elements_matching(
        graph,
        element,
        is_constraint_usage_kind,
    ))
    .collect::<Vec<_>>();

    RequirementEvaluationModel {
        requirement: element_ref(element),
        formal_constraints: dedup_refs(formal_constraints),
        verification_cases: related_elements(
            graph,
            element,
            &["verification_case", "verification_cases", "verification"],
        ),
    }
}

fn collect_by_kind<F>(graph: &Graph, predicate: F) -> Vec<AnalysisElementRef>
where
    F: Fn(&str) -> bool,
{
    graph
        .elements()
        .iter()
        .filter(|element| is_authored_model_element(element))
        .filter(|element| predicate(element.kind.as_ref()))
        .map(element_ref)
        .collect()
}

fn related_elements(
    graph: &Graph,
    element: &Element,
    properties: &[&str],
) -> Vec<AnalysisElementRef> {
    let mut refs = Vec::new();
    for property in properties {
        if let Some(value) = element.properties.get(*property) {
            collect_refs_from_value(graph, value, &mut refs);
        }
    }
    dedup_refs(refs)
}

fn collect_refs_from_value(graph: &Graph, value: &Value, refs: &mut Vec<AnalysisElementRef>) {
    match value {
        Value::String(element_id) => {
            if let Some(element) = graph.element_by_element_id(element_id) {
                refs.push(element_ref(element));
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_refs_from_value(graph, value, refs);
            }
        }
        _ => {}
    }
}

fn owned_elements_matching<F>(
    graph: &Graph,
    owner: &Element,
    predicate: F,
) -> Vec<AnalysisElementRef>
where
    F: Fn(&str) -> bool,
{
    graph
        .incoming(owner.id, "owner")
        .filter_map(|edge| graph.element(edge.source))
        .filter(|element| predicate(element.kind.as_ref()))
        .map(element_ref)
        .collect()
}

fn element_ref(element: &Element) -> AnalysisElementRef {
    AnalysisElementRef::from_graph_element(element)
}

fn string_property(element: &Element, name: &str) -> Option<String> {
    element
        .properties
        .get(name)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn has_text_property(element: &Element, names: &[&str]) -> bool {
    names
        .iter()
        .any(|name| string_property(element, name).is_some())
}

fn dedup_refs(refs: Vec<AnalysisElementRef>) -> Vec<AnalysisElementRef> {
    let mut seen = BTreeSet::new();
    refs.into_iter()
        .filter(|reference| seen.insert(reference.element_id.clone()))
        .collect()
}

fn is_analysis_case_kind(kind: &str) -> bool {
    kind_contains(kind, "analysiscase")
}

fn is_calculation_usage_kind(kind: &str) -> bool {
    kind_contains(kind, "calculationusage")
}

fn is_constraint_usage_kind(kind: &str) -> bool {
    kind_contains(kind, "constraintusage")
        || canonical_kind(kind) == "constraint"
        || kind_contains(kind, "assertconstraintusage")
}

fn is_requirement_kind(kind: &str) -> bool {
    kind_contains(kind, "requirement")
}

fn is_verification_case_kind(kind: &str) -> bool {
    kind_contains(kind, "verificationcase")
}

fn is_view_kind(kind: &str) -> bool {
    let kind = canonical_kind(kind);
    kind == "view" || kind.contains("viewusage") || kind.contains("viewdefinition")
}

fn is_concern_or_viewpoint_kind(kind: &str) -> bool {
    kind_contains(kind, "concern") || kind_contains(kind, "viewpoint")
}

fn is_simulation_kind(kind: &str) -> bool {
    kind_contains(kind, "simulation")
        || kind_contains(kind, "stateusage")
        || kind_contains(kind, "statedefinition")
        || kind_contains(kind, "behavior")
}

fn kind_contains(kind: &str, needle: &str) -> bool {
    canonical_kind(kind).contains(needle)
}

fn is_authored_model_element(element: &Element) -> bool {
    element.layer >= 2
}

fn canonical_kind(kind: &str) -> String {
    kind.replace([':', '.', ' ', '_'], "").to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mercurio_kir::{KirDocument, KirElement};
    use mercurio_model::Graph;
    use serde_json::json;
    use std::collections::BTreeMap;

    fn element(id: &str, kind: &str, properties: BTreeMap<String, Value>) -> KirElement {
        KirElement {
            id: id.to_string(),
            kind: kind.to_string(),
            layer: 2,
            properties,
        }
    }

    fn analysis_graph() -> Graph {
        Graph::from_document(KirDocument {
            metadata: BTreeMap::new(),
            elements: vec![
                element(
                    "part.Vehicle",
                    "PartUsage",
                    BTreeMap::from([("declared_name".to_string(), json!("Vehicle"))]),
                ),
                element(
                    "assumption.StartupNominal",
                    "RequirementUsage",
                    BTreeMap::from([("declared_name".to_string(), json!("StartupNominal"))]),
                ),
                element(
                    "input.coolantTemp",
                    "AttributeUsage",
                    BTreeMap::from([("declared_name".to_string(), json!("coolantTemp"))]),
                ),
                element(
                    "output.verdict",
                    "AttributeUsage",
                    BTreeMap::from([("declared_name".to_string(), json!("verdict"))]),
                ),
                element(
                    "calcdef.StartMargin",
                    "CalculationDefinition",
                    BTreeMap::from([("declared_name".to_string(), json!("StartMargin"))]),
                ),
                element(
                    "calc.StartMargin.Run",
                    "CalculationUsage",
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("RunStartMargin")),
                        ("definition".to_string(), json!("calcdef.StartMargin")),
                    ]),
                ),
                element(
                    "constraintdef.SafeStart",
                    "ConstraintDefinition",
                    BTreeMap::from([("declared_name".to_string(), json!("SafeStart"))]),
                ),
                element(
                    "constraint.SafeStart.Applied",
                    "ConstraintUsage",
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("AppliedSafeStart")),
                        ("definition".to_string(), json!("constraintdef.SafeStart")),
                    ]),
                ),
                element(
                    "req.SafeStart",
                    "RequirementUsage",
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("SafeStart")),
                        (
                            "constraints".to_string(),
                            json!(["constraint.SafeStart.Applied"]),
                        ),
                    ]),
                ),
                element(
                    "verify.SafeStart",
                    "VerificationCaseUsage",
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("VerifySafeStart")),
                        ("requirement".to_string(), json!("req.SafeStart")),
                    ]),
                ),
                element(
                    "state.StartupSimulation",
                    "StateUsage",
                    BTreeMap::from([("declared_name".to_string(), json!("StartupSimulation"))]),
                ),
                element(
                    "concern.Safety",
                    "ConcernUsage",
                    BTreeMap::from([("declared_name".to_string(), json!("Safety"))]),
                ),
                element(
                    "view.StartupSafety",
                    "ViewUsage",
                    BTreeMap::from([("declared_name".to_string(), json!("StartupSafety"))]),
                ),
                element(
                    "analysis.StartupSafety",
                    "AnalysisCaseUsage",
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("StartupSafety")),
                        ("subject".to_string(), json!("part.Vehicle")),
                        (
                            "assumptions".to_string(),
                            json!(["assumption.StartupNominal"]),
                        ),
                        ("inputs".to_string(), json!(["input.coolantTemp"])),
                        ("outputs".to_string(), json!(["output.verdict"])),
                        ("calculations".to_string(), json!(["calc.StartMargin.Run"])),
                        (
                            "constraints".to_string(),
                            json!(["constraint.SafeStart.Applied"]),
                        ),
                        ("requirements".to_string(), json!(["req.SafeStart"])),
                        (
                            "verification_cases".to_string(),
                            json!(["verify.SafeStart"]),
                        ),
                        (
                            "simulations".to_string(),
                            json!(["state.StartupSimulation"]),
                        ),
                        ("concerns".to_string(), json!(["concern.Safety"])),
                        ("views".to_string(), json!(["view.StartupSafety"])),
                        ("script".to_string(), json!("model.all_parts().len")),
                    ]),
                ),
            ],
        })
        .expect("analysis fixture should build a graph")
    }

    #[test]
    fn inventory_extracts_sysml_analysis_nouns() {
        let graph = analysis_graph();
        let inventory = AnalysisInventory::from_graph(&graph);

        assert_eq!(inventory.analysis_cases.len(), 1);
        assert_eq!(inventory.calculation_definitions.len(), 1);
        assert_eq!(inventory.calculation_usages.len(), 1);
        assert_eq!(inventory.constraint_definitions.len(), 1);
        assert_eq!(inventory.constraint_usages.len(), 1);
        assert_eq!(inventory.requirement_evaluations.len(), 2);
        assert_eq!(inventory.verification_cases.len(), 1);
        assert_eq!(inventory.views.len(), 1);
        assert_eq!(inventory.concerns.len(), 1);
        assert_eq!(inventory.simulations.len(), 1);

        let case = &inventory.analysis_cases[0];
        assert_eq!(case.element.element_id, "analysis.StartupSafety");
        assert_eq!(case.subjects[0].element_id, "part.Vehicle");
        assert_eq!(case.calculations[0].element_id, "calc.StartMargin.Run");
        assert_eq!(
            case.constraints[0].element_id,
            "constraint.SafeStart.Applied"
        );
        assert_eq!(case.requirements[0].element_id, "req.SafeStart");
        assert_eq!(case.verification_cases[0].element_id, "verify.SafeStart");
        assert_eq!(case.simulations[0].element_id, "state.StartupSimulation");
        assert!(
            case.techniques
                .contains(&AnalysisTechniqueKind::Calculation)
        );
        assert!(
            case.techniques
                .contains(&AnalysisTechniqueKind::ConstraintEvaluation)
        );
        assert!(case.techniques.contains(&AnalysisTechniqueKind::Simulation));
        assert!(
            case.techniques
                .contains(&AnalysisTechniqueKind::Verification)
        );
        assert!(case.techniques.contains(&AnalysisTechniqueKind::Query));
    }

    #[test]
    fn requirement_evaluation_links_formal_constraints() {
        let graph = analysis_graph();
        let inventory = AnalysisInventory::from_graph(&graph);
        let requirement = inventory
            .requirement_evaluations
            .iter()
            .find(|evaluation| evaluation.requirement.element_id == "req.SafeStart")
            .expect("safe-start requirement should be inventoried");

        assert_eq!(
            requirement.formal_constraints[0].element_id,
            "constraint.SafeStart.Applied"
        );
    }

    #[test]
    fn workflow_orders_analysis_execution_from_case_context() {
        let graph = analysis_graph();
        let inventory = AnalysisInventory::from_graph(&graph);
        let workflow = inventory.analysis_cases[0].workflow();
        let step_kinds = workflow
            .steps
            .iter()
            .map(|step| step.kind)
            .collect::<Vec<_>>();

        assert_eq!(
            step_kinds,
            vec![
                AnalysisWorkflowStepKind::ScopeSubject,
                AnalysisWorkflowStepKind::BindAssumptions,
                AnalysisWorkflowStepKind::ResolveInputs,
                AnalysisWorkflowStepKind::ExecuteTechnique,
                AnalysisWorkflowStepKind::EvaluateConstraints,
                AnalysisWorkflowStepKind::EvaluateRequirements,
                AnalysisWorkflowStepKind::ProduceViews,
                AnalysisWorkflowStepKind::RecordResult,
            ]
        );
        assert!(
            workflow.steps[3]
                .techniques
                .contains(&AnalysisTechniqueKind::Simulation)
        );
    }

    #[test]
    fn opportunities_surface_analysis_cases_and_executable_semantics() {
        let graph = analysis_graph();
        let inventory = AnalysisInventory::from_graph(&graph);
        let report = inventory.opportunities();

        assert_eq!(report.schema, "mercurio.analysis.opportunities.v1");
        assert!(report.opportunities.iter().any(|opportunity| {
            opportunity.kind == AnalysisOpportunityKind::AnalysisCase
                && opportunity.action_id.as_deref() == Some("run_analysis_case")
        }));
        assert!(report.opportunities.iter().any(|opportunity| {
            opportunity.kind == AnalysisOpportunityKind::ConstraintEvaluation
                && opportunity.capability_id.as_deref() == Some("sysml.constraint.analysis")
        }));
        assert!(report.opportunities.iter().any(|opportunity| {
            opportunity.kind == AnalysisOpportunityKind::RequirementEvaluation
                && opportunity.capability_id.as_deref() == Some("sysml.requirement.analysis")
        }));
        assert!(report.opportunities.iter().any(|opportunity| {
            opportunity.kind == AnalysisOpportunityKind::Simulation
                && opportunity.capability_id.as_deref() == Some("sysml.behavior.dynamic")
                && opportunity.elements[0].element_id == "state.StartupSimulation"
        }));
    }
}
