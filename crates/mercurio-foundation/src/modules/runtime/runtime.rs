use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::kir::ExpressionResult;
use crate::model::{
    DerivedFeatureCache, DerivedFeatureManifestError, DerivedFeatureRegistry, DerivedPropertyValue,
    ElementProperties, ExpressionEvaluationContext, ExpressionEvaluationError, ExpressionIr,
    ExpressionPathSegment, Graph, GraphArtifact, GraphError, KirDocument, KirElement, NodeId,
    manifest_from_metadata,
};
use crate::runtime::datalog::{
    DatalogError, DerivedIndexes, RulePack, load_default_rulepacks, materialize_core_indexes,
};

#[derive(Debug, Clone)]
pub struct Runtime {
    graph: Graph,
    derived: DerivedIndexes,
    derived_feature_registry: DerivedFeatureRegistry,
    derived_feature_cache: DerivedFeatureCache,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeArtifact {
    pub graph: GraphArtifact,
    pub derived: DerivedIndexes,
}

#[derive(Debug, Clone)]
pub struct RuntimeBase {
    document: Arc<KirDocument>,
    graph: Arc<Graph>,
    derived: Arc<DerivedIndexes>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RuntimeOverlay {
    pub added_elements: BTreeMap<String, KirElement>,
    pub updated_properties: BTreeMap<String, BTreeMap<String, Value>>,
    pub added_members: BTreeMap<String, Vec<String>>,
    pub removed_elements: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct LayeredRuntime {
    graph: Arc<Graph>,
    derived: Arc<DerivedIndexes>,
    overlay_element_count: usize,
    assembly: LayeredRuntimeAssembly,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayeredRuntimeAssembly {
    SharedBase,
    OverlayMaterialized { elapsed_millis: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeProfile {
    pub element_count: usize,
    pub timings: RuntimeProfileTimings,
    pub graph_element_count: usize,
    pub graph_edge_count: usize,
    pub subtype_count: usize,
    pub ownership_count: usize,
    pub inherited_feature_count: usize,
    pub requirement_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeProfileTimings {
    pub workspace_revision_millis: f64,
    pub derived_manifest_millis: f64,
    pub rulepack_load_millis: f64,
    pub graph_build_millis: f64,
    pub derived_materialization_millis: f64,
    pub cache_setup_millis: f64,
    pub total_millis: f64,
}

#[derive(Debug, Clone, Default)]
pub struct ExecutionContext {
    pub values: HashMap<(String, String), Value>,
    pub version: u64,
}

#[derive(Debug, Clone)]
pub struct QueryResult<T> {
    pub value: T,
    pub explanation: Vec<String>,
}

#[derive(Debug)]
pub enum RuntimeError {
    Graph(GraphError),
    Datalog(DatalogError),
    InvalidExpression(String),
    MissingElement(String),
    MissingDerivedFeature { element: String, feature: String },
    DerivedFeatureManifest(DerivedFeatureManifestError),
    UnsupportedAggregation(String),
    NonNumericValue { owner: String, feature: String },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Graph(err) => write!(f, "{err}"),
            Self::Datalog(err) => write!(f, "{err}"),
            Self::InvalidExpression(expr) => write!(f, "invalid expression: {expr}"),
            Self::MissingElement(id) => write!(f, "missing element: {id}"),
            Self::MissingDerivedFeature { element, feature } => {
                write!(f, "missing derived feature {feature} for {element}")
            }
            Self::DerivedFeatureManifest(err) => write!(f, "{err}"),
            Self::UnsupportedAggregation(expr) => {
                write!(f, "unsupported aggregation expression: {expr}")
            }
            Self::NonNumericValue { owner, feature } => {
                write!(
                    f,
                    "non-numeric value encountered while reading {feature} from {owner}"
                )
            }
        }
    }
}

impl std::error::Error for RuntimeError {}

impl From<GraphError> for RuntimeError {
    fn from(value: GraphError) -> Self {
        Self::Graph(value)
    }
}

impl From<DatalogError> for RuntimeError {
    fn from(value: DatalogError) -> Self {
        Self::Datalog(value)
    }
}

impl From<DerivedFeatureManifestError> for RuntimeError {
    fn from(value: DerivedFeatureManifestError) -> Self {
        Self::DerivedFeatureManifest(value)
    }
}

impl From<ExpressionEvaluationError> for RuntimeError {
    fn from(value: ExpressionEvaluationError) -> Self {
        match value {
            ExpressionEvaluationError::ContractViolation(_)
            | ExpressionEvaluationError::DuplicateBinding { .. }
            | ExpressionEvaluationError::UnboundParameter { .. }
            | ExpressionEvaluationError::ResourceLimitExceeded { .. }
            | ExpressionEvaluationError::MissingBinding(_)
            | ExpressionEvaluationError::DivisionByZero
            | ExpressionEvaluationError::NonFiniteResult => {
                Self::InvalidExpression(value.to_string())
            }
            ExpressionEvaluationError::InvalidExpression(expression) => {
                Self::InvalidExpression(expression)
            }
            ExpressionEvaluationError::UnsupportedAggregation { expression } => {
                Self::UnsupportedAggregation(expression)
            }
            ExpressionEvaluationError::UnsupportedFunction { .. } => {
                Self::InvalidExpression(value.to_string())
            }
            ExpressionEvaluationError::NonNumericValue { owner, feature } => {
                Self::NonNumericValue { owner, feature }
            }
        }
    }
}

impl Runtime {
    pub fn from_graph(graph: Graph) -> Result<Self, RuntimeError> {
        let rulepacks = load_default_rulepacks()?;
        Self::from_graph_with_rulepacks(graph, &rulepacks)
    }

    pub fn from_graph_with_rulepacks(
        graph: Graph,
        rulepacks: &[RulePack],
    ) -> Result<Self, RuntimeError> {
        let derived = materialize_core_indexes(&graph, rulepacks)?;
        Ok(Self {
            graph,
            derived,
            derived_feature_registry: DerivedFeatureRegistry::with_builtin_core_specs(),
            derived_feature_cache: DerivedFeatureCache::new("graph"),
        })
    }

    pub fn from_document(document: KirDocument) -> Result<Self, RuntimeError> {
        let revision =
            workspace_revision_fingerprint(&document).unwrap_or_else(|_| "document".to_string());
        let derived_feature_registry = DerivedFeatureRegistry::with_manifest_and_builtins(
            manifest_from_metadata(&document.metadata)?,
        )?;
        let rulepacks = load_default_rulepacks()?;
        let graph = Graph::from_document(document)?;
        let derived = materialize_core_indexes(&graph, &rulepacks)?;
        Ok(Self {
            graph,
            derived,
            derived_feature_registry,
            derived_feature_cache: DerivedFeatureCache::new(revision),
        })
    }

    pub fn profile_from_document(document: KirDocument) -> Result<RuntimeProfile, RuntimeError> {
        let element_count = document.elements.len();
        let total_timer = Instant::now();

        let revision_timer = Instant::now();
        let revision =
            workspace_revision_fingerprint(&document).unwrap_or_else(|_| "document".to_string());
        let workspace_revision_millis = millis(revision_timer.elapsed());

        let manifest_timer = Instant::now();
        let derived_feature_registry = DerivedFeatureRegistry::with_manifest_and_builtins(
            manifest_from_metadata(&document.metadata)?,
        )?;
        let derived_manifest_millis = millis(manifest_timer.elapsed());

        let rulepack_timer = Instant::now();
        let rulepacks = load_default_rulepacks()?;
        let rulepack_load_millis = millis(rulepack_timer.elapsed());

        let graph_timer = Instant::now();
        let graph = Graph::from_document(document)?;
        let graph_build_millis = millis(graph_timer.elapsed());
        let graph_element_count = graph.elements().len();
        let graph_edge_count = graph.edges().len();

        let materialize_timer = Instant::now();
        let derived = materialize_core_indexes(&graph, &rulepacks)?;
        let derived_materialization_millis = millis(materialize_timer.elapsed());
        let subtype_count = derived.subtypes.len();
        let ownership_count = derived.ownership.len();
        let inherited_feature_count = derived.inherited_features.len();
        let requirement_count = derived.requirements.len();

        let cache_timer = Instant::now();
        let derived_feature_cache = DerivedFeatureCache::new(revision);
        let cache_setup_millis = millis(cache_timer.elapsed());
        drop((derived_feature_registry, derived_feature_cache));

        Ok(RuntimeProfile {
            element_count,
            timings: RuntimeProfileTimings {
                workspace_revision_millis,
                derived_manifest_millis,
                rulepack_load_millis,
                graph_build_millis,
                derived_materialization_millis,
                cache_setup_millis,
                total_millis: millis(total_timer.elapsed()),
            },
            graph_element_count,
            graph_edge_count,
            subtype_count,
            ownership_count,
            inherited_feature_count,
            requirement_count,
        })
    }

    pub fn from_artifact(artifact: RuntimeArtifact) -> Result<Self, RuntimeError> {
        Ok(Self {
            graph: Graph::from_artifact(artifact.graph)?,
            derived: artifact.derived,
            derived_feature_registry: DerivedFeatureRegistry::with_builtin_core_specs(),
            derived_feature_cache: DerivedFeatureCache::new("artifact"),
        })
    }

    pub fn into_artifact(self) -> RuntimeArtifact {
        RuntimeArtifact {
            graph: self.graph.artifact(),
            derived: self.derived,
        }
    }

    pub fn artifact(&self) -> RuntimeArtifact {
        RuntimeArtifact {
            graph: self.graph.artifact(),
            derived: self.derived.clone(),
        }
    }

    pub fn into_base(self, document: Arc<KirDocument>) -> RuntimeBase {
        RuntimeBase {
            document,
            graph: Arc::new(self.graph),
            derived: Arc::new(self.derived),
        }
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn derived(&self) -> &DerivedIndexes {
        &self.derived
    }

    pub fn derived_feature_revision(&self) -> &str {
        self.derived_feature_cache.revision()
    }

    pub fn derived_property(
        &self,
        element_id: &str,
        feature: &str,
    ) -> Result<QueryResult<Value>, RuntimeError> {
        let element = self
            .graph
            .element_by_element_id(element_id)
            .ok_or_else(|| RuntimeError::MissingElement(element_id.to_string()))?;
        let DerivedPropertyValue { value, source } = self
            .derived_feature_cache
            .derived_property(
                &self.derived_feature_registry,
                &self.graph,
                element,
                feature,
            )
            .ok_or_else(|| RuntimeError::MissingDerivedFeature {
                element: element_id.to_string(),
                feature: feature.to_string(),
            })?;
        Ok(QueryResult {
            value,
            explanation: vec![format!(
                "{feature} for {element_id} resolved from {source:?} at revision {}",
                self.derived_feature_cache.revision()
            )],
        })
    }

    pub fn get_subtypes(&self, type_id: &str) -> Result<QueryResult<Vec<String>>, RuntimeError> {
        let Some(type_node) = self.graph.node_id(type_id) else {
            return Err(RuntimeError::MissingElement(type_id.to_string()));
        };

        let subtypes = self.transitive_subtypes_of(type_node);
        let explanation = subtypes
            .iter()
            .map(|subtype| {
                if let Some(explanation) = self
                    .derived
                    .explanation_for("subtype", &[subtype.as_str(), type_id])
                {
                    format!("{subtype} derived by {}", explanation.rule_id)
                } else {
                    format!("{subtype} is a subtype of {type_id}")
                }
            })
            .collect();

        Ok(QueryResult {
            value: subtypes,
            explanation,
        })
    }

    pub fn get_features(&self, type_id: &str) -> Result<QueryResult<Vec<String>>, RuntimeError> {
        let Some(type_node) = self.graph.node_id(type_id) else {
            return Err(RuntimeError::MissingElement(type_id.to_string()));
        };

        let mut features = self
            .derived
            .inherited_features
            .iter()
            .filter_map(|(owner, feature)| (owner == type_id).then(|| feature.to_string()))
            .collect::<BTreeSet<_>>();
        for supertype in self.transitive_supertypes_of(type_node) {
            for (_, feature) in self
                .derived
                .inherited_features
                .iter()
                .filter(|(owner, _)| owner == &supertype)
            {
                features.insert(feature.clone());
            }
        }
        let features = features.into_iter().collect::<Vec<_>>();
        let explanation = features
            .iter()
            .map(|feature| {
                if let Some(explanation) = self
                    .derived
                    .explanation_for("inherited_feature", &[type_id, feature.as_str()])
                {
                    format!("{feature} derived by {}", explanation.rule_id)
                } else {
                    format!("{type_id} owns feature {feature}")
                }
            })
            .collect();

        Ok(QueryResult {
            value: features,
            explanation,
        })
    }

    fn transitive_subtypes_of(&self, type_node: NodeId) -> Vec<String> {
        let mut result = BTreeSet::new();
        let mut visited = BTreeSet::new();
        let mut stack = self
            .graph
            .incoming(type_node, "specializes")
            .map(|edge| edge.source)
            .collect::<Vec<_>>();

        while let Some(current) = stack.pop() {
            if !visited.insert(current) {
                continue;
            }
            if let Some(element_id) = self.graph.element_id(current) {
                result.insert(element_id.to_string());
            }
            for edge in self.graph.incoming(current, "specializes") {
                stack.push(edge.source);
            }
        }

        result.into_iter().collect()
    }

    fn transitive_supertypes_of(&self, type_node: NodeId) -> Vec<String> {
        let mut result = BTreeSet::new();
        let mut visited = BTreeSet::new();
        let mut stack = self
            .graph
            .outgoing(type_node, "specializes")
            .map(|edge| edge.target)
            .collect::<Vec<_>>();

        while let Some(current) = stack.pop() {
            if !visited.insert(current) {
                continue;
            }
            if let Some(element_id) = self.graph.element_id(current) {
                result.insert(element_id.to_string());
            }
            for edge in self.graph.outgoing(current, "specializes") {
                stack.push(edge.target);
            }
        }

        result.into_iter().collect()
    }

    pub fn evaluate(
        &self,
        feature_id: &str,
        owner_id: &str,
        context: &ExecutionContext,
    ) -> Result<QueryResult<Value>, RuntimeError> {
        let result = self.evaluate_result(feature_id, owner_id, context)?;
        Ok(QueryResult {
            value: result.value.into_legacy_value(),
            explanation: result.explanation,
        })
    }

    /// Evaluate a feature while preserving the distinction between result
    /// sequences and a single JSON array data value.
    pub fn evaluate_result(
        &self,
        feature_id: &str,
        owner_id: &str,
        context: &ExecutionContext,
    ) -> Result<QueryResult<ExpressionResult>, RuntimeError> {
        let mut state = RuntimeFeatureEvaluationState::default();
        state.enter(owner_id, feature_id)?;
        let feature = self
            .graph
            .element_by_element_id(feature_id)
            .ok_or_else(|| RuntimeError::MissingElement(feature_id.to_string()))?;
        if let Some(expression_ir) = feature.properties.get("expression_ir") {
            let value = self.evaluate_expression_ir_result(
                expression_ir, owner_id, context, &mut state,
            )?;
            return Ok(QueryResult {
                value,
                explanation: vec![
                    format!("read structured expression from {feature_id}"),
                    format!(
                        "evaluated against owner {owner_id} at context version {}",
                        context.version
                    ),
                ],
            });
        }

        let expression = feature
            .properties
            .get("expression")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                RuntimeError::InvalidExpression(format!("{feature_id} has no expression"))
            })?;

        let value = self.evaluate_expression_result(expression, owner_id, context, &mut state)?;
        Ok(QueryResult {
            value,
            explanation: vec![
                format!("read expression `{expression}` from {feature_id}"),
                format!(
                    "evaluated against owner {owner_id} at context version {}",
                    context.version
                ),
            ],
        })
    }

    pub fn explain<T>(&self, result: &QueryResult<T>) -> String {
        result.explanation.join(" -> ")
    }

    fn evaluate_expression_result(
        &self,
        expression: &str,
        owner_id: &str,
        context: &ExecutionContext,
        state: &mut RuntimeFeatureEvaluationState,
    ) -> Result<ExpressionResult, RuntimeError> {
        for function in ["count", "sum"] {
            if let Some(path) = parse_function(expression, function) {
                let path = path
                    .strip_prefix("self.")
                    .ok_or_else(|| RuntimeError::InvalidExpression(expression.to_string()))?;
                let expression = ExpressionIr::Call {
                    function: function.to_string(),
                    args: vec![ExpressionIr::Path {
                        root: crate::kir::ExpressionPathRoot::SelfRef,
                        segments: path
                            .split('.')
                            .map(|name| ExpressionPathSegment::Name(name.to_string()))
                            .collect(),
                    }],
                };
                return expression
                    .evaluate_result(&mut RuntimeExpressionEvaluationContext {
                        runtime: self,
                        owner_id,
                        context,
                        state,
                    })
                    .map_err(RuntimeError::from);
            }
        }
        Err(RuntimeError::InvalidExpression(expression.to_string()))
    }

    #[cfg(test)]
    fn evaluate_expression_ir(
        &self,
        expression: &Value,
        owner_id: &str,
        context: &ExecutionContext,
    ) -> Result<Value, RuntimeError> {
        self.evaluate_expression_ir_result(
            expression, owner_id, context, &mut RuntimeFeatureEvaluationState::default(),
        ).map(ExpressionResult::into_legacy_value)
    }

    fn evaluate_expression_ir_result(
        &self,
        expression: &Value,
        owner_id: &str,
        context: &ExecutionContext,
        state: &mut RuntimeFeatureEvaluationState,
    ) -> Result<ExpressionResult, RuntimeError> {
        let expression_ir = ExpressionIr::from_value(expression)
            .map_err(|err| RuntimeError::InvalidExpression(format!("{err}: {expression}")))?;
        let mut evaluation_context = RuntimeExpressionEvaluationContext {
            runtime: self,
            owner_id,
            context,
            state,
        };
        expression_ir
            .evaluate_result(&mut evaluation_context)
            .map_err(RuntimeError::from)
    }

    #[cfg(test)]
    fn resolve_path_segments(
        &self,
        owner_id: &str,
        segments: &[&str],
        context: &ExecutionContext,
    ) -> Result<Vec<Value>, RuntimeError> {
        self.resolve_path_segments_with_state(
            owner_id, segments, context, &mut RuntimeFeatureEvaluationState::default(),
        )
    }

    fn resolve_path_segments_with_state(
        &self,
        owner_id: &str,
        segments: &[&str],
        context: &ExecutionContext,
        state: &mut RuntimeFeatureEvaluationState,
    ) -> Result<Vec<Value>, RuntimeError> {
        if segments.is_empty() {
            return Err(RuntimeError::InvalidExpression("self".to_string()));
        }

        let mut current_ids = vec![owner_id.to_string()];

        for segment in &segments[..segments.len() - 1] {
            let mut next_ids = Vec::new();

            for current in &current_ids {
                let related = self.graph.relation_targets(current, segment)?;
                let features = self.named_feature_targets(current, segment)?;
                let declared = self
                    .graph
                    .element_by_element_id(current)
                    .is_some_and(|element| element.properties.contains_key(*segment));
                if related.is_empty() && features.is_empty() && !declared {
                    return Err(RuntimeError::InvalidExpression(format!(
                        "unresolved path {current}.{segment}"
                    )));
                }
                next_ids.extend(
                    related
                        .into_iter()
                        .map(|element| element.element_id.clone()),
                );
                for target in features {
                    push_unique(&mut next_ids, target);
                }
            }

            current_ids = next_ids;
        }

        let final_segment = segments
            .last()
            .ok_or_else(|| RuntimeError::InvalidExpression("self".to_string()))?;

        let mut values = Vec::new();
        for current in &current_ids {
            let key = (current.clone(), (*final_segment).to_string());
            if let Some(value) = context.values.get(&key) {
                if let Value::Array(items) = value {
                    values.extend(items.clone());
                } else {
                    values.push(value.clone());
                }
                continue;
            }

            if let Some(element) = self.graph.element_by_element_id(current) {
                if let Some(value) = element.properties.get(*final_segment) {
                    if let Value::Array(items) = value {
                        values.extend(items.clone());
                    } else {
                        values.push(value.clone());
                    }
                    continue;
                }
            }

            let related = self.graph.relation_targets(current, final_segment)?;
            let mut related_values = related
                .into_iter()
                .map(|element| Value::String(element.element_id.clone()))
                .collect::<Vec<_>>();
            if related_values.is_empty() {
                let mut feature_ids = Vec::new();
                for feature_id in self.named_feature_targets(current, final_segment)? {
                    push_unique(&mut feature_ids, feature_id);
                }
                if feature_ids.is_empty() {
                    return Err(RuntimeError::InvalidExpression(format!(
                        "unresolved path {}.{}",
                        current, final_segment
                    )));
                }
                for feature_id in feature_ids {
                    values.extend(
                        self.feature_result(&feature_id, current, context, state)?.values,
                    );
                }
            } else {
                values.append(&mut related_values);
            }
        }

        Ok(values)
    }

    fn named_feature_targets(
        &self,
        owner_id: &str,
        feature_name: &str,
    ) -> Result<Vec<String>, RuntimeError> {
        let mut matches = self.direct_named_feature_targets(owner_id, feature_name)?;
        if !matches.is_empty() {
            return Ok(matches);
        }

        for relation in ["type", "definition"] {
            for target in self.graph.relation_targets(owner_id, relation)? {
                for matched in
                    self.direct_named_feature_targets(&target.element_id, feature_name)?
                {
                    push_unique(&mut matches, matched);
                }
            }
            if !matches.is_empty() {
                return Ok(matches);
            }
        }

        for target in self.graph.relation_targets(owner_id, "specializes")? {
            for matched in self.direct_named_feature_targets(&target.element_id, feature_name)? {
                push_unique(&mut matches, matched);
            }
        }

        Ok(matches)
    }

    fn direct_named_feature_targets(
        &self,
        owner_id: &str,
        feature_name: &str,
    ) -> Result<Vec<String>, RuntimeError> {
        let mut matches = Vec::new();
        for relation in ["features", "members"] {
            for target in self.graph.relation_targets(owner_id, relation)? {
                if element_name_matches(&target.properties, feature_name) {
                    push_unique(&mut matches, target.element_id.clone());
                }
            }
        }
        Ok(matches)
    }

    fn resolved_feature_target(&self, identity: &str) -> Result<Option<String>, RuntimeError> {
        if self.graph.element_by_element_id(identity).is_some() {
            return Ok(Some(identity.to_string()));
        }
        // Cross-file compilation may retain the resolver's canonical feature
        // identity until its owning module is loaded. Match its full qualified
        // name exactly; never resolve a lexical capture by its short name here.
        let Some(qualified_name) = identity.strip_prefix("feature.") else {
            return Ok(None);
        };
        let mut matches = self.graph.elements().iter().filter(|element| {
            element.properties.get("qualified_name")
                .or_else(|| element.properties.get("qualifiedName"))
                .and_then(Value::as_str) == Some(qualified_name)
        });
        let Some(target) = matches.next() else {
            return Ok(None);
        };
        if matches.next().is_some() {
            return Err(RuntimeError::InvalidExpression(format!(
                "ambiguous resolved feature identity {identity}"
            )));
        }
        Ok(Some(target.element_id.clone()))
    }

    fn resolved_feature_result(
        &self,
        feature_id: &str,
        current_owner: &str,
        context: &ExecutionContext,
        state: &mut RuntimeFeatureEvaluationState,
    ) -> Result<ExpressionResult, RuntimeError> {
        let feature = self.graph.element_by_element_id(feature_id)
            .ok_or_else(|| RuntimeError::MissingElement(feature_id.to_string()))?;
        let lexical_owner = feature.properties.get("owner")
            .or_else(|| feature.properties.get("owning_type"))
            .and_then(Value::as_str);
        let instance_of_owner = lexical_owner.is_some_and(|owner| {
            self.graph.element_by_element_id(current_owner).is_some_and(|instance| {
                ["type", "definition"].iter().any(|key| {
                    instance.properties.get(*key).is_some_and(|value| {
                        value.as_str() == Some(owner) || value.as_array().is_some_and(|values| {
                            values.iter().any(|value| value.as_str() == Some(owner))
                        })
                    })
                })
            })
        });
        let owner_id = if instance_of_owner {
            current_owner
        } else {
            lexical_owner.unwrap_or(current_owner)
        };
        self.feature_result(feature_id, owner_id, context, state)
    }

    fn feature_result(
        &self,
        feature_id: &str,
        owner_id: &str,
        context: &ExecutionContext,
        state: &mut RuntimeFeatureEvaluationState,
    ) -> Result<ExpressionResult, RuntimeError> {
        let feature = self
            .graph
            .element_by_element_id(feature_id)
            .ok_or_else(|| RuntimeError::MissingElement(feature_id.to_string()))?;

        if let Some(name) = feature_name(&feature.properties)
            && let Some(value) = context
                .values
                .get(&(owner_id.to_string(), name.to_string()))
                .or_else(|| self.graph.element_by_element_id(owner_id)
                    .and_then(|owner| owner.properties.get(name)))
        {
            // ExecutionContext retains its legacy convention: arrays supply
            // multiple bindings. Compiled expressions use explicit results below.
            return Ok(ExpressionResult {
                values: match value {
                    Value::Array(items) => items.clone(),
                    value => vec![value.clone()],
                },
            });
        }

        if let Some(expression_ir) = feature.properties.get("expression_ir") {
            state.enter(owner_id, feature_id)?;
            let result = self.evaluate_expression_ir_result(expression_ir, owner_id, context, state);
            state.leave(owner_id, feature_id);
            return result;
        }

        Ok(ExpressionResult { values: vec![Value::String(feature_id.to_string())] })
    }
}

impl RuntimeBase {
    pub fn from_document(document: Arc<KirDocument>) -> Result<Self, RuntimeError> {
        let runtime = Runtime::from_document(document.as_ref().clone())?;
        Ok(runtime.into_base(document))
    }

    pub fn from_artifact(
        document: Arc<KirDocument>,
        artifact: RuntimeArtifact,
    ) -> Result<Self, RuntimeError> {
        let runtime = Runtime::from_artifact(artifact)?;
        Ok(runtime.into_base(document))
    }

    pub fn document(&self) -> &Arc<KirDocument> {
        &self.document
    }

    pub fn graph(&self) -> &Arc<Graph> {
        &self.graph
    }

    pub fn derived(&self) -> &Arc<DerivedIndexes> {
        &self.derived
    }
}

impl RuntimeOverlay {
    pub fn is_empty(&self) -> bool {
        self.added_elements.is_empty()
            && self.updated_properties.is_empty()
            && self.added_members.is_empty()
            && self.removed_elements.is_empty()
    }

    pub fn added_element_count(&self) -> usize {
        self.added_elements.len()
    }
}

impl LayeredRuntime {
    pub fn from_base_and_overlay(
        base: &RuntimeBase,
        overlay: &RuntimeOverlay,
    ) -> Result<Self, RuntimeError> {
        if overlay.is_empty() {
            return Ok(Self {
                graph: base.graph.clone(),
                derived: base.derived.clone(),
                overlay_element_count: 0,
                assembly: LayeredRuntimeAssembly::SharedBase,
            });
        }

        let start = Instant::now();
        let document = apply_runtime_overlay(base.document.as_ref(), overlay);
        let runtime = Runtime::from_document(document)?;
        Ok(Self {
            graph: Arc::new(runtime.graph),
            derived: Arc::new(runtime.derived),
            overlay_element_count: overlay.added_element_count(),
            assembly: LayeredRuntimeAssembly::OverlayMaterialized {
                elapsed_millis: millis(start.elapsed()),
            },
        })
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn graph_arc(&self) -> Arc<Graph> {
        self.graph.clone()
    }

    pub fn derived(&self) -> &DerivedIndexes {
        &self.derived
    }

    pub fn derived_arc(&self) -> Arc<DerivedIndexes> {
        self.derived.clone()
    }

    pub fn overlay_element_count(&self) -> usize {
        self.overlay_element_count
    }

    pub fn assembly(&self) -> &LayeredRuntimeAssembly {
        &self.assembly
    }
}

fn apply_runtime_overlay(base: &KirDocument, overlay: &RuntimeOverlay) -> KirDocument {
    let mut document = base.clone();
    document.metadata.remove("derived_feature_manifest");
    document.metadata.remove("merged_sources");
    document
        .elements
        .retain(|element| !overlay.removed_elements.contains(&element.id));

    let mut element_index = document
        .elements
        .iter()
        .enumerate()
        .map(|(index, element)| (element.id.clone(), index))
        .collect::<BTreeMap<_, _>>();

    for (element_id, properties) in &overlay.updated_properties {
        let Some(index) = element_index.get(element_id).copied() else {
            continue;
        };
        for (property, value) in properties {
            document.elements[index]
                .properties
                .insert(property.clone(), value.clone());
        }
    }

    for (owner_id, members) in &overlay.added_members {
        let Some(index) = element_index.get(owner_id).copied() else {
            continue;
        };
        let entry = document.elements[index]
            .properties
            .entry("members".to_string())
            .or_insert_with(|| Value::Array(Vec::new()));
        if let Value::Array(existing) = entry {
            existing.extend(members.iter().cloned().map(Value::String));
        }
    }

    for element in overlay.added_elements.values() {
        if element_index.contains_key(&element.id) {
            continue;
        }
        element_index.insert(element.id.clone(), document.elements.len());
        document.elements.push(element.clone());
    }

    document
}

fn millis(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

fn workspace_revision_fingerprint(document: &KirDocument) -> Result<String, serde_json::Error> {
    let bytes = serde_json::to_vec(document)?;
    Ok(stable_digest([(
        "kir-document".as_bytes(),
        bytes.as_slice(),
    )]))
}

fn stable_digest<'a, I>(chunks: I) -> String
where
    I: IntoIterator<Item = (&'a [u8], &'a [u8])>,
{
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET;
    for (label, bytes) in chunks {
        for byte in label
            .iter()
            .chain(&(bytes.len() as u64).to_le_bytes())
            .chain(bytes)
        {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(FNV_PRIME);
        }
    }

    format!("fnv1a64:{hash:016x}")
}

fn element_name_matches(properties: &ElementProperties, expected: &str) -> bool {
    feature_name(properties) == Some(expected)
}

fn feature_name(properties: &ElementProperties) -> Option<&str> {
    properties
        .get("declared_name")
        .or_else(|| properties.get("name"))
        .and_then(Value::as_str)
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.iter().any(|existing| existing == &value) {
        values.push(value);
    }
}

// These are deterministic execution-profile limits, not language multiplicities.
const MAX_RUNTIME_FEATURE_DEPTH: usize = 64;
const MAX_RUNTIME_FEATURE_VISITS: usize = 10_000;

struct RuntimeFeatureEvaluationState {
    active: BTreeSet<(String, String)>,
    remaining_visits: usize,
}

impl Default for RuntimeFeatureEvaluationState {
    fn default() -> Self {
        Self { active: BTreeSet::new(), remaining_visits: MAX_RUNTIME_FEATURE_VISITS }
    }
}

impl RuntimeFeatureEvaluationState {
    fn enter(&mut self, owner_id: &str, feature_id: &str) -> Result<(), RuntimeError> {
        let key = (owner_id.to_string(), feature_id.to_string());
        if self.active.contains(&key) {
            return Err(RuntimeError::InvalidExpression(format!(
                "feature dependency cycle at {owner_id}.{feature_id}"
            )));
        }
        if self.active.len() >= MAX_RUNTIME_FEATURE_DEPTH {
            return Err(ExpressionEvaluationError::ResourceLimitExceeded {
                resource: "feature dependency depth".to_string(),
                limit: MAX_RUNTIME_FEATURE_DEPTH,
            }.into());
        }
        if self.remaining_visits == 0 {
            return Err(ExpressionEvaluationError::ResourceLimitExceeded {
                resource: "feature dependency visits".to_string(),
                limit: MAX_RUNTIME_FEATURE_VISITS,
            }.into());
        }
        self.remaining_visits -= 1;
        self.active.insert(key);
        Ok(())
    }

    fn leave(&mut self, owner_id: &str, feature_id: &str) {
        self.active.remove(&(owner_id.to_string(), feature_id.to_string()));
    }
}

struct RuntimeExpressionEvaluationContext<'a> {
    runtime: &'a Runtime,
    owner_id: &'a str,
    context: &'a ExecutionContext,
    state: &'a mut RuntimeFeatureEvaluationState,
}

impl ExpressionEvaluationContext for RuntimeExpressionEvaluationContext<'_> {
    fn owner_id(&self) -> &str {
        self.owner_id
    }

    fn resolve_path(
        &mut self,
        segments: &[ExpressionPathSegment],
    ) -> Result<Vec<Value>, ExpressionEvaluationError> {
        if let [ExpressionPathSegment::Resolved { feature: Some(identity), .. }] = segments {
            let feature_id = self.runtime.resolved_feature_target(identity)
                .map_err(|error| ExpressionEvaluationError::InvalidExpression(error.to_string()))?
                .ok_or_else(|| ExpressionEvaluationError::MissingBinding(identity.clone()))?;
            let result = self.runtime.resolved_feature_result(
                &feature_id, self.owner_id, self.context, self.state,
            ).map_err(|error| ExpressionEvaluationError::InvalidExpression(error.to_string()))?;
            if result.values.iter().any(Value::is_null) {
                return Err(ExpressionEvaluationError::MissingBinding(identity.clone()));
            }
            return Ok(result.values);
        }
        let owned = segments
            .iter()
            .map(ExpressionPathSegment::name)
            .map(str::to_string)
            .collect::<Vec<_>>();
        let borrowed = owned.iter().map(String::as_str).collect::<Vec<_>>();
        let values = self
            .runtime
            .resolve_path_segments_with_state(self.owner_id, &borrowed, self.context, self.state)
            .map_err(|err| ExpressionEvaluationError::InvalidExpression(err.to_string()))?;
        if values.iter().any(Value::is_null) {
            return Err(ExpressionEvaluationError::MissingBinding(owned.join(".")));
        }
        Ok(values)
    }
}

fn parse_function<'a>(expression: &'a str, function: &str) -> Option<&'a str> {
    let prefix = format!("{function}(");
    expression
        .strip_prefix(&prefix)
        .and_then(|rest| rest.strip_suffix(')'))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::Arc;

    use serde_json::{Value, json};

    use super::{
        ExecutionContext, LayeredRuntime, LayeredRuntimeAssembly, Runtime, RuntimeBase,
        RuntimeOverlay,
    };
    use crate::model::{KIR_SCHEMA_VERSION, KirDocument, KirElement};

    #[test]
    fn shared_expression_runtime_bindings_and_collections() {
        let runtime = sample_runtime();
        let mut context = ExecutionContext::default();
        context
            .values
            .insert(("owner".into(), "x".into()), json!(8.0));
        context
            .values
            .insert(("owner".into(), "items".into()), json!([]));
        context
            .values
            .insert(("owner".into(), "unset".into()), Value::Null);
        for (expression, expected) in [
            (
                json!({"kind":"binary","op":"-","left":{"kind":"path","segments":["x"]},"right":{"kind":"literal","value":2}}),
                json!(6.0),
            ),
            (
                json!({"kind":"unary","op":"negate","operand":{"kind":"path","segments":["x"]}}),
                json!(-8.0),
            ),
            (
                json!({"kind":"call","function":"count","args":[{"kind":"path","segments":["items"]}]}),
                json!(0),
            ),
            (
                json!({"kind":"call","function":"sum","args":[{"kind":"path","segments":["items"]}]}),
                json!(0),
            ),
        ] {
            assert_eq!(
                runtime
                    .evaluate_expression_ir(&expression, "owner", &context)
                    .unwrap(),
                expected
            );
        }
        for name in ["missing", "unset"] {
            let expression = json!({"kind":"binary","op":"==","left":{"kind":"path","segments":[name]},"right":{"kind":"literal","value":null}});
            assert!(
                runtime
                    .evaluate_expression_ir(&expression, "owner", &context)
                    .is_err()
            );
        }
    }

    #[test]
    fn compiled_sequence_features_are_flat_path_values() {
        let runtime = Runtime::from_document(KirDocument {
            metadata: [("kir_schema_version".into(), json!(KIR_SCHEMA_VERSION))].into_iter().collect(),
            elements: vec![
                KirElement { id: "owner".into(), kind: "Package".into(), layer: 2,
                    properties: [("features".into(), json!(["items"]))].into_iter().collect() },
                KirElement { id: "items".into(), kind: "AttributeUsage".into(), layer: 2,
                    properties: [
                        ("declared_name".into(), json!("items")),
                        ("expression_ir".into(), json!({"kind":"tuple","items":[
                            {"kind":"literal","value":1}, {"kind":"literal","value":2}
                        ]})),
                    ].into_iter().collect() },
            ],
        }).unwrap();
        let context = ExecutionContext::default();
        assert_eq!(runtime.resolve_path_segments("owner", &["items"], &context).unwrap(), vec![json!(1), json!(2)]);
        let sum = json!({"kind":"call","function":"sum","args":[{"kind":"path","segments":["items"]}]});
        assert_eq!(runtime.evaluate_expression_ir(&sum, "owner", &context).unwrap(), json!(3));
    }

    fn expression_runtime(features: Vec<(&str, Value)>) -> Runtime {
        let feature_ids = features.iter().map(|(name, _)| *name).collect::<Vec<_>>();
        let mut elements = vec![KirElement {
            id: "owner".into(),
            kind: "Package".into(),
            layer: 2,
            properties: [("features".into(), json!(feature_ids))].into_iter().collect(),
        }];
        elements.extend(features.into_iter().map(|(name, expression)| KirElement {
            id: name.into(),
            kind: "AttributeUsage".into(),
            layer: 2,
            properties: [
                ("declared_name".into(), json!(name)),
                ("expression_ir".into(), expression),
            ].into_iter().collect(),
        }));
        Runtime::from_document(KirDocument {
            metadata: [("kir_schema_version".into(), json!(KIR_SCHEMA_VERSION))].into_iter().collect(),
            elements,
        }).unwrap()
    }

    #[test]
    fn compiled_array_data_remains_one_result_through_feature_references() {
        let runtime = expression_runtime(vec![
            ("array", json!({"kind":"literal","value":[1,2]})),
            ("alias", json!({"kind":"path","segments":["array"]})),
            ("count", json!({"kind":"call","function":"count","args":[
                {"kind":"path","segments":["alias"]}
            ]})),
        ]);
        let context = ExecutionContext::default();
        for feature in ["array", "alias"] {
            let result = runtime.evaluate_result(feature, "owner", &context).unwrap();
            assert_eq!(result.value.values, vec![json!([1, 2])]);
            assert_eq!(result.value.cardinality(), 1);
            assert_eq!(runtime.evaluate(feature, "owner", &context).unwrap().value, json!([1, 2]));
        }
        assert_eq!(runtime.evaluate("count", "owner", &context).unwrap().value, json!(1));
    }

    #[test]
    fn compiled_empty_and_sequence_results_survive_feature_references() {
        let runtime = expression_runtime(vec![
            ("empty", json!({"kind":"literal","value":null})),
            ("items", json!({"kind":"tuple","items":[
                {"kind":"literal","value":1}, {"kind":"literal","value":2}
            ]})),
            ("combined", json!({"kind":"tuple","items":[
                {"kind":"path","segments":["empty"]}, {"kind":"path","segments":["items"]}
            ]})),
        ]);
        let context = ExecutionContext::default();
        assert!(runtime.evaluate_result("empty", "owner", &context).unwrap().value.values.is_empty());
        assert_eq!(runtime.evaluate_result("combined", "owner", &context).unwrap().value.values,
            vec![json!(1), json!(2)]);
    }

    #[test]
    fn compiled_feature_cycles_report_without_poisoning_later_evaluations() {
        let runtime = expression_runtime(vec![
            ("a", json!({"kind":"path","segments":["b"]})),
            ("b", json!({"kind":"path","segments":["a"]})),
            ("self", json!({"kind":"path","segments":["self"]})),
            ("leaf", json!({"kind":"literal","value":3})),
            ("repeated", json!({"kind":"binary","op":"+",
                "left":{"kind":"path","segments":["leaf"]},
                "right":{"kind":"path","segments":["leaf"]}})),
        ]);
        let context = ExecutionContext::default();
        for feature in ["a", "self", "a"] {
            let error = runtime.evaluate(feature, "owner", &context).unwrap_err().to_string();
            assert!(error.contains("feature dependency cycle"), "{error}");
            assert!(error.contains(&format!("owner.{feature}")), "{error}");
        }
        assert_eq!(runtime.evaluate("repeated", "owner", &context).unwrap().value, json!(6));
    }

    #[test]
    fn compiled_feature_dependency_depth_has_a_deterministic_limit() {
        let names = (0..=super::MAX_RUNTIME_FEATURE_DEPTH).map(|i| format!("f{i}")).collect::<Vec<_>>();
        let features = names.iter().enumerate().map(|(i, name)| {
            let expression = match names.get(i + 1) {
                Some(next) => json!({"kind":"path","segments":[next]}),
                None => json!({"kind":"literal","value":1}),
            };
            (name.as_str(), expression)
        }).collect();
        let runtime = expression_runtime(features);
        let context = ExecutionContext::default();
        let error = runtime.evaluate(&names[0], "owner", &context).unwrap_err().to_string();
        assert!(error.contains("feature dependency depth"), "{error}");
        // Exactly the supported depth remains evaluable.
        assert_eq!(runtime.evaluate(&names[1], "owner", &context).unwrap().value, json!(1));
    }

    #[test]
    fn invocation_lexical_capture_uses_resolved_identity_despite_caller_shadow() {
        for identity in ["attribute.Library.offset", "feature.Library.offset"] {
            let runtime = Runtime::from_document(KirDocument {
                metadata: [("kir_schema_version".into(), json!(KIR_SCHEMA_VERSION))].into_iter().collect(),
                elements: vec![
                    KirElement { id:"Library".into(), kind:"Package".into(), layer:2,
                        properties:BTreeMap::new() },
                    KirElement { id:"Caller".into(), kind:"Package".into(), layer:2,
                        properties:[("offset".into(), json!(99))].into_iter().collect() },
                    KirElement { id:"attribute.Library.offset".into(), kind:"AttributeUsage".into(), layer:2,
                        properties:[
                            ("owner".into(), json!("Library")),
                            ("qualified_name".into(), json!("Library.offset")),
                            ("declared_name".into(), json!("offset")),
                            ("expression_ir".into(), json!({"kind":"literal","value":10})),
                        ].into_iter().collect() },
                    KirElement { id:"result".into(), kind:"AttributeUsage".into(), layer:2,
                        properties:[("expression_ir".into(), json!({"kind":"invoke",
                            "function":"type.Library.F", "bindings":[],
                            "body":{"kind":"path","segments":[{"name":"offset","feature":identity}]}
                        }))].into_iter().collect() },
                ],
            }).unwrap();
            let mut context = ExecutionContext::default();
            context.values.insert(("Caller".into(), "offset".into()), json!(123));
            assert_eq!(runtime.evaluate("result", "Caller", &context).unwrap().value, json!(10));
            context.values.insert(("Library".into(), "offset".into()), json!(15));
            assert_eq!(runtime.evaluate("result", "Caller", &context).unwrap().value, json!(15));
        }
    }

    #[test]
    fn missing_resolved_capture_cannot_fall_back_to_caller_name() {
        let runtime = expression_runtime(vec![
            ("offset", json!({"kind":"literal","value":99})),
            ("result", json!({"kind":"invoke","function":"type.Library.F","bindings":[],
                "body":{"kind":"path","segments":[
                    {"name":"offset","feature":"feature.Missing.offset"}
                ]}})),
        ]);
        let context = ExecutionContext::default();
        let error = runtime.evaluate("result", "owner", &context).unwrap_err().to_string();
        assert!(error.contains("unresolved expression path: feature.Missing.offset"), "{error}");
    }

    #[test]
    fn resolved_type_feature_keeps_current_instance_values() {
        let runtime = Runtime::from_document(KirDocument {
            metadata:[("kir_schema_version".into(), json!(KIR_SCHEMA_VERSION))].into_iter().collect(),
            elements:vec![
                KirElement { id:"Device".into(), kind:"PartDefinition".into(), layer:2,
                    properties:BTreeMap::new() },
                KirElement { id:"device".into(), kind:"PartUsage".into(), layer:2,
                    properties:[("type".into(), json!("Device")), ("temperature".into(), json!(25))].into_iter().collect() },
                KirElement { id:"attribute.Device.temperature".into(), kind:"AttributeUsage".into(), layer:2,
                    properties:[
                        ("owner".into(), json!("Device")), ("declared_name".into(), json!("temperature")),
                        ("expression_ir".into(), json!({"kind":"literal","value":10})),
                    ].into_iter().collect() },
                KirElement { id:"result".into(), kind:"AttributeUsage".into(), layer:2,
                    properties:[("expression_ir".into(), json!({"kind":"path","segments":[
                        {"name":"temperature","feature":"attribute.Device.temperature"}
                    ]}))].into_iter().collect() },
            ],
        }).unwrap();
        let mut context = ExecutionContext::default();
        assert_eq!(runtime.evaluate("result", "device", &context).unwrap().value, json!(25));
        context.values.insert(("device".into(), "temperature".into()), json!(30));
        assert_eq!(runtime.evaluate("result", "device", &context).unwrap().value, json!(30));
    }

    fn sample_runtime() -> Runtime {
        Runtime::from_document(KirDocument {
            metadata: [("kir_schema_version".to_string(), json!(KIR_SCHEMA_VERSION))]
                .into_iter()
                .collect(),
            elements: vec![
                KirElement {
                    id: "Core::Core::Type".to_string(),
                    kind: "model.Type".to_string(),
                    layer: 1,
                    properties: [("qualified_name".to_string(), json!("Core.Type"))]
                        .into_iter()
                        .collect(),
                },
                KirElement {
                    id: "Model::Systems::PartDefinition".to_string(),
                    kind: "model.PartDefinition".to_string(),
                    layer: 1,
                    properties: [
                        (
                            "qualified_name".to_string(),
                            json!("Model.Systems.PartDefinition"),
                        ),
                        ("specializes".to_string(), json!(["Core::Core::Type"])),
                        ("features".to_string(), json!(["df.partCount"])),
                    ]
                    .into_iter()
                    .collect(),
                },
                KirElement {
                    id: "type.Vehicle".to_string(),
                    kind: "model.PartDefinition".to_string(),
                    layer: 2,
                    properties: [
                        ("qualified_name".to_string(), json!("Vehicle")),
                        (
                            "specializes".to_string(),
                            json!(["Model::Systems::PartDefinition"]),
                        ),
                        ("features".to_string(), json!(["feature.engine"])),
                    ]
                    .into_iter()
                    .collect(),
                },
                KirElement {
                    id: "type.Car".to_string(),
                    kind: "model.PartDefinition".to_string(),
                    layer: 2,
                    properties: [
                        ("qualified_name".to_string(), json!("Car")),
                        ("specializes".to_string(), json!(["type.Vehicle"])),
                    ]
                    .into_iter()
                    .collect(),
                },
                KirElement {
                    id: "feature.engine".to_string(),
                    kind: "model.PartUsage".to_string(),
                    layer: 2,
                    properties: [("qualified_name".to_string(), json!("Vehicle.engine"))]
                        .into_iter()
                        .collect(),
                },
                KirElement {
                    id: "df.partCount".to_string(),
                    kind: "Core::Core::Feature".to_string(),
                    layer: 1,
                    properties: [(
                        "qualified_name".to_string(),
                        json!("PartDefinition.partCount"),
                    )]
                    .into_iter()
                    .collect(),
                },
                KirElement {
                    id: "Base::Anything".to_string(),
                    kind: "model.Type".to_string(),
                    layer: 1,
                    properties: [
                        ("qualified_name".to_string(), json!("Base.Anything")),
                        ("doc".to_string(), json!({ "source": "foundation" })),
                    ]
                    .into_iter()
                    .collect(),
                },
                KirElement {
                    id: "assembly.VehicleInstance".to_string(),
                    kind: "type.Vehicle".to_string(),
                    layer: 2,
                    properties: [
                        (
                            "qualified_name".to_string(),
                            json!("assembly.VehicleInstance"),
                        ),
                        (
                            "parts".to_string(),
                            json!(["part.engine_left", "part.engine_right"]),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                },
                KirElement {
                    id: "part.engine_left".to_string(),
                    kind: "type.Engine".to_string(),
                    layer: 2,
                    properties: [("qualified_name".to_string(), json!("assembly.leftEngine"))]
                        .into_iter()
                        .collect(),
                },
                KirElement {
                    id: "part.engine_right".to_string(),
                    kind: "type.Engine".to_string(),
                    layer: 2,
                    properties: [("qualified_name".to_string(), json!("assembly.rightEngine"))]
                        .into_iter()
                        .collect(),
                },
                aggregate_feature("df.totalMass", "sum"),
            ],
        })
        .unwrap()
    }

    fn layered_base_document() -> KirDocument {
        KirDocument {
            metadata: [("kir_schema_version".to_string(), json!(KIR_SCHEMA_VERSION))]
                .into_iter()
                .collect(),
            elements: vec![
                KirElement {
                    id: "pkg.Base".to_string(),
                    kind: "model.Package".to_string(),
                    layer: 1,
                    properties: [
                        ("qualified_name".to_string(), json!("Base")),
                        ("members".to_string(), json!(["part.Base.Vehicle"])),
                    ]
                    .into_iter()
                    .collect(),
                },
                KirElement {
                    id: "part.Base.Vehicle".to_string(),
                    kind: "model.PartDefinition".to_string(),
                    layer: 1,
                    properties: [
                        ("qualified_name".to_string(), json!("Base.Vehicle")),
                        ("owner".to_string(), json!("pkg.Base")),
                    ]
                    .into_iter()
                    .collect(),
                },
            ],
        }
    }

    #[test]
    fn layered_runtime_empty_overlay_shares_base_graph_and_indexes() {
        let document = Arc::new(layered_base_document());
        let base = RuntimeBase::from_document(document).unwrap();
        let layered =
            LayeredRuntime::from_base_and_overlay(&base, &RuntimeOverlay::default()).unwrap();

        assert_eq!(layered.assembly(), &LayeredRuntimeAssembly::SharedBase);
        assert!(Arc::ptr_eq(base.graph(), &layered.graph_arc()));
        assert!(Arc::ptr_eq(base.derived(), &layered.derived_arc()));
        assert_eq!(layered.overlay_element_count(), 0);
    }

    #[test]
    fn layered_runtime_overlay_matches_flat_document_build() {
        let document = Arc::new(layered_base_document());
        let base = RuntimeBase::from_document(document.clone()).unwrap();
        let mut overlay = RuntimeOverlay::default();
        overlay
            .added_members
            .insert("pkg.Base".to_string(), vec!["part.Base.Wheel".to_string()]);
        overlay.added_elements.insert(
            "part.Base.Wheel".to_string(),
            KirElement {
                id: "part.Base.Wheel".to_string(),
                kind: "model.PartDefinition".to_string(),
                layer: 2,
                properties: [
                    ("qualified_name".to_string(), json!("Base.Wheel")),
                    ("owner".to_string(), json!("pkg.Base")),
                ]
                .into_iter()
                .collect(),
            },
        );

        let layered = LayeredRuntime::from_base_and_overlay(&base, &overlay).unwrap();
        let mut flat = document.as_ref().clone();
        flat.elements[0].properties.insert(
            "members".to_string(),
            json!(["part.Base.Vehicle", "part.Base.Wheel"]),
        );
        flat.elements
            .push(overlay.added_elements["part.Base.Wheel"].clone());
        let flat_runtime = Runtime::from_document(flat).unwrap();

        assert!(matches!(
            layered.assembly(),
            LayeredRuntimeAssembly::OverlayMaterialized { .. }
        ));
        assert_eq!(
            layered.graph().elements().len(),
            flat_runtime.graph().elements().len()
        );
        assert_eq!(
            layered.graph().edge_count(),
            flat_runtime.graph().edge_count()
        );
        assert_eq!(
            layered.derived().ownership.len(),
            flat_runtime.derived().ownership.len()
        );
    }

    fn aggregate_feature(id: &str, function: &str) -> KirElement {
        KirElement {
            id: id.to_string(),
            kind: "Core::Core::Feature".to_string(),
            layer: 2,
            properties: [(
                "expression_ir".to_string(),
                json!({
                    "kind": "call",
                    "function": function,
                    "args": [{
                        "kind": "path",
                        "root": "self",
                        "segments": ["parts", "mass"]
                    }]
                }),
            )]
            .into_iter()
            .collect(),
        }
    }

    #[test]
    fn finds_transitive_subtypes() {
        let runtime = sample_runtime();

        let result = runtime.get_subtypes("Core::Core::Type").unwrap();
        assert!(
            result
                .value
                .contains(&"Model::Systems::PartDefinition".to_string())
        );
        assert!(result.value.contains(&"type.Vehicle".to_string()));
    }

    #[test]
    fn representative_example_builds_runtime_indexes() {
        let runtime =
            Runtime::from_document(KirDocument::representative_example().unwrap()).unwrap();

        assert!(
            runtime
                .graph()
                .element_by_element_id("activity.Example.Startup")
                .is_some()
        );
        let package = runtime.graph().node_id("pkg.Example").unwrap();
        let activity = runtime.graph().node_id("activity.Example.Startup").unwrap();
        assert!(
            runtime
                .graph()
                .outgoing(package, "members")
                .any(|edge| edge.target == activity)
        );
    }

    #[test]
    fn inherits_features_across_specialization() {
        let runtime = sample_runtime();

        let result = runtime.get_features("type.Car").unwrap();
        assert!(result.value.contains(&"feature.engine".to_string()));
        assert!(result.value.contains(&"df.partCount".to_string()));
    }

    #[test]
    fn derives_documentation_on_request() {
        let runtime = Runtime::from_document(KirDocument {
            metadata: Default::default(),
            elements: vec![
                KirElement {
                    id: "type.Demo.A".to_string(),
                    kind: "Model::Systems::PartDefinition".to_string(),
                    layer: 2,
                    properties: Default::default(),
                },
                KirElement {
                    id: "doc.type.Demo.A.1".to_string(),
                    kind: "Core::Root::Documentation".to_string(),
                    layer: 2,
                    properties: [
                        ("owner".to_string(), json!("type.Demo.A")),
                        ("body".to_string(), json!("doc from A")),
                    ]
                    .into_iter()
                    .collect(),
                },
            ],
        })
        .unwrap();

        assert!(!runtime.derived_feature_revision().is_empty());
        assert_eq!(
            runtime
                .derived_property("type.Demo.A", "documentation")
                .unwrap()
                .value,
            json!("doc.type.Demo.A.1")
        );
        assert_eq!(
            runtime
                .derived_property("type.Demo.A", "ownedElement")
                .unwrap()
                .value,
            json!(["doc.type.Demo.A.1"])
        );
        assert_eq!(
            runtime
                .derived_property("doc.type.Demo.A.1", "documentedElement")
                .unwrap()
                .value,
            json!("type.Demo.A")
        );
    }

    #[test]
    fn loads_derived_feature_manifest_from_document_metadata() {
        let runtime = Runtime::from_document(KirDocument {
            metadata: [(
                "derived_feature_manifest".to_string(),
                json!({
                    "metamodel": "test",
                    "derived_features": [
                        {
                            "owner": "*",
                            "feature": "label",
                            "kind": "name"
                        }
                    ]
                }),
            )]
            .into_iter()
            .collect(),
            elements: vec![KirElement {
                id: "type.Demo.A".to_string(),
                kind: "Model::Systems::PartDefinition".to_string(),
                layer: 2,
                properties: [("declared_name".to_string(), json!("A"))]
                    .into_iter()
                    .collect(),
            }],
        })
        .unwrap();

        assert_eq!(
            runtime
                .derived_property("type.Demo.A", "label")
                .unwrap()
                .value,
            json!("A")
        );
    }

    #[test]
    fn evaluates_derived_feature_against_overlay_context() {
        let runtime = sample_runtime();
        let mut context = ExecutionContext {
            values: std::collections::HashMap::new(),
            version: 7,
        };

        context.values.insert(
            ("part.engine_left".to_string(), "mass".to_string()),
            json!(120.5),
        );
        context.values.insert(
            ("part.engine_right".to_string(), "mass".to_string()),
            json!(130.0),
        );

        let result = runtime
            .evaluate("df.totalMass", "assembly.VehicleInstance", &context)
            .unwrap();
        assert_eq!(result.value, Value::from(250.5));
    }

    #[test]
    fn imported_stdlib_documentation_remains_passive_metadata() {
        let runtime = sample_runtime();
        let anything = runtime
            .graph()
            .element_by_element_id("Base::Anything")
            .unwrap();

        assert_eq!(anything.properties["doc"]["source"], "foundation");
        assert!(anything.properties.get("specializes").is_none());
    }

    #[test]
    fn evaluates_structured_expression_ir_against_overlay_context() {
        let document = KirDocument {
            metadata: Default::default(),
            elements: vec![
                KirElement {
                    id: "part.engine_left".to_string(),
                    kind: "type.Engine".to_string(),
                    layer: 2,
                    properties: Default::default(),
                },
                KirElement {
                    id: "part.engine_right".to_string(),
                    kind: "type.Engine".to_string(),
                    layer: 2,
                    properties: Default::default(),
                },
                KirElement {
                    id: "assembly.VehicleInstance".to_string(),
                    kind: "type.Vehicle".to_string(),
                    layer: 2,
                    properties: [(
                        "parts".to_string(),
                        json!(["part.engine_left", "part.engine_right"]),
                    )]
                    .into_iter()
                    .collect(),
                },
                KirElement {
                    id: "df.totalMass".to_string(),
                    kind: "Core::Core::Feature".to_string(),
                    layer: 2,
                    properties: [(
                        "expression_ir".to_string(),
                        json!({
                            "kind": "call",
                            "function": "sum",
                            "args": [{
                                "kind": "path",
                                "root": "self",
                                "segments": ["parts", "mass"]
                            }]
                        }),
                    )]
                    .into_iter()
                    .collect(),
                },
            ],
        };
        let runtime = Runtime::from_document(document).unwrap();
        let mut context = ExecutionContext {
            values: std::collections::HashMap::new(),
            version: 11,
        };

        context.values.insert(
            ("part.engine_left".to_string(), "mass".to_string()),
            json!(120.5),
        );
        context.values.insert(
            ("part.engine_right".to_string(), "mass".to_string()),
            json!(130.0),
        );

        let result = runtime
            .evaluate("df.totalMass", "assembly.VehicleInstance", &context)
            .unwrap();
        assert_eq!(result.value, Value::from(250.5));
    }

    #[test]
    fn evaluates_structured_numeric_aggregate_functions() {
        let document = KirDocument {
            metadata: Default::default(),
            elements: vec![
                KirElement {
                    id: "part.engine_left".to_string(),
                    kind: "type.Engine".to_string(),
                    layer: 2,
                    properties: Default::default(),
                },
                KirElement {
                    id: "part.engine_center".to_string(),
                    kind: "type.Engine".to_string(),
                    layer: 2,
                    properties: Default::default(),
                },
                KirElement {
                    id: "part.engine_right".to_string(),
                    kind: "type.Engine".to_string(),
                    layer: 2,
                    properties: Default::default(),
                },
                KirElement {
                    id: "assembly.VehicleInstance".to_string(),
                    kind: "type.Vehicle".to_string(),
                    layer: 2,
                    properties: [(
                        "parts".to_string(),
                        json!([
                            "part.engine_left",
                            "part.engine_center",
                            "part.engine_right"
                        ]),
                    )]
                    .into_iter()
                    .collect(),
                },
                aggregate_feature("df.minMass", "min"),
                aggregate_feature("df.maxMass", "max"),
                aggregate_feature("df.avgMass", "avg"),
            ],
        };
        let runtime = Runtime::from_document(document).unwrap();
        let mut context = ExecutionContext::default();
        context.values.insert(
            ("part.engine_left".to_string(), "mass".to_string()),
            json!(100.0),
        );
        context.values.insert(
            ("part.engine_center".to_string(), "mass".to_string()),
            json!(125.0),
        );
        context.values.insert(
            ("part.engine_right".to_string(), "mass".to_string()),
            json!(150.0),
        );

        assert_eq!(
            runtime
                .evaluate("df.minMass", "assembly.VehicleInstance", &context)
                .unwrap()
                .value,
            Value::from(100.0)
        );
        assert_eq!(
            runtime
                .evaluate("df.maxMass", "assembly.VehicleInstance", &context)
                .unwrap()
                .value,
            Value::from(150.0)
        );
        assert_eq!(
            runtime
                .evaluate("df.avgMass", "assembly.VehicleInstance", &context)
                .unwrap()
                .value,
            Value::from(125.0)
        );
    }

    #[test]
    fn evaluates_structured_tuple_expression_ir() {
        let document = KirDocument {
            metadata: Default::default(),
            elements: vec![
                KirElement {
                    id: "assembly.VehicleInstance".to_string(),
                    kind: "type.Vehicle".to_string(),
                    layer: 2,
                    properties: Default::default(),
                },
                KirElement {
                    id: "df.tupleValue".to_string(),
                    kind: "Core::Core::Feature".to_string(),
                    layer: 2,
                    properties: [(
                        "expression_ir".to_string(),
                        json!({
                            "kind": "tuple",
                            "items": [
                                {"kind": "literal", "value": 1},
                                {"kind": "literal", "value": true},
                                {"kind": "literal", "value": "ready"}
                            ]
                        }),
                    )]
                    .into_iter()
                    .collect(),
                },
            ],
        };
        let runtime = Runtime::from_document(document).unwrap();

        let result = runtime
            .evaluate(
                "df.tupleValue",
                "assembly.VehicleInstance",
                &ExecutionContext::default(),
            )
            .unwrap();
        assert_eq!(result.value, json!([1, true, "ready"]));
    }

    #[test]
    fn reports_unsupported_expression_ir_kind() {
        let document = KirDocument {
            metadata: Default::default(),
            elements: vec![
                KirElement {
                    id: "assembly.VehicleInstance".to_string(),
                    kind: "type.Vehicle".to_string(),
                    layer: 2,
                    properties: Default::default(),
                },
                KirElement {
                    id: "df.unsupported".to_string(),
                    kind: "Core::Core::Feature".to_string(),
                    layer: 2,
                    properties: [(
                        "expression_ir".to_string(),
                        json!({"kind": "select", "source": {"kind": "self"}}),
                    )]
                    .into_iter()
                    .collect(),
                },
            ],
        };
        let runtime = Runtime::from_document(document).unwrap();

        let error = runtime
            .evaluate(
                "df.unsupported",
                "assembly.VehicleInstance",
                &ExecutionContext::default(),
            )
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("unsupported expression_ir kind `select`")
        );
    }

    #[test]
    fn reports_unsupported_expression_ir_function() {
        let document = KirDocument {
            metadata: Default::default(),
            elements: vec![
                KirElement {
                    id: "assembly.VehicleInstance".to_string(),
                    kind: "type.Vehicle".to_string(),
                    layer: 2,
                    properties: Default::default(),
                },
                KirElement {
                    id: "df.unsupported".to_string(),
                    kind: "Core::Core::Feature".to_string(),
                    layer: 2,
                    properties: [(
                        "expression_ir".to_string(),
                        json!({
                            "kind": "call",
                            "function": "median",
                            "args": [{"kind": "literal", "value": 1}]
                        }),
                    )]
                    .into_iter()
                    .collect(),
                },
            ],
        };
        let runtime = Runtime::from_document(document).unwrap();

        let error = runtime
            .evaluate(
                "df.unsupported",
                "assembly.VehicleInstance",
                &ExecutionContext::default(),
            )
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("unsupported expression_ir function `median`")
        );
    }

    #[test]
    fn rejects_nonnumeric_values_in_structured_sum() {
        let document = KirDocument {
            metadata: Default::default(),
            elements: vec![
                KirElement {
                    id: "part.engine_left".to_string(),
                    kind: "type.Engine".to_string(),
                    layer: 2,
                    properties: Default::default(),
                },
                KirElement {
                    id: "assembly.VehicleInstance".to_string(),
                    kind: "type.Vehicle".to_string(),
                    layer: 2,
                    properties: [("parts".to_string(), json!(["part.engine_left"]))]
                        .into_iter()
                        .collect(),
                },
                KirElement {
                    id: "df.totalMass".to_string(),
                    kind: "Core::Core::Feature".to_string(),
                    layer: 2,
                    properties: [(
                        "expression_ir".to_string(),
                        json!({
                            "kind": "call",
                            "function": "sum",
                            "args": [{
                                "kind": "path",
                                "root": "self",
                                "segments": [{"name": "parts", "feature": "feature.parts"}, {"name": "mass", "feature": "feature.mass"}]
                            }]
                        }),
                    )]
                    .into_iter()
                    .collect(),
                },
            ],
        };
        let runtime = Runtime::from_document(document).unwrap();
        let mut context = ExecutionContext::default();
        context.values.insert(
            ("part.engine_left".to_string(), "mass".to_string()),
            json!("heavy"),
        );

        let error = runtime
            .evaluate("df.totalMass", "assembly.VehicleInstance", &context)
            .unwrap_err();
        assert!(matches!(error, super::RuntimeError::NonNumericValue { .. }));
    }

    #[test]
    fn evaluates_feature_path_defaults_from_type_members() {
        let document = KirDocument {
            metadata: Default::default(),
            elements: vec![
                KirElement {
                    id: "type.EvalDemo.Engine".to_string(),
                    kind: "Model::Systems::PartDefinition".to_string(),
                    layer: 2,
                    properties: [(
                        "features".to_string(),
                        json!(["feature.EvalDemo.Engine.mass"]),
                    )]
                    .into_iter()
                    .collect(),
                },
                KirElement {
                    id: "feature.EvalDemo.Engine.mass".to_string(),
                    kind: "Core::Core::Feature".to_string(),
                    layer: 2,
                    properties: [
                        ("declared_name".to_string(), json!("mass")),
                        (
                            "expression_ir".to_string(),
                            json!({"kind": "literal", "value": 4.0}),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                },
                KirElement {
                    id: "type.EvalDemo.Vehicle".to_string(),
                    kind: "Model::Systems::PartDefinition".to_string(),
                    layer: 2,
                    properties: [(
                        "features".to_string(),
                        json!([
                            "feature.EvalDemo.Vehicle.leftEngine",
                            "feature.EvalDemo.Vehicle.rightEngine"
                        ]),
                    )]
                    .into_iter()
                    .collect(),
                },
                KirElement {
                    id: "feature.EvalDemo.Vehicle.leftEngine".to_string(),
                    kind: "Model::Parts::PartUsage".to_string(),
                    layer: 2,
                    properties: [
                        ("declared_name".to_string(), json!("leftEngine")),
                        ("type".to_string(), json!("type.EvalDemo.Engine")),
                    ]
                    .into_iter()
                    .collect(),
                },
                KirElement {
                    id: "feature.EvalDemo.Vehicle.rightEngine".to_string(),
                    kind: "Model::Parts::PartUsage".to_string(),
                    layer: 2,
                    properties: [
                        ("declared_name".to_string(), json!("rightEngine")),
                        ("type".to_string(), json!("type.EvalDemo.Engine")),
                    ]
                    .into_iter()
                    .collect(),
                },
                KirElement {
                    id: "feature.EvalDemo.Vehicle.totalMass".to_string(),
                    kind: "Core::Core::Feature".to_string(),
                    layer: 2,
                    properties: [(
                        "expression_ir".to_string(),
                        json!({
                            "kind": "binary",
                            "op": "add",
                            "left": {
                                "kind": "call",
                                "function": "sum",
                                "args": [{
                                    "kind": "path",
                                    "root": "self",
                                    "segments": ["leftEngine", "mass"]
                                }]
                            },
                            "right": {
                                "kind": "call",
                                "function": "sum",
                                "args": [{
                                    "kind": "path",
                                    "root": "self",
                                    "segments": ["rightEngine", "mass"]
                                }]
                            }
                        }),
                    )]
                    .into_iter()
                    .collect(),
                },
            ],
        };
        let runtime = Runtime::from_document(document).unwrap();

        let result = runtime
            .evaluate(
                "feature.EvalDemo.Vehicle.totalMass",
                "type.EvalDemo.Vehicle",
                &ExecutionContext::default(),
            )
            .unwrap();

        assert_eq!(result.value, Value::from(8.0));
    }
}
