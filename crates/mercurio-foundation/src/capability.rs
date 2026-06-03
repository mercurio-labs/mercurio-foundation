use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::graph::{Edge, Graph};
use crate::identity::{SourceSpanRef, workspace_revision_for_kir_document};
use crate::ir::{KirDocument, KirElement, KirError};
use crate::metamodel::MetamodelAttributeRegistry;
use crate::mutation::WorkspaceRevision;

#[derive(Debug, Clone)]
pub struct SemanticWorkspaceSnapshot {
    pub revision: WorkspaceRevision,
    pub kir: Arc<KirDocument>,
    pub graph: Arc<Graph>,
    pub metamodel_registry: Arc<MetamodelAttributeRegistry>,
    pub profile_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityDescriptor {
    pub id: String,
    pub name: String,
    pub kind: CapabilityKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_kinds: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationship_kinds: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_artifact_kinds: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub produced_insight_kinds: Vec<InsightKind>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub produced_artifact_kinds: Vec<String>,
    pub deterministic: bool,
    pub cost_class: CapabilityCostClass,
    pub maturity: CapabilityMaturity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityKind {
    RequirementAnalysis,
    Traceability,
    ImpactAnalysis,
    DynamicBehavior,
    ConstraintAnalysis,
    ContractAnalysis,
    MutationPreview,
    SemanticComparison,
    DecisionAssessment,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityCostClass {
    Cheap,
    Moderate,
    Expensive,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityMaturity {
    Experimental,
    Prototype,
    Stable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityRunRequest {
    pub run_id: String,
    pub capability_id: String,
    #[serde(default)]
    pub target: CapabilityTarget,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_artifacts: Vec<SemanticArtifact>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CapabilityTarget {
    Workspace,
    Element { element_id: String },
    Scope { scope_id: String },
}

impl Default for CapabilityTarget {
    fn default() -> Self {
        Self::Workspace
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityReadinessReport {
    pub capability_id: String,
    pub target: CapabilityTarget,
    pub status: CapabilityReadinessStatus,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_inputs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityReadinessStatus {
    Ready,
    Partial,
    NotApplicable,
    Blocked,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityRunReport {
    pub run_id: String,
    pub capability_id: String,
    pub status: CapabilityRunStatus,
    pub target: CapabilityTarget,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insights: Vec<SemanticInsight>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<SemanticArtifact>,
    #[serde(default)]
    pub evidence: EvidenceGraph,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<SemanticDiagnostic>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityRunStatus {
    Passed,
    Failed,
    Inconclusive,
    Partial,
    NotApplicable,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticInsight {
    pub id: String,
    pub kind: InsightKind,
    pub subject: SemanticElementRef,
    pub claim: String,
    pub polarity: InsightPolarity,
    pub severity: InsightSeverity,
    pub confidence: InsightConfidence,
    pub scope: InsightScope,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_spans: Vec<SourceSpanRef>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metrics: BTreeMap<String, Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assumptions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticElementRef {
    pub element_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualified_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightKind {
    CoverageGap,
    VerificationGap,
    SatisfactionEvidence,
    RequirementRisk,
    RequirementConflict,
    TraceCompleteness,
    ImpactHotspot,
    DependencyClosure,
    AffectedElement,
    ChangeRisk,
    BehaviorObserved,
    ScenarioFailure,
    RequirementViolation,
    ReachabilityFinding,
    RuntimeMetric,
    ConstraintViolation,
    SatisfiedConstraint,
    UnknownVariable,
    FeasibleRegion,
    DerivedMetric,
    AssumptionGap,
    GuaranteeViolation,
    InterfaceRisk,
    ContractCompatibility,
    ProposedEdit,
    FeasibilityIssue,
    RequiredChoice,
    ValidationDelta,
    SemanticDiff,
    ModelDelta,
    ConformanceGap,
    Regression,
    Improvement,
    CriterionPass,
    CriterionFail,
    MissingEvidence,
    DecisionRisk,
    RecommendedNextAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightPolarity {
    Supports,
    Weakens,
    Neutral,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightConfidence {
    High,
    Medium,
    Low,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum InsightScope {
    Workspace,
    Element { element_id: String },
    Scenario { scenario_id: String },
    Alternative { alternative_id: String },
    Revision { revision: WorkspaceRevision },
}

impl Default for InsightScope {
    fn default() -> Self {
        Self::Workspace
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticArtifact {
    pub id: String,
    pub kind: String,
    pub schema: String,
    pub digest: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub element_refs: Vec<SemanticElementRef>,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticDiagnostic {
    pub code: String,
    pub severity: SemanticDiagnosticSeverity,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<SemanticElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_spans: Vec<SourceSpanRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticDiagnosticSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EvidenceGraph {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<EvidenceNode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edges: Vec<EvidenceEdge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceNode {
    pub id: String,
    pub kind: EvidenceNodeKind,
    pub label: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub element_refs: Vec<SemanticElementRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_spans: Vec<SourceSpanRef>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceNodeKind {
    KirElement,
    SourceSpan,
    Fact,
    Rule,
    AnalysisRun,
    Capability,
    Artifact,
    HumanDecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceEdge {
    pub source_id: String,
    pub target_id: String,
    pub relation: EvidenceRelation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceRelation {
    Supports,
    DerivedFrom,
    ProducedBy,
    Consumed,
    Affects,
    Explains,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionContext {
    pub id: String,
    pub question: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternatives: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub criteria: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assumptions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scenarios: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_elements: Vec<SemanticElementRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionAssessment {
    pub context: DecisionContext,
    pub status: CapabilityRunStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insights: Vec<SemanticInsight>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing_evidence: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommended_next_actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityError {
    DuplicateCapability(String),
    MissingCapability(String),
    InvalidRequest(String),
    Workspace(String),
    Execution(String),
}

impl fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateCapability(id) => write!(f, "duplicate capability `{id}`"),
            Self::MissingCapability(id) => write!(f, "missing capability `{id}`"),
            Self::InvalidRequest(message) => write!(f, "invalid capability request: {message}"),
            Self::Workspace(message) => write!(f, "workspace capability error: {message}"),
            Self::Execution(message) => write!(f, "capability execution error: {message}"),
        }
    }
}

impl std::error::Error for CapabilityError {}

impl From<KirError> for CapabilityError {
    fn from(value: KirError) -> Self {
        Self::Workspace(value.to_string())
    }
}

pub trait SemanticCapability: Send + Sync {
    fn descriptor(&self) -> CapabilityDescriptor;

    fn readiness(
        &self,
        workspace: &SemanticWorkspaceSnapshot,
        target: &CapabilityTarget,
    ) -> CapabilityReadinessReport;

    fn run(
        &self,
        workspace: &SemanticWorkspaceSnapshot,
        request: CapabilityRunRequest,
    ) -> Result<CapabilityRunReport, CapabilityError>;
}

#[derive(Default)]
pub struct CapabilityRegistry {
    capabilities: BTreeMap<String, Box<dyn SemanticCapability>>,
}

#[derive(Debug, Clone, Default)]
pub struct GenericImpactCapability;

impl SemanticWorkspaceSnapshot {
    pub fn from_document(kir: KirDocument) -> Result<Self, CapabilityError> {
        Self::from_document_with_profile(kir, None)
    }

    pub fn from_document_with_profile(
        kir: KirDocument,
        profile_id: Option<String>,
    ) -> Result<Self, CapabilityError> {
        kir.validate()?;
        let revision = workspace_revision_for_kir_document(&kir)?;
        let graph = Graph::from_document(kir.clone())
            .map_err(|err| CapabilityError::Workspace(err.to_string()))?;
        let metamodel_registry = MetamodelAttributeRegistry::build(&graph);
        Ok(Self {
            revision,
            kir: Arc::new(kir),
            graph: Arc::new(graph),
            metamodel_registry: Arc::new(metamodel_registry),
            profile_id,
        })
    }

    pub fn from_graph_with_profile(
        graph: Graph,
        profile_id: Option<String>,
    ) -> Result<Self, CapabilityError> {
        let kir = KirDocument {
            metadata: BTreeMap::new(),
            elements: graph
                .elements()
                .iter()
                .map(|element| KirElement {
                    id: element.element_id.clone(),
                    kind: element.kind.to_string(),
                    layer: element.layer,
                    properties: element.properties.to_btree_map(),
                })
                .collect(),
        };
        let revision = workspace_revision_for_kir_document(&kir)?;
        let metamodel_registry = MetamodelAttributeRegistry::build(&graph);
        Ok(Self {
            revision,
            kir: Arc::new(kir),
            graph: Arc::new(graph),
            metamodel_registry: Arc::new(metamodel_registry),
            profile_id,
        })
    }

    pub fn element(&self, element_id: &str) -> Option<&crate::graph::Element> {
        self.graph.element_by_element_id(element_id)
    }

    pub fn element_ref(&self, element_id: &str) -> SemanticElementRef {
        self.element(element_id)
            .map(element_ref)
            .unwrap_or_else(|| SemanticElementRef {
                element_id: element_id.to_string(),
                qualified_name: None,
                label: None,
            })
    }

    pub fn source_spans(&self, element_id: &str) -> Vec<SourceSpanRef> {
        self.element(element_id)
            .and_then(source_span_for_element)
            .into_iter()
            .collect()
    }
}

impl CapabilityRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_foundation_builtins() -> Self {
        let mut registry = Self::new();
        registry
            .register(GenericImpactCapability)
            .expect("foundation impact capability id is unique");
        registry
    }

    pub fn register(
        &mut self,
        capability: impl SemanticCapability + 'static,
    ) -> Result<(), CapabilityError> {
        let descriptor = capability.descriptor();
        if self.capabilities.contains_key(&descriptor.id) {
            return Err(CapabilityError::DuplicateCapability(descriptor.id));
        }
        self.capabilities
            .insert(descriptor.id, Box::new(capability));
        Ok(())
    }

    pub fn list(&self) -> Vec<CapabilityDescriptor> {
        self.capabilities
            .values()
            .map(|capability| capability.descriptor())
            .collect()
    }

    pub fn readiness(
        &self,
        workspace: &SemanticWorkspaceSnapshot,
        capability_id: &str,
        target: &CapabilityTarget,
    ) -> Result<CapabilityReadinessReport, CapabilityError> {
        Ok(self
            .capabilities
            .get(capability_id)
            .ok_or_else(|| CapabilityError::MissingCapability(capability_id.to_string()))?
            .readiness(workspace, target))
    }

    pub fn run(
        &self,
        workspace: &SemanticWorkspaceSnapshot,
        request: CapabilityRunRequest,
    ) -> Result<CapabilityRunReport, CapabilityError> {
        self.capabilities
            .get(&request.capability_id)
            .ok_or_else(|| CapabilityError::MissingCapability(request.capability_id.clone()))?
            .run(workspace, request)
    }
}

pub fn assess_decision_context(
    context: DecisionContext,
    readiness: &[CapabilityReadinessReport],
    reports: &[CapabilityRunReport],
) -> DecisionAssessment {
    let insights = reports
        .iter()
        .flat_map(|report| report.insights.iter().cloned())
        .collect::<Vec<_>>();
    let missing_evidence = missing_evidence(readiness, reports);
    let status = decision_status(readiness, reports, &insights, &missing_evidence);
    let recommended_next_actions =
        recommended_next_actions(readiness, reports, &insights, &missing_evidence);

    DecisionAssessment {
        context,
        status,
        insights,
        missing_evidence,
        recommended_next_actions,
    }
}

fn missing_evidence(
    readiness: &[CapabilityReadinessReport],
    reports: &[CapabilityRunReport],
) -> Vec<String> {
    let reported = reports
        .iter()
        .map(|report| report.capability_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut missing = Vec::new();

    for readiness in readiness {
        match readiness.status {
            CapabilityReadinessStatus::NotApplicable => {}
            CapabilityReadinessStatus::Ready | CapabilityReadinessStatus::Partial
                if reported.contains(readiness.capability_id.as_str()) => {}
            _ => push_unique(
                &mut missing,
                format!("{}: {}", readiness.capability_id, readiness.message),
            ),
        }
    }

    for report in reports {
        if report.status == CapabilityRunStatus::Inconclusive && report.insights.is_empty() {
            push_unique(
                &mut missing,
                format!(
                    "{}: produced no decision-relevant insights",
                    report.capability_id
                ),
            );
        }
    }

    missing
}

fn decision_status(
    readiness: &[CapabilityReadinessReport],
    reports: &[CapabilityRunReport],
    insights: &[SemanticInsight],
    missing_evidence: &[String],
) -> CapabilityRunStatus {
    if readiness
        .iter()
        .any(|report| report.status == CapabilityReadinessStatus::Error)
        || reports
            .iter()
            .any(|report| report.status == CapabilityRunStatus::Error)
    {
        return CapabilityRunStatus::Error;
    }

    if reports
        .iter()
        .any(|report| report.status == CapabilityRunStatus::Failed)
        || insights.iter().any(is_failure_insight)
    {
        return CapabilityRunStatus::Failed;
    }

    if reports.is_empty() || insights.is_empty() {
        return CapabilityRunStatus::Inconclusive;
    }

    if !missing_evidence.is_empty()
        || reports
            .iter()
            .any(|report| report.status == CapabilityRunStatus::Partial)
        || insights.iter().any(is_cautionary_insight)
    {
        return CapabilityRunStatus::Partial;
    }

    CapabilityRunStatus::Passed
}

fn recommended_next_actions(
    readiness: &[CapabilityReadinessReport],
    reports: &[CapabilityRunReport],
    insights: &[SemanticInsight],
    missing_evidence: &[String],
) -> Vec<String> {
    let mut actions = Vec::new();

    let mut prioritized_insights = insights.iter().collect::<Vec<_>>();
    prioritized_insights.sort_by(|left, right| {
        action_priority(left)
            .cmp(&action_priority(right))
            .then_with(|| severity_rank(right.severity).cmp(&severity_rank(left.severity)))
    });

    for insight in prioritized_insights {
        match insight.kind {
            InsightKind::CoverageGap => push_unique(
                &mut actions,
                "Add satisfy evidence for uncovered requirements, then rerun requirement analysis.",
            ),
            InsightKind::VerificationGap => push_unique(
                &mut actions,
                "Add or link verification cases for requirements with verification gaps, then rerun requirement analysis.",
            ),
            InsightKind::RequirementRisk | InsightKind::RequirementConflict => push_unique(
                &mut actions,
                "Resolve the requirement risk or conflict before accepting the decision.",
            ),
            InsightKind::ScenarioFailure
            | InsightKind::RequirementViolation
            | InsightKind::ReachabilityFinding => push_unique(
                &mut actions,
                "Make the behavior scenario executable and rerun dynamic behavior analysis.",
            ),
            InsightKind::ConstraintViolation
            | InsightKind::UnknownVariable
            | InsightKind::FeasibilityIssue => push_unique(
                &mut actions,
                "Resolve constraint violations or bind unknown variables, then rerun constraint analysis.",
            ),
            InsightKind::AssumptionGap
            | InsightKind::GuaranteeViolation
            | InsightKind::InterfaceRisk => push_unique(
                &mut actions,
                "Resolve contract assumptions, guarantees, or interface risks before committing the design.",
            ),
            InsightKind::ImpactHotspot | InsightKind::AffectedElement | InsightKind::ChangeRisk => {
                push_unique(
                    &mut actions,
                    "Review impact hotspots before selecting a model edit.",
                )
            }
            InsightKind::RequiredChoice => push_unique(
                &mut actions,
                "Capture the required engineering choice explicitly so downstream analyses can evaluate it.",
            ),
            InsightKind::Regression | InsightKind::ConformanceGap => push_unique(
                &mut actions,
                "Compare the candidate against the baseline and address regressions or conformance gaps.",
            ),
            InsightKind::DecisionRisk => push_unique(
                &mut actions,
                "Resolve the highest-severity decision risk before accepting an alternative.",
            ),
            InsightKind::MissingEvidence => push_unique(
                &mut actions,
                "Add or run the missing evidence capability before making the decision.",
            ),
            _ => {}
        }
    }

    for report in reports {
        if report.status == CapabilityRunStatus::Error {
            push_unique(
                &mut actions,
                format!(
                    "Fix capability execution for `{}` before relying on its result.",
                    report.capability_id
                ),
            );
        }
    }

    for readiness in readiness.iter().filter(|readiness| {
        matches!(
            readiness.status,
            CapabilityReadinessStatus::Blocked | CapabilityReadinessStatus::Error
        )
    }) {
        push_unique(
            &mut actions,
            format!(
                "Unblock `{}`: {}",
                readiness.capability_id, readiness.message
            ),
        );
    }

    if !missing_evidence.is_empty() {
        push_unique(
            &mut actions,
            format!(
                "Add or unblock missing capability evidence: {}.",
                missing_evidence
                    .iter()
                    .take(3)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        );
    }

    if actions.is_empty() {
        if insights.is_empty() {
            actions.push(
                "Run at least one applicable semantic capability before committing to a design decision."
                    .to_string(),
            );
        } else {
            actions.push(
                "Record the decision rationale and rerun capability analysis after the next model change."
                    .to_string(),
            );
        }
    }

    actions
}

fn action_priority(insight: &SemanticInsight) -> u8 {
    if is_failure_insight(insight) {
        return 0;
    }

    match insight.kind {
        InsightKind::ScenarioFailure
        | InsightKind::RequirementViolation
        | InsightKind::ConstraintViolation
        | InsightKind::GuaranteeViolation
        | InsightKind::Regression => 1,
        InsightKind::CoverageGap
        | InsightKind::VerificationGap
        | InsightKind::RequirementRisk
        | InsightKind::RequirementConflict
        | InsightKind::UnknownVariable
        | InsightKind::FeasibilityIssue
        | InsightKind::AssumptionGap
        | InsightKind::InterfaceRisk
        | InsightKind::ConformanceGap
        | InsightKind::DecisionRisk
        | InsightKind::MissingEvidence => 2,
        InsightKind::RequiredChoice => 3,
        InsightKind::ImpactHotspot | InsightKind::AffectedElement | InsightKind::ChangeRisk => 4,
        _ => 5,
    }
}

fn severity_rank(severity: InsightSeverity) -> u8 {
    match severity {
        InsightSeverity::Critical => 4,
        InsightSeverity::Error => 3,
        InsightSeverity::Warning => 2,
        InsightSeverity::Info => 1,
    }
}

fn is_failure_insight(insight: &SemanticInsight) -> bool {
    insight.severity == InsightSeverity::Critical
        || (insight.polarity == InsightPolarity::Weakens
            && matches!(
                insight.severity,
                InsightSeverity::Error | InsightSeverity::Critical
            ))
}

fn is_cautionary_insight(insight: &SemanticInsight) -> bool {
    insight.polarity == InsightPolarity::Weakens
        && matches!(
            insight.severity,
            InsightSeverity::Warning | InsightSeverity::Error | InsightSeverity::Critical
        )
}

fn push_unique(values: &mut Vec<String>, value: impl Into<String>) {
    let value = value.into();
    if !values.contains(&value) {
        values.push(value);
    }
}

impl SemanticCapability for GenericImpactCapability {
    fn descriptor(&self) -> CapabilityDescriptor {
        CapabilityDescriptor {
            id: "foundation.impact.graph".to_string(),
            name: "Generic Graph Impact".to_string(),
            kind: CapabilityKind::ImpactAnalysis,
            profile_id: None,
            target_kinds: Vec::new(),
            relationship_kinds: Vec::new(),
            input_artifact_kinds: Vec::new(),
            produced_insight_kinds: vec![
                InsightKind::ImpactHotspot,
                InsightKind::AffectedElement,
                InsightKind::DependencyClosure,
            ],
            produced_artifact_kinds: vec!["graph_impact_summary".to_string()],
            deterministic: true,
            cost_class: CapabilityCostClass::Cheap,
            maturity: CapabilityMaturity::Prototype,
        }
    }

    fn readiness(
        &self,
        workspace: &SemanticWorkspaceSnapshot,
        target: &CapabilityTarget,
    ) -> CapabilityReadinessReport {
        if workspace.graph.elements().is_empty() {
            return readiness(
                self.descriptor().id,
                target.clone(),
                CapabilityReadinessStatus::NotApplicable,
                "workspace has no semantic elements",
            );
        }
        match target {
            CapabilityTarget::Element { element_id } if workspace.element(element_id).is_none() => {
                readiness(
                    self.descriptor().id,
                    target.clone(),
                    CapabilityReadinessStatus::Blocked,
                    format!("target element `{element_id}` does not exist"),
                )
            }
            _ => readiness(
                self.descriptor().id,
                target.clone(),
                CapabilityReadinessStatus::Ready,
                "graph impact can run over the selected scope",
            ),
        }
    }

    fn run(
        &self,
        workspace: &SemanticWorkspaceSnapshot,
        request: CapabilityRunRequest,
    ) -> Result<CapabilityRunReport, CapabilityError> {
        let readiness = self.readiness(workspace, &request.target);
        if readiness.status == CapabilityReadinessStatus::Blocked
            || readiness.status == CapabilityReadinessStatus::NotApplicable
        {
            return Ok(CapabilityRunReport {
                run_id: request.run_id,
                capability_id: request.capability_id,
                status: match readiness.status {
                    CapabilityReadinessStatus::NotApplicable => CapabilityRunStatus::NotApplicable,
                    _ => CapabilityRunStatus::Error,
                },
                target: request.target,
                insights: Vec::new(),
                artifacts: Vec::new(),
                evidence: EvidenceGraph::default(),
                diagnostics: Vec::new(),
                limitations: vec![readiness.message],
            });
        }

        let (insights, payload, evidence) = match &request.target {
            CapabilityTarget::Element { element_id } => {
                graph_impact_for_element(workspace, element_id)?
            }
            _ => graph_impact_for_workspace(workspace, parameter_usize(&request, "limit", 5)),
        };
        let artifact = SemanticArtifact {
            id: format!("artifact.{}.impact", request.run_id),
            kind: "graph_impact_summary".to_string(),
            schema: "mercurio.capability.graph_impact.v1".to_string(),
            digest: value_digest(&payload),
            element_refs: insights
                .iter()
                .map(|insight| insight.subject.clone())
                .collect(),
            payload,
        };

        Ok(CapabilityRunReport {
            run_id: request.run_id,
            capability_id: request.capability_id,
            status: if insights.is_empty() {
                CapabilityRunStatus::Inconclusive
            } else {
                CapabilityRunStatus::Partial
            },
            target: request.target,
            insights,
            artifacts: vec![artifact],
            evidence,
            diagnostics: Vec::new(),
            limitations: Vec::new(),
        })
    }
}

fn readiness(
    capability_id: String,
    target: CapabilityTarget,
    status: CapabilityReadinessStatus,
    message: impl Into<String>,
) -> CapabilityReadinessReport {
    CapabilityReadinessReport {
        capability_id,
        target,
        status,
        message: message.into(),
        required_inputs: Vec::new(),
        limitations: Vec::new(),
    }
}

fn graph_impact_for_workspace(
    workspace: &SemanticWorkspaceSnapshot,
    limit: usize,
) -> (Vec<SemanticInsight>, Value, EvidenceGraph) {
    let mut degree_by_node = BTreeMap::<u32, usize>::new();
    for edge in workspace.graph.edges() {
        *degree_by_node.entry(edge.source).or_default() += 1;
        *degree_by_node.entry(edge.target).or_default() += 1;
    }
    let mut ranked = degree_by_node
        .into_iter()
        .filter_map(|(node_id, degree)| {
            let element = workspace.graph.element(node_id)?;
            Some((element.element_id.clone(), degree))
        })
        .collect::<Vec<_>>();
    ranked.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    ranked.truncate(limit);

    let mut evidence = EvidenceGraph::default();
    let insights = ranked
        .iter()
        .enumerate()
        .map(|(index, (element_id, degree))| {
            let evidence_id = format!("evidence.impact.hotspot.{element_id}");
            let subject = workspace.element_ref(element_id);
            evidence.nodes.push(EvidenceNode {
                id: evidence_id.clone(),
                kind: EvidenceNodeKind::Fact,
                label: format!("{element_id} has {degree} incident graph edges"),
                element_refs: vec![subject.clone()],
                source_spans: workspace.source_spans(element_id),
                properties: BTreeMap::from([("degree".to_string(), Value::from(*degree))]),
            });
            SemanticInsight {
                id: format!("insight.impact.hotspot.{}", index + 1),
                kind: InsightKind::ImpactHotspot,
                subject,
                claim: format!(
                    "`{element_id}` is a graph impact hotspot with {degree} incident edges."
                ),
                polarity: InsightPolarity::Neutral,
                severity: if *degree >= 10 {
                    InsightSeverity::Warning
                } else {
                    InsightSeverity::Info
                },
                confidence: InsightConfidence::Medium,
                scope: InsightScope::Workspace,
                evidence_ids: vec![evidence_id],
                source_spans: workspace.source_spans(element_id),
                metrics: BTreeMap::from([("degree".to_string(), Value::from(*degree))]),
                assumptions: Vec::new(),
                limitations: vec![
                    "generic graph impact does not apply profile-specific semantics".to_string(),
                ],
            }
        })
        .collect::<Vec<_>>();
    let payload = json!({
        "schema": "mercurio.capability.graph_impact.v1",
        "scope": "workspace",
        "hotspots": ranked
            .iter()
            .map(|(element_id, degree)| json!({ "elementId": element_id, "degree": degree }))
            .collect::<Vec<_>>(),
    });

    (insights, payload, evidence)
}

fn graph_impact_for_element(
    workspace: &SemanticWorkspaceSnapshot,
    element_id: &str,
) -> Result<(Vec<SemanticInsight>, Value, EvidenceGraph), CapabilityError> {
    let node = workspace.graph.node_id(element_id).ok_or_else(|| {
        CapabilityError::InvalidRequest(format!("unknown element `{element_id}`"))
    })?;
    let outgoing = workspace.graph.outgoing_edges(node).collect::<Vec<_>>();
    let incoming = workspace.graph.incoming_edges(node).collect::<Vec<_>>();
    let degree = outgoing.len() + incoming.len();
    let subject = workspace.element_ref(element_id);
    let evidence_id = format!("evidence.impact.element.{element_id}");
    let insight = SemanticInsight {
        id: format!("insight.impact.element.{element_id}"),
        kind: InsightKind::AffectedElement,
        subject: subject.clone(),
        claim: format!("`{element_id}` has {degree} direct semantic graph connections."),
        polarity: InsightPolarity::Neutral,
        severity: if degree >= 10 {
            InsightSeverity::Warning
        } else {
            InsightSeverity::Info
        },
        confidence: InsightConfidence::Medium,
        scope: InsightScope::Element {
            element_id: element_id.to_string(),
        },
        evidence_ids: vec![evidence_id.clone()],
        source_spans: workspace.source_spans(element_id),
        metrics: BTreeMap::from([
            ("incoming_edges".to_string(), Value::from(incoming.len())),
            ("outgoing_edges".to_string(), Value::from(outgoing.len())),
            ("degree".to_string(), Value::from(degree)),
        ]),
        assumptions: Vec::new(),
        limitations: vec!["generic graph impact reports direct KIR references only".to_string()],
    };
    let payload = json!({
        "schema": "mercurio.capability.graph_impact.v1",
        "scope": "element",
        "elementId": element_id,
        "incoming": edge_payload(workspace, &incoming),
        "outgoing": edge_payload(workspace, &outgoing),
    });
    let evidence = EvidenceGraph {
        nodes: vec![EvidenceNode {
            id: evidence_id,
            kind: EvidenceNodeKind::Fact,
            label: format!("{element_id} direct graph connections"),
            element_refs: vec![subject],
            source_spans: workspace.source_spans(element_id),
            properties: BTreeMap::from([
                ("incoming_edges".to_string(), Value::from(incoming.len())),
                ("outgoing_edges".to_string(), Value::from(outgoing.len())),
            ]),
        }],
        edges: Vec::new(),
    };

    Ok((vec![insight], payload, evidence))
}

fn edge_payload(workspace: &SemanticWorkspaceSnapshot, edges: &[&Edge]) -> Vec<Value> {
    edges
        .iter()
        .filter_map(|edge| {
            Some(json!({
                "source": workspace.graph.element_id(edge.source)?,
                "relation": edge.relation.as_ref(),
                "target": workspace.graph.element_id(edge.target)?,
            }))
        })
        .collect()
}

fn parameter_usize(request: &CapabilityRunRequest, key: &str, default: usize) -> usize {
    request
        .parameters
        .get(key)
        .and_then(Value::as_u64)
        .map(|value| value as usize)
        .unwrap_or(default)
}

fn element_ref(element: &crate::graph::Element) -> SemanticElementRef {
    SemanticElementRef {
        element_id: element.element_id.clone(),
        qualified_name: string_property(element, "qualified_name"),
        label: string_property(element, "declared_name")
            .or_else(|| string_property(element, "name"))
            .or_else(|| {
                element
                    .element_id
                    .rsplit(['.', ':', '/'])
                    .find(|part| !part.is_empty())
                    .map(ToOwned::to_owned)
            }),
    }
}

fn source_span_for_element(element: &crate::graph::Element) -> Option<SourceSpanRef> {
    let direct = element.properties.get("source_span");
    let metadata = element.properties.get("metadata");
    let span = direct.or_else(|| metadata.and_then(|metadata| metadata.get("source_span")))?;
    let file = metadata
        .and_then(|metadata| metadata.get("source_file"))
        .and_then(Value::as_str)
        .or_else(|| span.get("file").and_then(Value::as_str))
        .unwrap_or("");
    Some(SourceSpanRef {
        file: file.to_string(),
        start_line: span
            .get("start_line")
            .or_else(|| span.get("startLine"))
            .and_then(Value::as_u64)
            .unwrap_or(0) as u32,
        start_col: span
            .get("start_col")
            .or_else(|| span.get("startCol"))
            .and_then(Value::as_u64)
            .unwrap_or(0) as u32,
        end_line: span
            .get("end_line")
            .or_else(|| span.get("endLine"))
            .and_then(Value::as_u64)
            .unwrap_or(0) as u32,
        end_col: span
            .get("end_col")
            .or_else(|| span.get("endCol"))
            .and_then(Value::as_u64)
            .unwrap_or(0) as u32,
    })
}

fn string_property(element: &crate::graph::Element, key: &str) -> Option<String> {
    element
        .properties
        .get(key)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn value_digest(value: &Value) -> String {
    let bytes = serde_json::to_vec(value).unwrap_or_default();
    crate::stable_digest([("semantic-artifact".as_bytes(), bytes.as_slice())])
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::Value;

    use crate::{KirDocument, KirElement};

    use super::*;

    #[test]
    fn registry_lists_and_runs_graph_impact() {
        let workspace = SemanticWorkspaceSnapshot::from_document(KirDocument {
            metadata: BTreeMap::new(),
            elements: vec![
                KirElement {
                    id: "pkg.Demo".to_string(),
                    kind: "Model::Package".to_string(),
                    layer: 2,
                    properties: BTreeMap::from([(
                        "members".to_string(),
                        Value::Array(vec![Value::String("type.Vehicle".to_string())]),
                    )]),
                },
                KirElement {
                    id: "type.Vehicle".to_string(),
                    kind: "Model::PartDefinition".to_string(),
                    layer: 2,
                    properties: BTreeMap::new(),
                },
            ],
        })
        .unwrap();
        let registry = CapabilityRegistry::with_foundation_builtins();

        assert!(
            registry
                .list()
                .iter()
                .any(|descriptor| descriptor.id == "foundation.impact.graph")
        );

        let report = registry
            .run(
                &workspace,
                CapabilityRunRequest {
                    run_id: "run.impact".to_string(),
                    capability_id: "foundation.impact.graph".to_string(),
                    target: CapabilityTarget::Workspace,
                    parameters: BTreeMap::new(),
                    input_artifacts: Vec::new(),
                },
            )
            .unwrap();

        assert!(!report.insights.is_empty());
        assert_eq!(report.artifacts[0].kind, "graph_impact_summary");
    }

    #[test]
    fn decision_composition_reports_blocked_missing_evidence() {
        let readiness = vec![CapabilityReadinessReport {
            capability_id: "sysml.constraint.analysis".to_string(),
            target: CapabilityTarget::Workspace,
            status: CapabilityReadinessStatus::Blocked,
            message: "no supported constraint usages were found".to_string(),
            required_inputs: Vec::new(),
            limitations: Vec::new(),
        }];

        let assessment = assess_decision_context(test_decision_context(), &readiness, &[]);

        assert_eq!(assessment.status, CapabilityRunStatus::Inconclusive);
        assert_eq!(
            assessment.missing_evidence,
            vec!["sysml.constraint.analysis: no supported constraint usages were found"]
        );
        assert!(
            assessment
                .recommended_next_actions
                .iter()
                .any(|action| action.contains("Unblock `sysml.constraint.analysis`"))
        );
    }

    #[test]
    fn decision_composition_turns_weakening_insights_into_actions() {
        let report = CapabilityRunReport {
            run_id: "run.requirements".to_string(),
            capability_id: "sysml.requirement.analysis".to_string(),
            status: CapabilityRunStatus::Passed,
            target: CapabilityTarget::Workspace,
            insights: vec![test_insight(
                InsightKind::CoverageGap,
                InsightPolarity::Weakens,
                InsightSeverity::Warning,
            )],
            artifacts: Vec::new(),
            evidence: EvidenceGraph::default(),
            diagnostics: Vec::new(),
            limitations: Vec::new(),
        };

        let assessment = assess_decision_context(test_decision_context(), &[], &[report]);

        assert_eq!(assessment.status, CapabilityRunStatus::Partial);
        assert_eq!(assessment.insights.len(), 1);
        assert!(
            assessment
                .recommended_next_actions
                .iter()
                .any(|action| action.contains("satisfy evidence"))
        );
    }

    #[test]
    fn decision_composition_fails_on_critical_weakening_evidence() {
        let report = CapabilityRunReport {
            run_id: "run.behavior".to_string(),
            capability_id: "sysml.behavior.dynamic".to_string(),
            status: CapabilityRunStatus::Passed,
            target: CapabilityTarget::Workspace,
            insights: vec![test_insight(
                InsightKind::ScenarioFailure,
                InsightPolarity::Weakens,
                InsightSeverity::Critical,
            )],
            artifacts: Vec::new(),
            evidence: EvidenceGraph::default(),
            diagnostics: Vec::new(),
            limitations: Vec::new(),
        };

        let assessment = assess_decision_context(test_decision_context(), &[], &[report]);

        assert_eq!(assessment.status, CapabilityRunStatus::Failed);
        assert!(
            assessment
                .recommended_next_actions
                .iter()
                .any(|action| action.contains("dynamic behavior analysis"))
        );
    }

    #[test]
    fn decision_composition_prioritizes_severe_weakening_evidence() {
        let report = CapabilityRunReport {
            run_id: "run.mixed".to_string(),
            capability_id: "centaur.mixed".to_string(),
            status: CapabilityRunStatus::Passed,
            target: CapabilityTarget::Workspace,
            insights: vec![
                test_insight(
                    InsightKind::ImpactHotspot,
                    InsightPolarity::Neutral,
                    InsightSeverity::Warning,
                ),
                test_insight(
                    InsightKind::ScenarioFailure,
                    InsightPolarity::Weakens,
                    InsightSeverity::Error,
                ),
            ],
            artifacts: Vec::new(),
            evidence: EvidenceGraph::default(),
            diagnostics: Vec::new(),
            limitations: Vec::new(),
        };

        let assessment = assess_decision_context(test_decision_context(), &[], &[report]);

        assert_eq!(assessment.status, CapabilityRunStatus::Failed);
        assert_eq!(
            assessment.recommended_next_actions[0],
            "Make the behavior scenario executable and rerun dynamic behavior analysis."
        );
    }

    fn test_decision_context() -> DecisionContext {
        DecisionContext {
            id: "decision.test".to_string(),
            question: "What should the next model edit be?".to_string(),
            alternatives: Vec::new(),
            criteria: Vec::new(),
            assumptions: Vec::new(),
            scenarios: Vec::new(),
            target_elements: Vec::new(),
        }
    }

    fn test_insight(
        kind: InsightKind,
        polarity: InsightPolarity,
        severity: InsightSeverity,
    ) -> SemanticInsight {
        SemanticInsight {
            id: "insight.test".to_string(),
            kind,
            subject: SemanticElementRef {
                element_id: "element.test".to_string(),
                qualified_name: None,
                label: Some("Test Element".to_string()),
            },
            claim: "test insight".to_string(),
            polarity,
            severity,
            confidence: InsightConfidence::Medium,
            scope: InsightScope::Workspace,
            evidence_ids: Vec::new(),
            source_spans: Vec::new(),
            metrics: BTreeMap::new(),
            assumptions: Vec::new(),
            limitations: Vec::new(),
        }
    }
}
