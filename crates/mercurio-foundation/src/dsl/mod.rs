mod stdlib;
mod types;

use std::collections::BTreeSet;
use std::sync::Arc;

use rhai::{Engine, EvalAltResult, Scope};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use crate::graph::Graph;
use crate::ir::{KirFieldKind, KirFieldRegistry};
use types::ModelContext;

pub use types::{DslEdge, DslElement, ElementSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslQueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Value>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DslError(String);

impl std::fmt::Display for DslError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for DslError {}

impl From<Box<EvalAltResult>> for DslError {
    fn from(error: Box<EvalAltResult>) -> Self {
        Self(error.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslFieldSchema {
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslSchema {
    pub element_kinds: Vec<String>,
    pub fields: Vec<DslFieldSchema>,
    pub stdlib_functions: Vec<String>,
}

pub struct RhaiEngine {
    engine: Engine,
}

impl RhaiEngine {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        engine.set_max_operations(500_000);
        engine.set_max_string_size(1_000_000);
        engine.set_max_array_size(50_000);
        engine.set_max_map_size(50_000);
        engine.set_max_call_levels(32);
        engine.disable_symbol("print");
        engine.disable_symbol("debug");

        types::register_types(&mut engine);
        stdlib::register_stdlib(&mut engine);

        Self { engine }
    }

    pub fn eval_query(&self, graph: Arc<Graph>, script: &str) -> Result<DslQueryResult, DslError> {
        let mut scope = Scope::new();
        scope.push("model", ModelContext::new(graph));

        let result: rhai::Dynamic = self.engine.eval_with_scope(&mut scope, script)?;
        Ok(dynamic_to_query_result(result))
    }

    pub fn schema(graph: &Graph) -> DslSchema {
        let mut kinds = BTreeSet::new();
        let mut field_names = BTreeSet::from([
            "element_id".to_string(),
            "id".to_string(),
            "kind".to_string(),
            "layer".to_string(),
        ]);

        for element in graph.elements() {
            kinds.insert(element.kind.as_ref().to_string());
            field_names.extend(element.properties.to_btree_map().into_keys());
        }

        let registry = KirFieldRegistry::standard();
        let fields = field_names
            .into_iter()
            .map(|name| DslFieldSchema {
                kind: field_kind_label(&registry, &name).to_string(),
                name,
            })
            .collect();

        DslSchema {
            element_kinds: kinds.into_iter().collect(),
            fields,
            stdlib_functions: vec![
                "all_parts".into(),
                "count_by_kind".into(),
                "reachable".into(),
                "specialization_depth".into(),
                "sum".into(),
            ],
        }
    }
}

impl Default for RhaiEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn dynamic_to_query_result(result: rhai::Dynamic) -> DslQueryResult {
    if let Some(query_result) = result.clone().try_cast::<DslQueryResult>() {
        return query_result;
    }
    if let Some(set) = result.clone().try_cast::<ElementSet>() {
        return set.to_query_result();
    }
    if let Some(value) = result.clone().try_cast::<i64>() {
        return single_value_result(Value::from(value));
    }
    if let Some(value) = result.clone().try_cast::<f64>() {
        return single_value_result(float_to_json(value));
    }
    if let Some(value) = result.clone().try_cast::<bool>() {
        return single_value_result(Value::Bool(value));
    }
    if let Some(value) = result.clone().try_cast::<String>() {
        return single_value_result(Value::String(value));
    }
    if let Some(map) = result.clone().try_cast::<rhai::Map>() {
        return map_to_query_result(map);
    }
    if let Some(array) = result.clone().try_cast::<rhai::Array>() {
        return array_to_query_result(array);
    }
    single_value_result(rhai_dynamic_to_json(result))
}

fn single_value_result(value: Value) -> DslQueryResult {
    DslQueryResult {
        columns: vec!["value".into()],
        rows: vec![vec![value]],
    }
}

fn field_kind_label(registry: &KirFieldRegistry, name: &str) -> &'static str {
    match registry.field(name).map(|spec| spec.kind) {
        Some(KirFieldKind::Reference) => "reference",
        Some(KirFieldKind::ReferenceList) => "list",
        _ => "scalar",
    }
}

fn map_to_query_result(map: rhai::Map) -> DslQueryResult {
    let columns = map.keys().map(ToString::to_string).collect::<Vec<_>>();
    let row = columns
        .iter()
        .map(|column| {
            map.get(column.as_str())
                .cloned()
                .map(rhai_dynamic_to_json)
                .unwrap_or(Value::Null)
        })
        .collect();
    DslQueryResult {
        columns,
        rows: vec![row],
    }
}

fn array_to_query_result(array: rhai::Array) -> DslQueryResult {
    if let Some(first) = array.first() {
        if first.is::<rhai::Map>() {
            let first_map = first.clone().cast::<rhai::Map>();
            let columns = first_map
                .keys()
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            let rows = array
                .into_iter()
                .filter_map(|value| value.try_cast::<rhai::Map>())
                .map(|map| {
                    columns
                        .iter()
                        .map(|column| {
                            map.get(column.as_str())
                                .cloned()
                                .map(rhai_dynamic_to_json)
                                .unwrap_or(Value::Null)
                        })
                        .collect()
                })
                .collect();
            return DslQueryResult { columns, rows };
        }
    }

    DslQueryResult {
        columns: vec!["value".into()],
        rows: array
            .into_iter()
            .map(|value| vec![rhai_dynamic_to_json(value)])
            .collect(),
    }
}

fn rhai_dynamic_to_json(value: rhai::Dynamic) -> Value {
    if value.is::<i64>() {
        return Value::from(value.cast::<i64>());
    }
    if value.is::<f64>() {
        return float_to_json(value.cast::<f64>());
    }
    if value.is::<bool>() {
        return Value::Bool(value.cast::<bool>());
    }
    if value.is::<String>() {
        return Value::String(value.cast::<String>());
    }
    if value.is::<rhai::Array>() {
        return Value::Array(
            value
                .cast::<rhai::Array>()
                .into_iter()
                .map(rhai_dynamic_to_json)
                .collect(),
        );
    }
    if value.is::<rhai::Map>() {
        let object = value
            .cast::<rhai::Map>()
            .into_iter()
            .map(|(key, value)| (key.to_string(), rhai_dynamic_to_json(value)))
            .collect();
        return Value::Object(object);
    }
    if value.is_unit() {
        return Value::Null;
    }
    Value::String(format!("{value:?}"))
}

fn float_to_json(value: f64) -> Value {
    Number::from_f64(value)
        .map(Value::Number)
        .unwrap_or(Value::Null)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use mercurio_kir::{KirDocument, KirElement};

    use super::*;

    fn sample_graph() -> Arc<Graph> {
        let document = KirDocument {
            metadata: BTreeMap::new(),
            elements: vec![
                KirElement {
                    id: "type.Demo.Vehicle".into(),
                    kind: "PartDefinition".into(),
                    layer: 2,
                    properties: BTreeMap::from([
                        ("declared_name".into(), serde_json::json!("Vehicle")),
                        ("mass_kg".into(), serde_json::json!(10.0)),
                        (
                            "features".into(),
                            serde_json::json!(["feature.Demo.Vehicle.payloadMass"]),
                        ),
                        (
                            "members".into(),
                            serde_json::json!(["type.Demo.Vehicle.wheel"]),
                        ),
                    ]),
                },
                KirElement {
                    id: "type.Demo.Vehicle.wheel".into(),
                    kind: "PartUsage".into(),
                    layer: 2,
                    properties: BTreeMap::from([
                        ("declared_name".into(), serde_json::json!("wheel")),
                        ("mass_kg".into(), serde_json::json!(2.5)),
                        ("owner".into(), serde_json::json!("type.Demo.Vehicle")),
                    ]),
                },
                KirElement {
                    id: "feature.Demo.Vehicle.payloadMass".into(),
                    kind: "AttributeUsage".into(),
                    layer: 3,
                    properties: BTreeMap::from([
                        ("declared_name".into(), serde_json::json!("payload_mass_kg")),
                        ("owner".into(), serde_json::json!("type.Demo.Vehicle")),
                        (
                            "expression_ir".into(),
                            serde_json::json!({"kind": "literal", "value": 3.0}),
                        ),
                    ]),
                },
                KirElement {
                    id: "type.Demo.Animal".into(),
                    kind: "PartDefinition".into(),
                    layer: 2,
                    properties: BTreeMap::from([
                        ("declared_name".into(), serde_json::json!("Animal")),
                        ("mass_kg".into(), serde_json::json!(4.0)),
                    ]),
                },
            ],
        };
        Arc::new(Graph::from_document(document).unwrap())
    }

    #[test]
    fn count_all_parts() {
        let engine = RhaiEngine::new();
        let result = engine
            .eval_query(sample_graph(), "model.parts().count()")
            .unwrap();
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], serde_json::json!(3));
    }

    #[test]
    fn filter_by_kind() {
        let engine = RhaiEngine::new();
        let result = engine
            .eval_query(
                sample_graph(),
                r#"model.parts().where(|p| p.kind == "PartDefinition").count()"#,
            )
            .unwrap();
        assert_eq!(result.rows[0][0], serde_json::json!(2));
    }

    #[test]
    fn select_names() {
        let engine = RhaiEngine::new();
        let result = engine
            .eval_query(
                sample_graph(),
                r#"model.parts()
                       .where(|p| p.kind == "PartDefinition")
                       .select(["declared_name"])"#,
            )
            .unwrap();
        assert_eq!(result.columns, vec!["declared_name"]);
        assert_eq!(result.rows.len(), 2);
    }

    #[test]
    fn traverse_outgoing() {
        let engine = RhaiEngine::new();
        let result = engine
            .eval_query(
                sample_graph(),
                r#"model.element("type.Demo.Vehicle").outgoing("members").count()"#,
            )
            .unwrap();
        assert_eq!(result.rows[0][0], serde_json::json!(1));
    }

    #[test]
    fn stdlib_count_by_kind() {
        let engine = RhaiEngine::new();
        let result = engine
            .eval_query(sample_graph(), "count_by_kind(model.parts())")
            .unwrap();
        assert_eq!(result.columns, vec!["PartDefinition", "PartUsage"]);
        assert_eq!(result.rows.len(), 1);
    }

    #[test]
    fn stdlib_sum_numeric_properties() {
        let engine = RhaiEngine::new();
        let result = engine
            .eval_query(
                sample_graph(),
                r#"sum(model.parts().map(|p| p.property("mass_kg")))"#,
            )
            .unwrap();
        assert_eq!(result.columns, vec!["value"]);
        assert_eq!(result.rows[0][0], serde_json::json!(16.5));
    }

    #[test]
    fn property_resolves_owned_literal_feature() {
        let engine = RhaiEngine::new();
        let result = engine
            .eval_query(
                sample_graph(),
                r#"model.element("type.Demo.Vehicle").property("payload_mass_kg")"#,
            )
            .unwrap();
        assert_eq!(result.columns, vec!["value"]);
        assert_eq!(result.rows[0][0], serde_json::json!(3.0));
    }

    #[test]
    fn unknown_element_returns_unit() {
        let engine = RhaiEngine::new();
        let result = engine.eval_query(sample_graph(), r#"model.element("nonexistent")"#);
        assert!(result.is_ok());
    }
}
