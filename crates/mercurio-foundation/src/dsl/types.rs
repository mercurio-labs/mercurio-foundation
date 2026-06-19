use std::collections::BTreeMap;
use std::sync::Arc;

use rhai::{Array, Dynamic, Engine, EvalAltResult, FnPtr, Map, NativeCallContext};
use serde_json::Value;

use super::DslQueryResult;
use crate::graph::{Element, Graph, NodeId};
use crate::ir::{KirDocument, KirElement};

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
        self.elem()
            .and_then(|element| element.properties.get(&name))
            .cloned()
            .map(serde_json_to_dynamic)
            .unwrap_or(Dynamic::UNIT)
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

pub fn register_types(engine: &mut Engine) {
    engine
        .register_type_with_name::<ModelContext>("ModelContext")
        .register_fn("parts", ModelContext::parts)
        .register_fn("elements", ModelContext::elements)
        .register_fn("element", ModelContext::element)
        .register_fn("match_pattern", ModelContext::match_pattern);

    engine
        .register_type_with_name::<ElementSet>("ElementSet")
        .register_fn("count", ElementSet::count)
        .register_fn("first", ElementSet::first)
        .register_fn("collect", ElementSet::collect_elements)
        .register_fn("select", ElementSet::select_fields)
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
    match column {
        "id" | "element_id" => Value::String(element.element_id.clone()),
        "kind" => Value::String(element.kind.as_ref().to_string()),
        "layer" => Value::from(element.layer),
        property => element
            .properties
            .get(property)
            .cloned()
            .unwrap_or(Value::Null),
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
