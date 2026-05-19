use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::datalog::materialize_core_indexes;
use crate::derived::derived_properties;
use crate::graph::{Edge, Element, Graph};
use crate::metamodel::{
    AttributeRow, AttributeValueSource, ElementSummary, MetamodelAttributeRegistry,
    collect_specialization_ancestors, effective_properties_with_derived, query_element_attributes,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphScope {
    L2,
    L2PlusContext,
    Full,
}

impl GraphScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::L2 => "l2",
            Self::L2PlusContext => "l2_plus_context",
            Self::Full => "full",
        }
    }

    pub fn from_query(value: Option<&str>) -> Self {
        match value {
            Some("full") => Self::Full,
            Some("l2_plus_context") => Self::L2PlusContext,
            Some("l2") | None | Some(_) => Self::L2,
        }
    }

    pub fn all() -> Vec<String> {
        [Self::L2, Self::L2PlusContext, Self::Full]
            .into_iter()
            .map(|scope| scope.as_str().to_string())
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GraphDto {
    pub nodes: Vec<GraphNodeDto>,
    pub edges: Vec<GraphEdgeDto>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GraphNodeDto {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub layer: u8,
    pub property_count: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GraphEdgeDto {
    pub id: String,
    pub source: String,
    pub target: String,
    pub relation: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ElementDetailsDto {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub layer: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metatype: Option<ElementSummaryDto>,
    pub metatype_specialization_chain: Vec<ElementSummaryDto>,
    pub direct_properties: BTreeMap<String, Value>,
    pub inherited_properties: Vec<InheritedPropertiesDto>,
    pub effective_properties: BTreeMap<String, Value>,
    pub property_table: ElementPropertyTableDto,
    pub specialization_chain: Vec<ElementSummaryDto>,
    pub inbound: Vec<GraphEdgeDto>,
    pub outbound: Vec<GraphEdgeDto>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ElementSummaryDto {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub layer: u8,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct InheritedPropertiesDto {
    pub element: ElementSummaryDto,
    pub properties: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ElementPropertyTableDto {
    pub rows: Vec<ElementPropertyRowDto>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ElementPropertyRowDto {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_by: Option<ElementSummaryDto>,
    pub origin_kind: String,
    pub has_direct_value: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_value: Option<Value>,
    pub has_effective_value: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_value: Option<Value>,
    pub inherited_values: Vec<InheritedPropertyValueDto>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct InheritedPropertyValueDto {
    pub element: ElementSummaryDto,
    pub value: Value,
}

pub fn graph_view(graph: &Graph, scope: GraphScope) -> GraphDto {
    let visible_ids = collect_graph_scope_ids(graph, scope);
    let mut nodes = graph
        .elements()
        .iter()
        .filter(|element| visible_ids.contains(&element.id))
        .map(|element| GraphNodeDto {
            id: element.element_id.clone(),
            label: label_for_id(&element.element_id),
            kind: element.kind.clone(),
            layer: element.layer,
            property_count: element.properties.len(),
        })
        .collect::<Vec<_>>();
    nodes.sort_by(|left, right| left.id.cmp(&right.id));

    let mut edges = graph
        .edges()
        .iter()
        .filter(|edge| visible_ids.contains(&edge.source) && visible_ids.contains(&edge.target))
        .filter_map(|edge| edge_view(graph, edge))
        .collect::<Vec<_>>();
    edges.sort_by(|left, right| left.id.cmp(&right.id));

    GraphDto { nodes, edges }
}

pub fn element_details(
    graph: &Graph,
    metamodel_registry: &MetamodelAttributeRegistry,
    element_id: &str,
) -> Option<ElementDetailsDto> {
    let element = graph.element_by_element_id(element_id)?;

    let mut inbound = graph
        .incoming_edges(element.id)
        .filter_map(|edge| edge_view(graph, edge))
        .collect::<Vec<_>>();
    inbound.sort_by(|left, right| left.id.cmp(&right.id));

    let mut outbound = graph
        .outgoing_edges(element.id)
        .filter_map(|edge| edge_view(graph, edge))
        .collect::<Vec<_>>();
    outbound.sort_by(|left, right| left.id.cmp(&right.id));

    Some(build_element_details(
        graph,
        metamodel_registry,
        element,
        inbound,
        outbound,
    ))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequirementTableViewDto {
    pub title: String,
    pub columns: Vec<RequirementTableColumnDto>,
    pub rows: Vec<RequirementTableRowDto>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequirementTableColumnDto {
    pub key: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequirementTableRowDto {
    pub id: String,
    pub name: Option<String>,
    pub text: Option<String>,
    pub owner: Option<String>,
    pub satisfied_by: Vec<String>,
    pub verified_by: Vec<String>,
    pub source: Option<RequirementSourceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequirementSourceDto {
    pub file: Option<String>,
    pub start_line: Option<u64>,
    pub end_line: Option<u64>,
}

pub fn requirements_table_view(graph: &Graph) -> RequirementTableViewDto {
    let derived = materialize_core_indexes(graph, &[]).ok();
    let mut rows = graph
        .elements()
        .iter()
        .filter(|element| !is_library_requirement(element))
        .filter(|element| {
            derived
                .as_ref()
                .is_some_and(|derived| derived.requirements.contains(&element.element_id))
                || is_requirement(element)
        })
        .map(|requirement| RequirementTableRowDto {
            id: requirement.element_id.clone(),
            name: string_property(requirement, "declared_name")
                .or_else(|| string_property(requirement, "name")),
            text: string_property(requirement, "text")
                .or_else(|| string_property(requirement, "documentation")),
            owner: string_property(requirement, "owner"),
            satisfied_by: derived_sources(&derived, &requirement.element_id, "satisfies")
                .unwrap_or_else(|| related_sources(graph, requirement, &["satisfy", "satisfies"])),
            verified_by: derived_sources(&derived, &requirement.element_id, "verifies")
                .unwrap_or_else(|| related_sources(graph, requirement, &["verify", "verifies"])),
            source: source_for(requirement),
        })
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| left.id.cmp(&right.id));

    let mut warnings = Vec::new();
    if rows.is_empty() {
        warnings.push("No requirement elements were found in the semantic graph.".to_string());
    }

    RequirementTableViewDto {
        title: "Requirements".to_string(),
        columns: vec![
            column("id", "ID"),
            column("name", "Name"),
            column("text", "Text"),
            column("owner", "Owner"),
            column("satisfied_by", "Satisfied By"),
            column("verified_by", "Verified By"),
            column("source", "Source"),
        ],
        rows,
        warnings,
    }
}

fn build_element_details(
    graph: &Graph,
    metamodel_registry: &MetamodelAttributeRegistry,
    element: &Element,
    inbound: Vec<GraphEdgeDto>,
    outbound: Vec<GraphEdgeDto>,
) -> ElementDetailsDto {
    let ancestors = collect_specialization_ancestors(graph, element.id);
    let specialization_chain = ancestors
        .iter()
        .map(|ancestor| element_summary_dto(ancestor))
        .collect::<Vec<_>>();
    let inherited_properties = ancestors
        .iter()
        .filter(|ancestor| !ancestor.properties.is_empty())
        .map(|ancestor| InheritedPropertiesDto {
            element: element_summary_dto(ancestor),
            properties: ancestor.properties.clone(),
        })
        .collect::<Vec<_>>();

    let derived_properties = derived_properties(graph, element);
    let effective_properties =
        effective_properties_with_derived(&ancestors, &element.properties, &derived_properties);
    let attribute_query = query_element_attributes(graph, metamodel_registry, element.id, None)
        .unwrap_or_else(|| crate::metamodel::ElementAttributeQuery {
            metatype: None,
            metatype_specialization_chain: Vec::new(),
            rows: Vec::new(),
        });

    ElementDetailsDto {
        id: element.element_id.clone(),
        label: label_for_id(&element.element_id),
        kind: element.kind.clone(),
        layer: element.layer,
        metatype: attribute_query.metatype.map(element_summary_from_query),
        metatype_specialization_chain: attribute_query
            .metatype_specialization_chain
            .into_iter()
            .map(element_summary_from_query)
            .collect(),
        direct_properties: element.properties.clone(),
        inherited_properties,
        effective_properties,
        property_table: ElementPropertyTableDto {
            rows: attribute_query
                .rows
                .into_iter()
                .map(property_row_from_query)
                .collect(),
        },
        specialization_chain,
        inbound,
        outbound,
    }
}

fn element_summary_from_query(summary: ElementSummary) -> ElementSummaryDto {
    ElementSummaryDto {
        id: summary.id,
        label: summary.label,
        kind: summary.kind,
        layer: summary.layer,
    }
}

fn inherited_value_from_query(value: AttributeValueSource) -> InheritedPropertyValueDto {
    InheritedPropertyValueDto {
        element: element_summary_from_query(value.element),
        value: value.value,
    }
}

fn property_row_from_query(row: AttributeRow) -> ElementPropertyRowDto {
    ElementPropertyRowDto {
        name: row.name,
        declared_by: row.declared_by.map(element_summary_from_query),
        origin_kind: row.origin_kind,
        has_direct_value: row.has_direct_value,
        direct_value: row.direct_value,
        has_effective_value: row.has_effective_value,
        effective_value: row.effective_value,
        inherited_values: row
            .inherited_values
            .into_iter()
            .map(inherited_value_from_query)
            .collect(),
    }
}

fn element_summary_dto(element: &Element) -> ElementSummaryDto {
    ElementSummaryDto {
        id: element.element_id.clone(),
        label: label_for_id(&element.element_id),
        kind: element.kind.clone(),
        layer: element.layer,
    }
}

fn edge_view(graph: &Graph, edge: &Edge) -> Option<GraphEdgeDto> {
    let source = graph.element_id(edge.source)?.to_string();
    let target = graph.element_id(edge.target)?.to_string();

    Some(GraphEdgeDto {
        id: format!("{source}:{}:{target}", edge.relation),
        source,
        target,
        relation: edge.relation.clone(),
    })
}

fn label_for_id(id: &str) -> String {
    let tail = id.rsplit("::").next().unwrap_or(id);
    tail.rsplit('.').next().unwrap_or(tail).to_string()
}

fn collect_graph_scope_ids(graph: &Graph, scope: GraphScope) -> BTreeSet<u32> {
    let mut visible_ids = graph
        .elements()
        .iter()
        .filter(|element| match scope {
            GraphScope::L2 | GraphScope::L2PlusContext => element.layer == 2,
            GraphScope::Full => true,
        })
        .map(|element| element.id)
        .collect::<BTreeSet<_>>();

    if scope == GraphScope::L2PlusContext {
        let l2_ids = visible_ids.iter().copied().collect::<Vec<_>>();
        for node_id in l2_ids {
            for edge in graph.outgoing_edges(node_id) {
                visible_ids.insert(edge.target);
            }
            for edge in graph.incoming_edges(node_id) {
                visible_ids.insert(edge.source);
            }
        }
    }

    visible_ids
}

fn derived_sources(
    derived: &Option<crate::datalog::DerivedIndexes>,
    requirement_id: &str,
    relation: &str,
) -> Option<Vec<String>> {
    let derived = derived.as_ref()?;
    let sources = match relation {
        "satisfies" => derived.satisfied_by.get(requirement_id),
        "verifies" => derived.verified_by.get(requirement_id),
        _ => None,
    }?;
    Some(sources.iter().cloned().collect())
}

fn column(key: &str, label: &str) -> RequirementTableColumnDto {
    RequirementTableColumnDto {
        key: key.to_string(),
        label: label.to_string(),
    }
}

fn is_requirement(element: &Element) -> bool {
    if is_requirement_relationship(element) {
        return false;
    }

    element.layer == 2
        && (element.kind.contains("Requirement")
            || element
                .properties
                .get("specializes")
                .and_then(Value::as_array)
                .is_some_and(|specializations| {
                    specializations
                        .iter()
                        .filter_map(Value::as_str)
                        .any(|target| target.contains("Requirement"))
                }))
}

fn is_library_requirement(element: &Element) -> bool {
    element.element_id.contains("::")
}

fn is_requirement_relationship(element: &Element) -> bool {
    let kind = element.kind.to_ascii_lowercase();
    ["satisfy", "verify", "derive", "refine"]
        .iter()
        .any(|relationship| kind.contains(relationship))
}

fn related_sources(graph: &Graph, requirement: &Element, relations: &[&str]) -> Vec<String> {
    let mut sources = Vec::new();

    for relation in relations {
        for edge in graph.incoming(requirement.id, relation) {
            if let Some(source) = graph.element_id(edge.source) {
                push_unique(&mut sources, source.to_string());
            }
        }
    }

    for element in graph.elements() {
        if !is_relationship_element(element, relations) {
            continue;
        }
        let Some(target) = string_property(element, "target") else {
            continue;
        };
        if target != requirement.element_id {
            continue;
        }
        if let Some(source) = string_property(element, "source") {
            push_unique(&mut sources, source);
        }
    }

    sources.sort();
    sources
}

fn is_relationship_element(element: &Element, relations: &[&str]) -> bool {
    let kind = element.kind.to_ascii_lowercase();
    relations
        .iter()
        .any(|relation| kind.contains(&relation.to_ascii_lowercase()))
}

fn source_for(element: &Element) -> Option<RequirementSourceDto> {
    let metadata = element.properties.get("metadata")?;
    let file = metadata
        .get("source_file")
        .and_then(Value::as_str)
        .map(str::to_string);
    let span = metadata.get("source_span");
    let start_line = span
        .and_then(|span| span.get("start_line"))
        .and_then(Value::as_u64);
    let end_line = span
        .and_then(|span| span.get("end_line"))
        .and_then(Value::as_u64);

    Some(RequirementSourceDto {
        file,
        start_line,
        end_line,
    })
}

fn string_property(element: &Element, key: &str) -> Option<String> {
    element
        .properties
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.iter().any(|existing| existing == &value) {
        values.push(value);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::Value;

    use crate::{Graph, KirDocument, KirElement, MetamodelAttributeRegistry, Runtime, repo_path};

    use super::{GraphScope, element_details, graph_view, requirements_table_view};

    #[test]
    fn extracts_requirement_rows_from_example_model() {
        let document =
            KirDocument::from_path(&repo_path("examples/requirements_table_model.json")).unwrap();
        let runtime = Runtime::from_document(document).unwrap();

        let view = requirements_table_view(runtime.graph());

        assert_eq!(view.rows.len(), 3);
        assert_eq!(view.rows[0].id, "req.VehicleSafety.BrakingDistance");
        assert_eq!(
            view.rows[0].satisfied_by,
            vec!["part.VehicleSafety.BrakeController"]
        );
        assert_eq!(
            view.rows[0].verified_by,
            vec!["case.VehicleSafety.BrakingDistanceTest"]
        );
    }

    #[test]
    fn reports_empty_requirement_view() {
        let graph = Graph::from_document(KirDocument {
            metadata: Default::default(),
            elements: vec![],
        })
        .unwrap();

        let view = requirements_table_view(&graph);

        assert!(view.rows.is_empty());
        assert_eq!(
            view.warnings,
            vec!["No requirement elements were found in the semantic graph."]
        );
    }

    #[test]
    fn graph_view_l2_plus_context_includes_connected_library_nodes() {
        let graph = Graph::from_document(KirDocument {
            metadata: BTreeMap::new(),
            elements: vec![
                KirElement {
                    id: "type.Vehicle".to_string(),
                    kind: "SysML::Systems::PartDefinition".to_string(),
                    layer: 2,
                    properties: BTreeMap::from([(
                        "metatype".to_string(),
                        Value::String("SysML::Systems::PartDefinition".to_string()),
                    )]),
                },
                KirElement {
                    id: "SysML::Systems::PartDefinition".to_string(),
                    kind: "Metaclass".to_string(),
                    layer: 1,
                    properties: BTreeMap::new(),
                },
            ],
        })
        .unwrap();

        let l2 = graph_view(&graph, GraphScope::L2);
        let l2_plus_context = graph_view(&graph, GraphScope::L2PlusContext);

        assert_eq!(l2.nodes.len(), 1);
        assert_eq!(l2_plus_context.nodes.len(), 2);
        assert_eq!(l2_plus_context.edges.len(), 1);
        assert_eq!(l2_plus_context.edges[0].relation, "metatype");
    }

    #[test]
    fn element_details_include_effective_properties_and_edges() {
        let graph = Graph::from_document(KirDocument {
            metadata: BTreeMap::new(),
            elements: vec![
                KirElement {
                    id: "type.BaseVehicle".to_string(),
                    kind: "SysML::Systems::PartDefinition".to_string(),
                    layer: 2,
                    properties: BTreeMap::from([(
                        "mass".to_string(),
                        Value::String("1000 kg".to_string()),
                    )]),
                },
                KirElement {
                    id: "type.Vehicle".to_string(),
                    kind: "SysML::Systems::PartDefinition".to_string(),
                    layer: 2,
                    properties: BTreeMap::from([(
                        "specializes".to_string(),
                        Value::Array(vec![Value::String("type.BaseVehicle".to_string())]),
                    )]),
                },
            ],
        })
        .unwrap();
        let registry = MetamodelAttributeRegistry::build(&graph);

        let details = element_details(&graph, &registry, "type.Vehicle").unwrap();

        assert_eq!(details.id, "type.Vehicle");
        assert_eq!(details.label, "Vehicle");
        assert_eq!(details.specialization_chain[0].id, "type.BaseVehicle");
        assert_eq!(
            details.effective_properties.get("mass"),
            Some(&Value::String("1000 kg".to_string()))
        );
        assert_eq!(details.outbound.len(), 1);
        assert_eq!(details.outbound[0].relation, "specializes");
    }
}
