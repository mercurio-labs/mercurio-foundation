use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use rhai::{Array, Dynamic, Engine, EvalAltResult, FnPtr, Map, NativeCallContext};
use serde::Serialize;
use serde_json::Value;

use super::{DslExtensionSpec, DslQueryResult, rhai_dynamic_to_json};
use crate::graph::{Element, Graph, NodeId};
use crate::identity::workspace_revision_for_kir_document;
use crate::ir::{KirDocument, KirElement};
use crate::mutation::{ElementRef, SemanticMutation};
use crate::transaction::{SemanticTransaction, TransactionOperation};

#[derive(Debug, Clone)]
pub struct ModelContext {
    graph: Arc<Graph>,
}

impl ModelContext {
    pub fn new(graph: Arc<Graph>) -> Self {
        Self { graph }
    }

    pub fn parts(&mut self) -> ElementSet {
        let ids = self
            .graph
            .elements()
            .iter()
            .filter(|element| element.layer == 2)
            .map(|element| element.id)
            .collect();
        ElementSet {
            ids,
            graph: Arc::clone(&self.graph),
        }
    }

    pub fn elements(&mut self) -> ElementSet {
        let ids = self
            .graph
            .elements()
            .iter()
            .map(|element| element.id)
            .collect();
        ElementSet {
            ids,
            graph: Arc::clone(&self.graph),
        }
    }

    pub fn element(&mut self, element_id: String) -> Dynamic {
        match self.graph.element_by_element_id(&element_id) {
            Some(element) => Dynamic::from(DslElement {
                id: element.id,
                graph: Arc::clone(&self.graph),
            }),
            None => Dynamic::UNIT,
        }
    }

    pub fn elements_where_contains_any(&mut self, field: &str, expected: &[String]) -> ElementSet {
        let ids = self
            .graph
            .elements()
            .iter()
            .filter(|element| {
                let value = element_property_json(element, field);
                expected
                    .iter()
                    .any(|expected| value_contains(&value, expected))
            })
            .map(|element| element.id)
            .collect();
        ElementSet {
            ids,
            graph: Arc::clone(&self.graph),
        }
    }

    pub fn match_pattern(&mut self, pattern: String) -> Result<Array, Box<EvalAltResult>> {
        let query = crate::query::parse_query(&pattern).map_err(|error| {
            Box::new(EvalAltResult::ErrorRuntime(
                error.to_string().into(),
                rhai::Position::NONE,
            ))
        })?;
        let document = graph_to_kir_document(&self.graph);
        let result_set = crate::query::QueryEngine::new(&document)
            .execute(&query)
            .map_err(|error| {
                Box::new(EvalAltResult::ErrorRuntime(
                    error.to_string().into(),
                    rhai::Position::NONE,
                ))
            })?;

        Ok(result_set
            .rows
            .into_iter()
            .map(|row| {
                let map = row
                    .into_iter()
                    .map(|(key, value)| (key.into(), serde_json_to_dynamic(value)))
                    .collect::<Map>();
                Dynamic::from(map)
            })
            .collect())
    }

    pub fn capabilities(&mut self) -> Array {
        builtin_capabilities()
            .into_iter()
            .map(|capability| Dynamic::from(capability.to_map()))
            .collect()
    }

    pub fn capability(&mut self, id: String) -> DslCapability {
        DslCapability {
            id,
            graph: Arc::clone(&self.graph),
        }
    }

    pub fn requires(&mut self, id: String, version: String) -> Map {
        let available = builtin_capability(&id).is_some();
        map_from_entries([
            ("id", Dynamic::from(id)),
            ("version", Dynamic::from(version)),
            ("available", Dynamic::from(available)),
        ])
    }

    pub fn changes(&mut self) -> DslChangeSetBuilder {
        DslChangeSetBuilder::default()
    }

    pub fn transaction(&mut self, label: String) -> DslTransactionBuilder {
        DslTransactionBuilder {
            label,
            graph: Arc::clone(&self.graph),
            operations: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElementSet {
    pub(crate) ids: Vec<NodeId>,
    pub(crate) graph: Arc<Graph>,
}

impl ElementSet {
    pub fn count(&mut self) -> i64 {
        self.ids.len() as i64
    }

    pub fn first(&mut self) -> Dynamic {
        match self.ids.first().copied() {
            Some(id) => Dynamic::from(DslElement {
                id,
                graph: Arc::clone(&self.graph),
            }),
            None => Dynamic::UNIT,
        }
    }

    pub fn collect_elements(&mut self) -> Array {
        self.ids
            .iter()
            .copied()
            .map(|id| {
                Dynamic::from(DslElement {
                    id,
                    graph: Arc::clone(&self.graph),
                })
            })
            .collect()
    }

    pub fn select_fields(&mut self, fields: Array) -> DslQueryResult {
        let columns = fields
            .into_iter()
            .filter_map(|field| field.try_cast::<String>())
            .collect::<Vec<_>>();
        let rows = self
            .ids
            .iter()
            .filter_map(|id| self.graph.element(*id))
            .map(|element| {
                columns
                    .iter()
                    .map(|column| element_property_json(element, column))
                    .collect()
            })
            .collect();

        DslQueryResult { columns, rows }
    }

    pub fn where_eq(&mut self, field: String, expected: Dynamic) -> ElementSet {
        let expected = rhai_dynamic_to_json(expected);
        self.filter_by_property(&field, |value| value == expected)
    }

    pub fn where_ne(&mut self, field: String, expected: Dynamic) -> ElementSet {
        let expected = rhai_dynamic_to_json(expected);
        self.filter_by_property(&field, |value| value != expected)
    }

    pub fn where_contains(&mut self, field: String, expected: String) -> ElementSet {
        self.filter_by_property(&field, |value| value_contains(&value, &expected))
    }

    pub fn where_in(&mut self, field: String, expected: Array) -> ElementSet {
        let expected = expected
            .into_iter()
            .map(rhai_dynamic_to_json)
            .collect::<Vec<_>>();
        self.filter_by_property(&field, |value| expected.iter().any(|item| item == &value))
    }

    pub fn order_by(&mut self, field: String) -> ElementSet {
        self.order_by_field(&field, false)
    }

    pub fn order_by_desc(&mut self, field: String) -> ElementSet {
        self.order_by_field(&field, true)
    }

    pub fn related(&mut self, relation: String) -> ElementSet {
        let ids = self
            .ids
            .iter()
            .flat_map(|id| self.graph.outgoing(*id, &relation).map(|edge| edge.target))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        ElementSet {
            ids,
            graph: Arc::clone(&self.graph),
        }
    }

    pub fn select_related(
        &mut self,
        relation: String,
        source_fields: Array,
        target_fields: Array,
    ) -> DslQueryResult {
        self.select_related_filtered(relation, None, source_fields, target_fields)
    }

    pub fn select_related_where_eq(
        &mut self,
        relation: String,
        target_field: String,
        expected: Dynamic,
        source_fields: Array,
        target_fields: Array,
    ) -> DslQueryResult {
        self.select_related_filtered(
            relation,
            Some((target_field, rhai_dynamic_to_json(expected))),
            source_fields,
            target_fields,
        )
    }

    fn filter_by_property(
        &self,
        field: &str,
        mut predicate: impl FnMut(Value) -> bool,
    ) -> ElementSet {
        let ids = self
            .ids
            .iter()
            .filter_map(|id| {
                let element = self.graph.element(*id)?;
                predicate(element_property_json(element, field)).then_some(*id)
            })
            .collect();
        ElementSet {
            ids,
            graph: Arc::clone(&self.graph),
        }
    }

    fn order_by_field(&self, field: &str, descending: bool) -> ElementSet {
        let mut ids = self.ids.clone();
        ids.sort_by(|left, right| {
            let ordering = match (self.graph.element(*left), self.graph.element(*right)) {
                (Some(left), Some(right)) => compare_json_values(
                    &element_property_json(left, field),
                    &element_property_json(right, field),
                ),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            };
            let ordering = if descending {
                ordering.reverse()
            } else {
                ordering
            };
            ordering.then_with(|| left.cmp(right))
        });
        ElementSet {
            ids,
            graph: Arc::clone(&self.graph),
        }
    }

    fn select_related_filtered(
        &self,
        relation: String,
        target_filter: Option<(String, Value)>,
        source_fields: Array,
        target_fields: Array,
    ) -> DslQueryResult {
        let source_fields = string_array(source_fields);
        let target_fields = string_array(target_fields);
        let columns = source_fields
            .iter()
            .map(|field| format!("source.{field}"))
            .chain(target_fields.iter().map(|field| format!("target.{field}")))
            .collect::<Vec<_>>();
        let mut rows = Vec::new();

        for source_id in &self.ids {
            let Some(source) = self.graph.element(*source_id) else {
                continue;
            };
            for edge in self.graph.outgoing(*source_id, &relation) {
                let Some(target) = self.graph.element(edge.target) else {
                    continue;
                };
                if let Some((field, expected)) = &target_filter
                    && element_property_json(target, field) != *expected
                {
                    continue;
                }

                let row = source_fields
                    .iter()
                    .map(|field| element_property_json(source, field))
                    .chain(
                        target_fields
                            .iter()
                            .map(|field| element_property_json(target, field)),
                    )
                    .collect();
                rows.push(row);
            }
        }

        DslQueryResult { columns, rows }
    }

    pub fn to_query_result(&self) -> DslQueryResult {
        let columns = vec!["element_id".into(), "kind".into(), "layer".into()];
        let rows = self
            .ids
            .iter()
            .filter_map(|id| self.graph.element(*id))
            .map(|element| {
                vec![
                    Value::String(element.element_id.clone()),
                    Value::String(element.kind.as_ref().to_string()),
                    Value::from(element.layer),
                ]
            })
            .collect();
        DslQueryResult { columns, rows }
    }
}

#[derive(Debug, Clone)]
pub struct DslElement {
    pub(crate) id: NodeId,
    pub(crate) graph: Arc<Graph>,
}

impl DslElement {
    fn elem(&self) -> Option<&Element> {
        self.graph.element(self.id)
    }

    pub fn get_id(&mut self) -> String {
        self.elem()
            .map(|element| element.element_id.clone())
            .unwrap_or_default()
    }

    pub fn get_kind(&mut self) -> String {
        self.elem()
            .map(|element| element.kind.as_ref().to_string())
            .unwrap_or_default()
    }

    pub fn get_layer(&mut self) -> i64 {
        self.elem().map_or(0, |element| i64::from(element.layer))
    }

    pub fn property(&mut self, name: String) -> Dynamic {
        if let Some(value) = self
            .elem()
            .and_then(|element| element.properties.get(&name))
            .cloned()
        {
            return serde_json_to_dynamic(value);
        }

        self.owned_feature_property(&name).unwrap_or(Dynamic::UNIT)
    }

    fn owned_feature_property(&self, name: &str) -> Option<Dynamic> {
        for relation in ["features", "members"] {
            for edge in self.graph.outgoing(self.id, relation) {
                let Some(feature) = self.graph.element(edge.target) else {
                    continue;
                };
                if string_property(feature, "declared_name") != Some(name) {
                    continue;
                }
                if let Some(value) = literal_expression_value(feature) {
                    return Some(serde_json_to_dynamic(value));
                }
                if let Some(value) = feature.properties.get("value").cloned() {
                    return Some(serde_json_to_dynamic(value));
                }
            }
        }
        None
    }

    pub fn outgoing(&mut self, relation: String) -> ElementSet {
        let ids = self
            .graph
            .outgoing(self.id, &relation)
            .map(|edge| edge.target)
            .collect();
        ElementSet {
            ids,
            graph: Arc::clone(&self.graph),
        }
    }

    pub fn incoming(&mut self, relation: String) -> ElementSet {
        let ids = self
            .graph
            .incoming(self.id, &relation)
            .map(|edge| edge.source)
            .collect();
        ElementSet {
            ids,
            graph: Arc::clone(&self.graph),
        }
    }

    pub fn outgoing_edges(&mut self) -> Array {
        self.graph
            .outgoing_edges(self.id)
            .map(|edge| {
                Dynamic::from(DslEdge {
                    source: edge.source,
                    target: edge.target,
                    relation: edge.relation.as_ref().to_string(),
                    graph: Arc::clone(&self.graph),
                })
            })
            .collect()
    }

    pub fn incoming_edges(&mut self) -> Array {
        self.graph
            .incoming_edges(self.id)
            .map(|edge| {
                Dynamic::from(DslEdge {
                    source: edge.source,
                    target: edge.target,
                    relation: edge.relation.as_ref().to_string(),
                    graph: Arc::clone(&self.graph),
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct DslEdge {
    pub(crate) source: NodeId,
    pub(crate) target: NodeId,
    pub(crate) relation: String,
    pub(crate) graph: Arc<Graph>,
}

impl DslEdge {
    pub fn get_relation(&mut self) -> String {
        self.relation.clone()
    }

    pub fn get_source_id(&mut self) -> String {
        self.graph
            .element_id(self.source)
            .map(str::to_string)
            .unwrap_or_default()
    }

    pub fn get_target_id(&mut self) -> String {
        self.graph
            .element_id(self.target)
            .map(str::to_string)
            .unwrap_or_default()
    }

    pub fn source_element(&mut self) -> Dynamic {
        Dynamic::from(DslElement {
            id: self.source,
            graph: Arc::clone(&self.graph),
        })
    }

    pub fn target_element(&mut self) -> Dynamic {
        Dynamic::from(DslElement {
            id: self.target,
            graph: Arc::clone(&self.graph),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DslCapability {
    id: String,
    graph: Arc<Graph>,
}

impl DslCapability {
    pub fn get_id(&mut self) -> String {
        self.id.clone()
    }

    pub fn available(&mut self) -> bool {
        builtin_capability(&self.id).is_some()
    }

    pub fn readiness(&mut self) -> Map {
        match builtin_capability(&self.id) {
            Some(capability) => map_from_entries([
                ("id", Dynamic::from(capability.id)),
                ("status", Dynamic::from("ready")),
                ("deterministic", Dynamic::from(capability.deterministic)),
            ]),
            None => map_from_entries([
                ("id", Dynamic::from(self.id.clone())),
                ("status", Dynamic::from("missing")),
                ("deterministic", Dynamic::from(false)),
            ]),
        }
    }

    pub fn run(&mut self, parameters: Map) -> Map {
        let element_count = self.graph.elements().len() as i64;
        match builtin_capability(&self.id) {
            Some(capability) => map_from_entries([
                ("id", Dynamic::from(capability.id)),
                ("status", Dynamic::from("passed")),
                ("deterministic", Dynamic::from(capability.deterministic)),
                ("element_count", Dynamic::from(element_count)),
                ("parameters", Dynamic::from(parameters)),
            ]),
            None => map_from_entries([
                ("id", Dynamic::from(self.id.clone())),
                ("status", Dynamic::from("missing")),
                ("deterministic", Dynamic::from(false)),
                ("element_count", Dynamic::from(element_count)),
                ("parameters", Dynamic::from(parameters)),
            ]),
        }
    }
}

#[derive(Debug, Clone)]
struct DslCapabilityDescriptor {
    id: String,
    name: String,
    deterministic: bool,
}

impl DslCapabilityDescriptor {
    fn to_map(self) -> Map {
        map_from_entries([
            ("id", Dynamic::from(self.id)),
            ("name", Dynamic::from(self.name)),
            ("deterministic", Dynamic::from(self.deterministic)),
        ])
    }
}

#[derive(Debug, Clone, Default)]
pub struct DslChangeSetBuilder {
    operations: Vec<SemanticMutation>,
}

impl DslChangeSetBuilder {
    pub fn rename(&mut self, element: String, new_name: String) -> Self {
        self.operations.push(SemanticMutation::RenameDeclaration {
            element: ElementRef::new(element),
            new_name,
        });
        self.clone()
    }

    pub fn set_attribute(&mut self, element: String, attribute: String, value: Dynamic) -> Self {
        self.operations.push(SemanticMutation::SetAttribute {
            element: ElementRef::new(element),
            attribute,
            value: rhai_dynamic_to_json(value),
        });
        self.clone()
    }

    pub fn preview(&mut self) -> Map {
        let operations = self
            .operations
            .iter()
            .map(|operation| match serde_json::to_value(operation) {
                Ok(value) => serde_json_to_dynamic(value),
                Err(_) => Dynamic::UNIT,
            })
            .collect::<Array>();
        map_from_entries([
            ("kind", Dynamic::from("change_set_preview")),
            (
                "operation_count",
                Dynamic::from(self.operations.len() as i64),
            ),
            ("applies_changes", Dynamic::from(false)),
            ("operations", Dynamic::from(operations)),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct DslBuildPlan {
    name: String,
    tasks: BTreeMap<String, DslBuildTask>,
}

impl DslBuildPlan {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tasks: BTreeMap::new(),
        }
    }

    pub fn task(&mut self, name: String) -> Self {
        self.tasks.entry(name).or_default();
        self.clone()
    }

    pub fn depends_on(&mut self, task: String, dependency: String) -> Self {
        self.tasks
            .entry(task)
            .or_default()
            .dependencies
            .insert(dependency.clone());
        self.tasks.entry(dependency).or_default();
        self.clone()
    }

    pub fn operation(&mut self, task: String, operation: String) -> Self {
        self.tasks
            .entry(task)
            .or_default()
            .operations
            .push(operation);
        self.clone()
    }

    pub fn plan(&mut self) -> Map {
        let tasks = self
            .tasks
            .iter()
            .map(|(name, task)| {
                let dependencies = task
                    .dependencies
                    .iter()
                    .cloned()
                    .map(Dynamic::from)
                    .collect::<Array>();
                let operations = task
                    .operations
                    .iter()
                    .cloned()
                    .map(Dynamic::from)
                    .collect::<Array>();
                Dynamic::from(map_from_entries([
                    ("name", Dynamic::from(name.clone())),
                    ("dependencies", Dynamic::from(dependencies)),
                    ("operations", Dynamic::from(operations)),
                ]))
            })
            .collect::<Array>();
        map_from_entries([
            ("kind", Dynamic::from("build_plan")),
            ("name", Dynamic::from(self.name.clone())),
            ("task_count", Dynamic::from(self.tasks.len() as i64)),
            ("tasks", Dynamic::from(tasks)),
        ])
    }
}

#[derive(Debug, Clone, Default)]
struct DslBuildTask {
    dependencies: BTreeSet<String>,
    operations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DslTransactionBuilder {
    label: String,
    graph: Arc<Graph>,
    operations: Vec<TransactionOperation>,
}

impl DslTransactionBuilder {
    pub fn rename(&mut self, element: String, new_name: String) -> Self {
        self.operations
            .push(TransactionOperation::semantic_mutation(
                SemanticMutation::RenameDeclaration {
                    element: ElementRef::new(element),
                    new_name,
                },
            ));
        self.clone()
    }

    pub fn set_attribute(&mut self, element: String, attribute: String, value: Dynamic) -> Self {
        self.operations
            .push(TransactionOperation::semantic_mutation(
                SemanticMutation::SetAttribute {
                    element: ElementRef::new(element),
                    attribute,
                    value: rhai_dynamic_to_json(value),
                },
            ));
        self.clone()
    }

    pub fn capability(&mut self, capability_id: String, parameters: Map) -> Self {
        self.operations.push(TransactionOperation::capability_run(
            capability_id,
            rhai_dynamic_to_json(Dynamic::from(parameters)),
        ));
        self.clone()
    }

    pub fn build_task(&mut self, task: String) -> Self {
        self.operations.push(TransactionOperation::build_task(
            task,
            Vec::new(),
            Vec::new(),
        ));
        self.clone()
    }

    pub fn build_operation(&mut self, task: String, operation: String) -> Self {
        self.operations.push(TransactionOperation::build_task(
            task,
            Vec::new(),
            vec![operation],
        ));
        self.clone()
    }

    pub fn build_depends_on(&mut self, task: String, dependency: String) -> Self {
        self.operations.push(TransactionOperation::build_task(
            task,
            vec![dependency],
            Vec::new(),
        ));
        self.clone()
    }

    pub fn preview(&mut self) -> Map {
        let transaction = SemanticTransaction::new(
            self.label.clone(),
            graph_revision(&self.graph),
            self.operations.clone(),
        );
        serializable_to_map(transaction.preview_report(Default::default()))
    }

    pub fn commit(&mut self) -> Map {
        let transaction = SemanticTransaction::new(
            self.label.clone(),
            graph_revision(&self.graph),
            self.operations.clone(),
        );
        serializable_to_map(transaction.rejected_report(
            "DSL_TRANSACTION_HOST_PERMISSION",
            "DSL transactions require an explicit host-approved commit operation",
        ))
    }
}

pub fn register_types(engine: &mut Engine) {
    engine
        .register_type_with_name::<ModelContext>("ModelContext")
        .register_fn("parts", ModelContext::parts)
        .register_fn("elements", ModelContext::elements)
        .register_fn("element", ModelContext::element)
        .register_fn("match_pattern", ModelContext::match_pattern)
        .register_fn("capabilities", ModelContext::capabilities)
        .register_fn("capability", ModelContext::capability)
        .register_fn("requires", ModelContext::requires)
        .register_fn("changes", ModelContext::changes)
        .register_fn("transaction", ModelContext::transaction);

    engine
        .register_type_with_name::<ElementSet>("ElementSet")
        .register_fn("count", ElementSet::count)
        .register_fn("first", ElementSet::first)
        .register_fn("collect", ElementSet::collect_elements)
        .register_fn("select", ElementSet::select_fields)
        .register_fn("where_eq", ElementSet::where_eq)
        .register_fn("where_ne", ElementSet::where_ne)
        .register_fn("where_contains", ElementSet::where_contains)
        .register_fn("where_in", ElementSet::where_in)
        .register_fn("order_by", ElementSet::order_by)
        .register_fn("order_by_desc", ElementSet::order_by_desc)
        .register_fn("related", ElementSet::related)
        .register_fn("select_related", ElementSet::select_related)
        .register_fn(
            "select_related_where_eq",
            ElementSet::select_related_where_eq,
        )
        .register_fn("where", filter_where)
        .register_fn("map", map_elements);

    engine
        .register_type_with_name::<DslElement>("Element")
        .register_get("id", DslElement::get_id)
        .register_get("kind", DslElement::get_kind)
        .register_get("layer", DslElement::get_layer)
        .register_fn("property", DslElement::property)
        .register_fn("outgoing", DslElement::outgoing)
        .register_fn("incoming", DslElement::incoming)
        .register_fn("outgoing_edges", DslElement::outgoing_edges)
        .register_fn("incoming_edges", DslElement::incoming_edges);

    engine
        .register_type_with_name::<DslEdge>("Edge")
        .register_get("relation", DslEdge::get_relation)
        .register_get("source_id", DslEdge::get_source_id)
        .register_get("target_id", DslEdge::get_target_id)
        .register_fn("source", DslEdge::source_element)
        .register_fn("target", DslEdge::target_element);

    engine
        .register_type_with_name::<DslCapability>("Capability")
        .register_get("id", DslCapability::get_id)
        .register_fn("available", DslCapability::available)
        .register_fn("readiness", DslCapability::readiness)
        .register_fn("run", DslCapability::run);

    engine
        .register_type_with_name::<DslChangeSetBuilder>("ChangeSetBuilder")
        .register_fn("rename", DslChangeSetBuilder::rename)
        .register_fn("set_attribute", DslChangeSetBuilder::set_attribute)
        .register_fn("preview", DslChangeSetBuilder::preview);

    engine
        .register_type_with_name::<DslBuildPlan>("BuildPlan")
        .register_fn("task", DslBuildPlan::task)
        .register_fn("depends_on", DslBuildPlan::depends_on)
        .register_fn("operation", DslBuildPlan::operation)
        .register_fn("plan", DslBuildPlan::plan);

    engine.register_fn("build", DslBuildPlan::new);

    engine
        .register_type_with_name::<DslTransactionBuilder>("TransactionBuilder")
        .register_fn("rename", DslTransactionBuilder::rename)
        .register_fn("set_attribute", DslTransactionBuilder::set_attribute)
        .register_fn("capability", DslTransactionBuilder::capability)
        .register_fn("build_task", DslTransactionBuilder::build_task)
        .register_fn("build_operation", DslTransactionBuilder::build_operation)
        .register_fn("build_depends_on", DslTransactionBuilder::build_depends_on)
        .register_fn("preview", DslTransactionBuilder::preview)
        .register_fn("commit", DslTransactionBuilder::commit);
}

pub fn register_extensions(engine: &mut Engine, extensions: &[DslExtensionSpec]) {
    for model_set in extensions
        .iter()
        .flat_map(|extension| extension.model_sets.iter())
    {
        let name = model_set.name.clone();
        let field = Arc::new(model_set.field.clone());
        let contains_any = Arc::new(model_set.contains_any.clone());
        engine.register_fn(name, move |context: &mut ModelContext| {
            context.elements_where_contains_any(field.as_str(), contains_any.as_slice())
        });
    }
}

fn filter_where(
    context: NativeCallContext,
    set: &mut ElementSet,
    predicate: FnPtr,
) -> Result<ElementSet, Box<EvalAltResult>> {
    let mut kept = Vec::new();
    for id in &set.ids {
        let element = DslElement {
            id: *id,
            graph: Arc::clone(&set.graph),
        };
        if predicate.call_within_context::<bool>(&context, (element,))? {
            kept.push(*id);
        }
    }
    Ok(ElementSet {
        ids: kept,
        graph: Arc::clone(&set.graph),
    })
}

fn map_elements(
    context: NativeCallContext,
    set: &mut ElementSet,
    mapper: FnPtr,
) -> Result<Array, Box<EvalAltResult>> {
    set.ids
        .iter()
        .map(|id| {
            let element = DslElement {
                id: *id,
                graph: Arc::clone(&set.graph),
            };
            mapper.call_within_context::<Dynamic>(&context, (element,))
        })
        .collect()
}

fn element_property_json(element: &Element, column: &str) -> Value {
    let mut segments = column.split('.');
    let Some(first) = segments.next() else {
        return Value::Null;
    };
    let Some(mut value) = element_root_property_json(element, first) else {
        return Value::Null;
    };

    for segment in segments {
        value = match value {
            Value::Object(map) => map.get(segment).cloned().unwrap_or(Value::Null),
            _ => return Value::Null,
        };
    }

    value
}

fn element_root_property_json(element: &Element, property: &str) -> Option<Value> {
    match property {
        "id" | "element_id" => Some(Value::String(element.element_id.clone())),
        "kind" => Some(Value::String(element.kind.as_ref().to_string())),
        "layer" => Some(Value::from(element.layer)),
        "qualified_name" => element
            .properties
            .get(property)
            .cloned()
            .or_else(|| qualified_name_from_element_id(&element.element_id).map(Value::String)),
        "metatype" => element.properties.get(property).cloned().or_else(|| {
            element
                .properties
                .get("metadata")
                .and_then(|metadata| metadata.get("metatype"))
                .cloned()
        }),
        property => element.properties.get(property).cloned(),
    }
}

fn qualified_name_from_element_id(element_id: &str) -> Option<String> {
    let (_, qualified_name) = element_id.split_once('.')?;
    (!qualified_name.is_empty()).then(|| qualified_name.to_string())
}

fn string_array(values: Array) -> Vec<String> {
    values
        .into_iter()
        .filter_map(|value| value.try_cast::<String>())
        .collect()
}

fn builtin_capabilities() -> Vec<DslCapabilityDescriptor> {
    vec![
        DslCapabilityDescriptor {
            id: "mercurio.dsl.analysis".to_string(),
            name: "DSL analysis".to_string(),
            deterministic: true,
        },
        DslCapabilityDescriptor {
            id: "mercurio.dsl.mutation-preview".to_string(),
            name: "DSL mutation preview".to_string(),
            deterministic: true,
        },
    ]
}

fn builtin_capability(id: &str) -> Option<DslCapabilityDescriptor> {
    builtin_capabilities()
        .into_iter()
        .find(|capability| capability.id == id)
}

fn map_from_entries<const N: usize>(entries: [(&str, Dynamic); N]) -> Map {
    entries
        .into_iter()
        .map(|(key, value)| (key.into(), value))
        .collect()
}

fn value_contains(value: &Value, expected: &str) -> bool {
    match value {
        Value::String(value) => value.contains(expected),
        Value::Array(values) => values.iter().any(|value| value_contains(value, expected)),
        Value::Object(values) => values.values().any(|value| value_contains(value, expected)),
        _ => false,
    }
}

fn compare_json_values(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Null, Value::Null) => Ordering::Equal,
        (Value::Null, _) => Ordering::Less,
        (_, Value::Null) => Ordering::Greater,
        (Value::Bool(left), Value::Bool(right)) => left.cmp(right),
        (Value::Number(left), Value::Number(right)) => match (left.as_f64(), right.as_f64()) {
            (Some(left), Some(right)) => left.partial_cmp(&right).unwrap_or(Ordering::Equal),
            _ => left.to_string().cmp(&right.to_string()),
        },
        (Value::String(left), Value::String(right)) => left.cmp(right),
        (Value::Array(left), Value::Array(right)) => left.len().cmp(&right.len()),
        (Value::Object(left), Value::Object(right)) => left.len().cmp(&right.len()),
        (left, right) => value_kind_rank(left)
            .cmp(&value_kind_rank(right))
            .then_with(|| left.to_string().cmp(&right.to_string())),
    }
}

fn value_kind_rank(value: &Value) -> u8 {
    match value {
        Value::Null => 0,
        Value::Bool(_) => 1,
        Value::Number(_) => 2,
        Value::String(_) => 3,
        Value::Array(_) => 4,
        Value::Object(_) => 5,
    }
}

fn string_property<'a>(element: &'a Element, name: &str) -> Option<&'a str> {
    element.properties.get(name).and_then(Value::as_str)
}

fn literal_expression_value(element: &Element) -> Option<Value> {
    let expression = element.properties.get("expression_ir")?;
    if expression.get("kind").and_then(Value::as_str) == Some("literal") {
        expression.get("value").cloned()
    } else {
        None
    }
}

fn graph_to_kir_document(graph: &Graph) -> KirDocument {
    KirDocument {
        metadata: BTreeMap::new(),
        elements: graph
            .elements()
            .iter()
            .map(|element| KirElement {
                id: element.element_id.clone(),
                kind: element.kind.as_ref().to_string(),
                layer: element.layer,
                properties: element.properties.to_btree_map(),
            })
            .collect(),
    }
}

fn graph_revision(graph: &Graph) -> Option<crate::mutation::WorkspaceRevision> {
    workspace_revision_for_kir_document(&graph_to_kir_document(graph)).ok()
}

fn serializable_to_map(value: impl Serialize) -> Map {
    match serde_json::to_value(value) {
        Ok(value) => json_to_map(value),
        Err(error) => map_from_entries([(
            "error",
            Dynamic::from(format!("could not serialize DSL value: {error}")),
        )]),
    }
}

fn json_to_map(value: Value) -> Map {
    match serde_json_to_dynamic(value).try_cast::<Map>() {
        Some(map) => map,
        None => Map::new(),
    }
}

fn serde_json_to_dynamic(value: Value) -> Dynamic {
    match value {
        Value::String(value) => Dynamic::from(value),
        Value::Number(value) => value
            .as_i64()
            .map(Dynamic::from)
            .or_else(|| value.as_f64().map(Dynamic::from))
            .unwrap_or(Dynamic::UNIT),
        Value::Bool(value) => Dynamic::from(value),
        Value::Array(values) => Dynamic::from(
            values
                .into_iter()
                .map(serde_json_to_dynamic)
                .collect::<Array>(),
        ),
        Value::Object(values) => Dynamic::from(
            values
                .into_iter()
                .map(|(key, value)| (key.into(), serde_json_to_dynamic(value)))
                .collect::<Map>(),
        ),
        Value::Null => Dynamic::UNIT,
    }
}
