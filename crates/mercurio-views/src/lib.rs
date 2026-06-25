//! View specification DTOs and render entrypoints.
//!
//! This crate exposes serializable diagram/table specs, validation diagnostics,
//! and render functions for model-backed views.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use mercurio_core::{
    Element, Graph, MetamodelAttributeRegistry, NodeId, collect_specialization_ancestors,
    effective_properties, element_metatype, metadata_annotations_named, query_element_attributes,
};

const DEFAULT_MAX_DEPTH: usize = 8;
const DEFAULT_MAX_NODES: usize = 350;
const DEFAULT_MAX_EDGES: usize = 900;
const MAX_RELATION_FANOUT_PER_NODE: usize = 250;
const TIMING_WARNING_THRESHOLD_MS: u128 = 250;
pub const VIEW_SCHEMA: &str = "mercurio.view.v1";
pub const VIEW_SPEC_VERSION: u8 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiagramKindDto {
    Structure,
    Activity,
    StateMachine,
    PackageTree,
    CompositionGraph,
    ReferenceGraph,
    DependencyGraph,
    MetatypeInstanceMap,
    ImpactView,
    PropertyInheritance,
    ValidationView,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiagramDirectionDto {
    Parents,
    Children,
    Both,
}

impl Default for DiagramDirectionDto {
    fn default() -> Self {
        Self::Children
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagramQueryOptionsDto {
    #[serde(default = "default_diagram_relations")]
    pub relations: Vec<String>,
    #[serde(default)]
    pub direction: DiagramDirectionDto,
    #[serde(default = "default_diagram_depth")]
    pub depth: usize,
    #[serde(default = "default_true")]
    pub include_libraries: bool,
    #[serde(default = "default_true")]
    pub include_user_model: bool,
    #[serde(default = "default_max_nodes")]
    pub max_nodes: usize,
    #[serde(default = "default_max_edges")]
    pub max_edges: usize,
}

impl Default for DiagramQueryOptionsDto {
    fn default() -> Self {
        Self {
            relations: default_diagram_relations(),
            direction: DiagramDirectionDto::default(),
            depth: default_diagram_depth(),
            include_libraries: true,
            include_user_model: true,
            max_nodes: default_max_nodes(),
            max_edges: default_max_edges(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagramLayoutOptionsDto {
    #[serde(default = "default_layout_engine")]
    pub engine: String,
    #[serde(default = "default_layout_direction")]
    pub direction: String,
}

impl Default for DiagramLayoutOptionsDto {
    fn default() -> Self {
        Self {
            engine: default_layout_engine(),
            direction: default_layout_direction(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagramStyleOptionsDto {
    #[serde(default = "default_true")]
    pub show_attributes: bool,
    #[serde(default = "default_true")]
    pub show_edge_labels: bool,
    #[serde(default)]
    pub group_by_layer: bool,
}

impl Default for DiagramStyleOptionsDto {
    fn default() -> Self {
        Self {
            show_attributes: true,
            show_edge_labels: true,
            group_by_layer: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagramSpecDto {
    pub version: u8,
    pub kind: DiagramKindDto,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
    #[serde(default)]
    pub query: DiagramQueryOptionsDto,
    #[serde(default)]
    pub layout: DiagramLayoutOptionsDto,
    #[serde(default)]
    pub style: DiagramStyleOptionsDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagramRenderRequestDto {
    pub spec: DiagramSpecDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ViewModeDto {
    Visualization,
    Creation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ViewDocumentDto {
    pub schema: String,
    pub version: u8,
    pub kind: String,
    pub mode: ViewModeDto,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagram: Option<DiagramSpecDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<TableSpecDto>,
}

impl ViewDocumentDto {
    pub fn diagram(spec: DiagramSpecDto) -> Self {
        Self {
            schema: VIEW_SCHEMA.to_string(),
            version: VIEW_SPEC_VERSION,
            kind: format!("diagram.{}", diagram_kind_name(&spec.kind)),
            mode: ViewModeDto::Visualization,
            parameters: BTreeMap::new(),
            diagram: Some(spec),
            table: None,
        }
    }

    pub fn table(spec: TableSpecDto) -> Self {
        Self {
            schema: VIEW_SCHEMA.to_string(),
            version: VIEW_SPEC_VERSION,
            kind: "table".to_string(),
            mode: ViewModeDto::Visualization,
            parameters: BTreeMap::new(),
            diagram: None,
            table: Some(spec),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TableKindDto {
    Elements,
    Requirements,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableColumnSpecDto {
    pub key: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableSpecDto {
    pub version: u8,
    pub kind: TableKindDto,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(default)]
    pub query: DiagramQueryOptionsDto,
    #[serde(default)]
    pub columns: Vec<TableColumnSpecDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableRenderRequestDto {
    pub spec: TableSpecDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableViewDto {
    pub spec: TableSpecDto,
    pub columns: Vec<TableColumnSpecDto>,
    #[serde(default)]
    pub available_columns: Vec<TableColumnSpecDto>,
    pub rows: Vec<TableRowDto>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableRowDto {
    pub id: String,
    pub element: String,
    pub cells: Vec<TableCellDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableCellDto {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ViewValidationDiagnostic {
    pub code: &'static str,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagramViewDto {
    pub spec: DiagramSpecDto,
    pub symbols: Vec<DiagramSymbolDto>,
    pub nodes: Vec<DiagramNodeDto>,
    pub edges: Vec<DiagramEdgeDto>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagramSymbolDto {
    pub id: String,
    pub element: String,
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    #[serde(default)]
    pub properties: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagramNodeDto {
    pub id: String,
    pub symbol: String,
    pub label: String,
    pub kind: String,
    pub layer: u8,
    pub badges: Vec<String>,
    pub attributes: Vec<DiagramAttributeDto>,
    pub properties: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagramAttributeDto {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagramEdgeDto {
    pub id: String,
    pub symbol: String,
    pub source: String,
    pub target: String,
    pub relation: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagramError {
    UnsupportedKind(DiagramKindDto),
    UnsupportedVersion(u8),
    MissingRoot,
    RootNotFound(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableError {
    UnsupportedKind(TableKindDto),
    UnsupportedVersion(u8),
    RootNotFound(String),
}

impl std::fmt::Display for TableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedKind(kind) => write!(f, "table kind is not implemented: {kind:?}"),
            Self::UnsupportedVersion(version) => {
                write!(f, "unsupported table spec version: {version}")
            }
            Self::RootNotFound(root) => write!(f, "table root not found: {root}"),
        }
    }
}

impl std::error::Error for TableError {}

impl std::fmt::Display for DiagramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedKind(kind) => write!(f, "diagram kind is not implemented: {kind:?}"),
            Self::UnsupportedVersion(version) => {
                write!(f, "unsupported diagram spec version: {version}")
            }
            Self::MissingRoot => write!(f, "diagram root is required"),
            Self::RootNotFound(root) => write!(f, "diagram root not found: {root}"),
        }
    }
}

impl std::error::Error for DiagramError {}

pub fn list_diagram_kinds() -> Vec<DiagramKindDto> {
    vec![
        DiagramKindDto::Structure,
        DiagramKindDto::Activity,
        DiagramKindDto::StateMachine,
        DiagramKindDto::PackageTree,
        DiagramKindDto::CompositionGraph,
        DiagramKindDto::ReferenceGraph,
        DiagramKindDto::DependencyGraph,
        DiagramKindDto::MetatypeInstanceMap,
        DiagramKindDto::ImpactView,
        DiagramKindDto::PropertyInheritance,
        DiagramKindDto::ValidationView,
    ]
}

pub fn list_table_kinds() -> Vec<TableKindDto> {
    vec![TableKindDto::Elements, TableKindDto::Requirements]
}

pub fn validate_view_document(
    document: &ViewDocumentDto,
) -> Result<(), Vec<ViewValidationDiagnostic>> {
    let mut diagnostics = Vec::new();
    if document.schema != VIEW_SCHEMA {
        diagnostics.push(view_diagnostic(
            "view.schema",
            "/schema",
            format!("expected schema `{VIEW_SCHEMA}`"),
        ));
    }
    if document.version != VIEW_SPEC_VERSION {
        diagnostics.push(view_diagnostic(
            "view.version",
            "/version",
            format!("expected version {VIEW_SPEC_VERSION}"),
        ));
    }
    match (&document.diagram, &document.table) {
        (Some(diagram), None) => {
            let expected = format!("diagram.{}", diagram_kind_name(&diagram.kind));
            if document.kind != expected {
                diagnostics.push(view_diagnostic(
                    "view.kind",
                    "/kind",
                    format!("expected kind `{expected}` for diagram payload"),
                ));
            }
            validate_diagram_spec(diagram, "/diagram", &mut diagnostics);
        }
        (None, Some(table)) => {
            let expected = "table";
            if document.kind != expected {
                diagnostics.push(view_diagnostic(
                    "view.kind",
                    "/kind",
                    format!("expected kind `{expected}` for table payload"),
                ));
            }
            validate_table_spec(table, "/table", &mut diagnostics);
        }
        (None, None) => {
            if document.kind.trim().is_empty() {
                diagnostics.push(view_diagnostic(
                    "view.kind",
                    "/kind",
                    "view kind is required when no inline payload is provided".to_string(),
                ));
            }
        }
        (Some(_), Some(_)) => diagnostics.push(view_diagnostic(
            "view.payload",
            "/",
            "view document must not contain multiple payloads".to_string(),
        )),
    }

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn validate_diagram_spec(
    spec: &DiagramSpecDto,
    path: &str,
    diagnostics: &mut Vec<ViewValidationDiagnostic>,
) {
    validate_common_spec(spec.version, &spec.title, &spec.query, path, diagnostics);
    if spec.layout.engine.trim().is_empty() {
        diagnostics.push(view_diagnostic(
            "view.diagram.layout.engine",
            format!("{path}/layout/engine"),
            "layout engine is required".to_string(),
        ));
    }
    let direction = spec.layout.direction.to_ascii_uppercase();
    if !matches!(direction.as_str(), "LR" | "RL" | "TB" | "BT") {
        diagnostics.push(view_diagnostic(
            "view.diagram.layout.direction",
            format!("{path}/layout/direction"),
            "layout direction must be one of LR, RL, TB, BT".to_string(),
        ));
    }
}

fn validate_table_spec(
    spec: &TableSpecDto,
    path: &str,
    diagnostics: &mut Vec<ViewValidationDiagnostic>,
) {
    validate_common_spec(spec.version, &spec.title, &spec.query, path, diagnostics);
    let mut column_keys = BTreeSet::new();
    for (index, column) in spec.columns.iter().enumerate() {
        if column.key.trim().is_empty() {
            diagnostics.push(view_diagnostic(
                "view.table.column.key",
                format!("{path}/columns/{index}/key"),
                "column key is required".to_string(),
            ));
        }
        if !column_keys.insert(column.key.clone()) {
            diagnostics.push(view_diagnostic(
                "view.table.column.key.duplicate",
                format!("{path}/columns/{index}/key"),
                format!("duplicate column key `{}`", column.key),
            ));
        }
        if column.label.trim().is_empty() {
            diagnostics.push(view_diagnostic(
                "view.table.column.label",
                format!("{path}/columns/{index}/label"),
                "column label is required".to_string(),
            ));
        }
        if let Some(path_value) = &column.path {
            validate_column_path(
                path_value,
                format!("{path}/columns/{index}/path"),
                diagnostics,
            );
        } else {
            validate_column_path(
                &column.key,
                format!("{path}/columns/{index}/key"),
                diagnostics,
            );
        }
    }
}

fn validate_column_path(
    path_value: &str,
    path: String,
    diagnostics: &mut Vec<ViewValidationDiagnostic>,
) {
    if path_value
        .split('.')
        .any(|segment| segment.trim().is_empty())
    {
        diagnostics.push(view_diagnostic(
            "view.table.column.path",
            path,
            "column path segments must not be empty".to_string(),
        ));
    }
}

fn validate_common_spec(
    version: u8,
    title: &str,
    query: &DiagramQueryOptionsDto,
    path: &str,
    diagnostics: &mut Vec<ViewValidationDiagnostic>,
) {
    if version != VIEW_SPEC_VERSION {
        diagnostics.push(view_diagnostic(
            "view.spec.version",
            format!("{path}/version"),
            format!("expected spec version {VIEW_SPEC_VERSION}"),
        ));
    }
    if title.trim().is_empty() {
        diagnostics.push(view_diagnostic(
            "view.spec.title",
            format!("{path}/title"),
            "title is required".to_string(),
        ));
    }
    if query.max_nodes == 0 {
        diagnostics.push(view_diagnostic(
            "view.query.max_nodes",
            format!("{path}/query/max_nodes"),
            "max_nodes must be greater than zero".to_string(),
        ));
    }
    if query.max_edges == 0 {
        diagnostics.push(view_diagnostic(
            "view.query.max_edges",
            format!("{path}/query/max_edges"),
            "max_edges must be greater than zero".to_string(),
        ));
    }
}

fn view_diagnostic(
    code: &'static str,
    path: impl Into<String>,
    message: String,
) -> ViewValidationDiagnostic {
    ViewValidationDiagnostic {
        code,
        path: path.into(),
        message,
    }
}

pub fn render_table(
    graph: &Graph,
    metamodel_registry: &MetamodelAttributeRegistry,
    mut spec: TableSpecDto,
) -> Result<TableViewDto, TableError> {
    if spec.version != 1 {
        return Err(TableError::UnsupportedVersion(spec.version));
    }

    match spec.kind {
        TableKindDto::Elements => render_elements_table(graph, metamodel_registry, &mut spec),
        TableKindDto::Requirements => {
            render_requirements_table(graph, metamodel_registry, &mut spec)
        }
    }
}

fn render_elements_table(
    graph: &Graph,
    metamodel_registry: &MetamodelAttributeRegistry,
    spec: &mut TableSpecDto,
) -> Result<TableViewDto, TableError> {
    let target_type = normalized_target_type(spec.target_type.as_deref());
    let available_columns =
        available_table_columns(graph, metamodel_registry, target_type.as_deref());
    let columns = if spec.columns.is_empty() {
        default_table_columns(&available_columns)
    } else {
        spec.columns.clone()
    };
    let visible_ids = if let Some(root) =
        spec.root.as_deref().filter(|root| !root.trim().is_empty())
    {
        let root =
            resolve_root(graph, root).ok_or_else(|| TableError::RootNotFound(root.to_string()))?;
        collect_structure_ids(graph, root.id, &spec.query, &default_diagram_relations()).visible_ids
    } else {
        graph.elements().iter().map(|element| element.id).collect()
    };

    let mut rows = visible_ids
        .iter()
        .filter_map(|node_id| graph.element(*node_id))
        .filter(|element| include_element(element, &spec.query))
        .filter(|element| table_target_matches(graph, element, target_type.as_deref()))
        .map(|element| table_row(graph, element, &columns))
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.id.cmp(&right.id));

    let mut warnings = Vec::new();
    if rows.is_empty() {
        warnings.push("No elements matched the requested filters.".to_string());
    }

    Ok(TableViewDto {
        spec: spec.clone(),
        columns,
        available_columns,
        rows,
        warnings,
    })
}

fn render_requirements_table(
    graph: &Graph,
    metamodel_registry: &MetamodelAttributeRegistry,
    spec: &mut TableSpecDto,
) -> Result<TableViewDto, TableError> {
    let target_type = normalized_target_type(spec.target_type.as_deref())
        .or_else(|| Some("Requirement".to_string()));
    let available_columns =
        available_table_columns(graph, metamodel_registry, target_type.as_deref());
    let columns = if spec.columns.is_empty() {
        default_requirements_columns()
    } else {
        spec.columns.clone()
    };
    let visible_ids =
        if let Some(root) = spec.root.as_deref().filter(|root| !root.trim().is_empty()) {
            let root = resolve_root(graph, root)
                .ok_or_else(|| TableError::RootNotFound(root.to_string()))?;
            collect_structure_ids(
                graph,
                root.id,
                &spec.query,
                &[
                    "owner".to_string(),
                    "satisfy".to_string(),
                    "verify".to_string(),
                ],
            )
            .visible_ids
        } else {
            graph.elements().iter().map(|element| element.id).collect()
        };

    let mut rows = visible_ids
        .iter()
        .filter_map(|node_id| graph.element(*node_id))
        .filter(|element| include_element(element, &spec.query))
        .filter(|element| table_target_matches(graph, element, target_type.as_deref()))
        .map(|element| table_row(graph, element, &columns))
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.id.cmp(&right.id));

    let mut warnings = Vec::new();
    if rows.is_empty() {
        warnings.push("No requirements matched the requested filters.".to_string());
    }

    Ok(TableViewDto {
        spec: spec.clone(),
        columns,
        available_columns,
        rows,
        warnings,
    })
}

fn default_elements_columns() -> Vec<TableColumnSpecDto> {
    [
        ("id", "ID"),
        ("name", "Name"),
        ("kind", "Kind"),
        ("owner", "Owner"),
        ("source_file", "Source"),
    ]
    .into_iter()
    .map(|(key, label)| TableColumnSpecDto {
        key: key.to_string(),
        label: label.to_string(),
        path: None,
    })
    .collect()
}

fn default_requirements_columns() -> Vec<TableColumnSpecDto> {
    [
        ("requirement_id", "ID"),
        ("name", "Name"),
        ("text", "Text"),
        ("status", "Status"),
        ("owner", "Owner"),
    ]
    .into_iter()
    .map(|(key, label)| TableColumnSpecDto {
        key: key.to_string(),
        label: label.to_string(),
        path: None,
    })
    .collect()
}

fn normalized_target_type(target_type: Option<&str>) -> Option<String> {
    target_type
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn default_table_columns(available_columns: &[TableColumnSpecDto]) -> Vec<TableColumnSpecDto> {
    let preferred = ["id", "name", "kind", "owner", "source_file"];
    let mut columns = preferred
        .iter()
        .filter_map(|key| {
            available_columns
                .iter()
                .find(|column| column.key == *key)
                .cloned()
        })
        .collect::<Vec<_>>();
    if columns.is_empty() {
        columns = default_elements_columns();
    }
    columns
}

fn available_table_columns(
    graph: &Graph,
    metamodel_registry: &MetamodelAttributeRegistry,
    target_type: Option<&str>,
) -> Vec<TableColumnSpecDto> {
    let mut keys = BTreeSet::new();
    let mut columns = Vec::new();
    for column in default_elements_columns() {
        if keys.insert(column.key.clone()) {
            columns.push(column);
        }
    }

    if let Some(target_type) = target_type {
        for element in graph.elements() {
            if !table_type_identifier_matches(element, target_type) {
                continue;
            }
            for declaration in metamodel_registry.declared_attributes_for(&element.element_id) {
                if keys.insert(declaration.name.clone()) {
                    columns.push(TableColumnSpecDto {
                        key: declaration.name.clone(),
                        label: title_from_column_key(&declaration.name),
                        path: Some(declaration.name.clone()),
                    });
                }
            }
            let Some(query) = query_element_attributes(graph, metamodel_registry, element.id, None)
            else {
                continue;
            };
            for row in query.rows {
                if keys.insert(row.name.clone()) {
                    columns.push(TableColumnSpecDto {
                        key: row.name.clone(),
                        label: title_from_column_key(&row.name),
                        path: Some(row.name),
                    });
                }
            }
        }
    }

    columns.sort_by(|left, right| {
        column_sort_rank(&left.key)
            .cmp(&column_sort_rank(&right.key))
            .then_with(|| left.label.cmp(&right.label))
    });
    columns
}

fn column_sort_rank(key: &str) -> usize {
    match key {
        "id" => 0,
        "name" => 1,
        "kind" => 2,
        "owner" => 3,
        "source_file" => 4,
        _ => 10,
    }
}

fn title_from_column_key(key: &str) -> String {
    key.split('_')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn table_target_matches(graph: &Graph, element: &Element, target_type: Option<&str>) -> bool {
    let Some(target_type) = target_type else {
        return true;
    };
    if table_type_is_element(target_type) {
        return true;
    }
    table_type_identifier_matches(element, target_type)
        || element_metatype(graph, element.id)
            .is_some_and(|metatype| table_type_identifier_matches(metatype, target_type))
        || collect_specialization_ancestors(graph, element.id)
            .into_iter()
            .any(|ancestor| table_type_identifier_matches(ancestor, target_type))
}

fn table_type_is_element(target_type: &str) -> bool {
    let normalized = canonical_table_type(target_type);
    normalized == "element" || normalized.ends_with("::element")
}

fn table_type_identifier_matches(element: &Element, target_type: &str) -> bool {
    let target = canonical_table_type(target_type);
    [element.element_id.as_str(), element.kind.as_ref()]
        .into_iter()
        .any(|candidate| {
            let candidate = canonical_table_type(candidate);
            candidate == target
                || candidate.ends_with(&format!("::{target}"))
                || (!target.contains("::") && label_for_id(&candidate).contains(&target))
        })
}

fn canonical_table_type(value: &str) -> String {
    let normalized = value
        .trim()
        .replace('.', "::")
        .replace(' ', "")
        .to_ascii_lowercase();
    normalized
        .strip_suffix("def")
        .map(|stem| format!("{stem}definition"))
        .unwrap_or(normalized)
}

fn table_row(graph: &Graph, element: &Element, columns: &[TableColumnSpecDto]) -> TableRowDto {
    TableRowDto {
        id: element.element_id.clone(),
        element: element.element_id.clone(),
        cells: columns
            .iter()
            .map(|column| TableCellDto {
                key: column.key.clone(),
                value: table_cell_value(graph, element, column),
            })
            .collect(),
    }
}

fn table_cell_value(graph: &Graph, element: &Element, column: &TableColumnSpecDto) -> String {
    let path = column.path.as_deref().unwrap_or(&column.key);
    if path.starts_with("metadata[") {
        return resolve_metadata_path(element, path).unwrap_or_default();
    }
    if path.contains('.') {
        return resolve_element_path(graph, element, path).unwrap_or_default();
    }

    match path {
        "id" | "element" => element.element_id.clone(),
        "kind" => element.kind.to_string(),
        "owner" => effective_property_text(graph, element, "owner")
            .or_else(|| owner_label(graph, element))
            .unwrap_or_default(),
        "name" => effective_property_text(graph, element, "declared_name")
            .or_else(|| effective_property_text(graph, element, "name"))
            .unwrap_or_else(|| label_for_id(&element.element_id)),
        "text" => effective_property_text(graph, element, "text")
            .or_else(|| effective_property_text(graph, element, "body"))
            .or_else(|| effective_property_text(graph, element, "doc"))
            .unwrap_or_default(),
        other => effective_property_text(graph, element, other).unwrap_or_default(),
    }
}

fn resolve_metadata_path(element: &Element, path: &str) -> Option<String> {
    let remainder = path.strip_prefix("metadata[")?;
    let (type_name, field_path) = remainder.split_once("].")?;
    if type_name.trim().is_empty() || field_path.trim().is_empty() {
        return None;
    }
    metadata_annotations_named(&element.properties.to_btree_map(), type_name)
        .into_iter()
        .find_map(|annotation| value_path_text(&annotation.properties, field_path))
}

fn value_path_text(value: &Value, path: &str) -> Option<String> {
    let mut current = value;
    for segment in path.split('.') {
        if segment.trim().is_empty() {
            return None;
        }
        current = current.get(segment)?;
    }
    Some(value_to_text(current))
}

fn resolve_element_path(graph: &Graph, element: &Element, path: &str) -> Option<String> {
    let mut segments = path.split('.');
    let first = segments.next()?;
    let mut current = resolve_element_reference(graph, element, first)?;
    let mut tail = segments.peekable();
    while let Some(segment) = tail.next() {
        if tail.peek().is_none() {
            return match segment {
                "id" | "element" => Some(current.element_id.clone()),
                "kind" => Some(current.kind.to_string()),
                "name" => property_text(current, "declared_name")
                    .or_else(|| property_text(current, "name"))
                    .or_else(|| Some(label_for_id(&current.element_id))),
                other => property_text(current, other),
            };
        }
        current = resolve_element_reference(graph, current, segment)?;
    }
    Some(label_for_id(&current.element_id))
}

fn resolve_element_reference<'a>(
    graph: &'a Graph,
    element: &'a Element,
    key: &str,
) -> Option<&'a Element> {
    if key == "self" {
        return Some(element);
    }
    property_text(element, key)
        .and_then(|id| graph.element_by_element_id(&id))
        .or_else(|| {
            graph
                .outgoing(element.id, key)
                .next()
                .and_then(|edge| graph.element(edge.target))
        })
}

fn owner_label(graph: &Graph, element: &Element) -> Option<String> {
    graph
        .outgoing(element.id, "owner")
        .next()
        .and_then(|edge| graph.element(edge.target))
        .map(|owner| label_for_id(&owner.element_id))
}

fn property_text(element: &Element, key: &str) -> Option<String> {
    element.properties.get(key).map(value_to_text)
}

fn effective_property_text(graph: &Graph, element: &Element, key: &str) -> Option<String> {
    property_text(element, key).or_else(|| {
        let ancestors = collect_specialization_ancestors(graph, element.id);
        effective_properties(&ancestors, &element.properties.to_btree_map())
            .get(key)
            .map(value_to_text)
    })
}

fn value_to_text(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => value.clone(),
        Value::Array(values) => values
            .iter()
            .map(value_to_text)
            .collect::<Vec<_>>()
            .join(", "),
        Value::Object(_) => value.to_string(),
    }
}

pub fn render_diagram(
    graph: &Graph,
    metamodel_registry: &MetamodelAttributeRegistry,
    spec: DiagramSpecDto,
) -> Result<DiagramViewDto, DiagramError> {
    if spec.version != 1 {
        return Err(DiagramError::UnsupportedVersion(spec.version));
    }

    match spec.kind {
        DiagramKindDto::Structure => render_structure_diagram(graph, metamodel_registry, spec),
        DiagramKindDto::Activity => render_activity_diagram(graph, metamodel_registry, spec),
        DiagramKindDto::StateMachine => {
            render_state_machine_diagram(graph, metamodel_registry, spec)
        }
        _ => Err(DiagramError::UnsupportedKind(spec.kind)),
    }
}

fn render_activity_diagram(
    graph: &Graph,
    metamodel_registry: &MetamodelAttributeRegistry,
    mut spec: DiagramSpecDto,
) -> Result<DiagramViewDto, DiagramError> {
    if spec.query.relations.is_empty() {
        spec.query.relations = vec![
            "owner".to_string(),
            "control_flow".to_string(),
            "object_flow".to_string(),
        ];
    }

    let mut view = render_structure_diagram(graph, metamodel_registry, spec)?;
    apply_activity_symbol_defaults(&mut view);
    Ok(view)
}

fn apply_activity_symbol_defaults(view: &mut DiagramViewDto) {
    let node_roles = view
        .nodes
        .iter()
        .map(|node| (node.symbol.clone(), activity_node_symbol(node)))
        .collect::<std::collections::BTreeMap<_, _>>();

    for symbol in &mut view.symbols {
        if symbol.role == "element" {
            if let Some((role, properties)) = node_roles.get(&symbol.id) {
                symbol.role = role.clone();
                symbol.properties = properties.clone();
            }
        } else if symbol.role == "edge" {
            let relation = symbol.relation.as_deref().unwrap_or_default();
            symbol.properties = edge_symbol_properties(relation);
        }
    }
}

fn activity_node_symbol(node: &DiagramNodeDto) -> (String, serde_json::Map<String, Value>) {
    let kind = node.kind.to_ascii_lowercase();
    let mut properties = serde_json::Map::new();
    let (role, shape) = if kind.contains("activity") {
        ("frame", "activity_frame")
    } else if kind.contains("object") {
        ("object_node", "object_node")
    } else if kind.contains("actionusage") || kind.ends_with("::action") {
        ("action", "action")
    } else if kind.contains("decision") {
        ("decision", "decision")
    } else if kind.contains("merge") {
        ("merge", "decision")
    } else if kind.contains("initial") {
        ("initial", "initial")
    } else if kind.contains("final") {
        ("activity_final", "activity_final")
    } else if kind.contains("parameter") {
        ("parameter", "parameter")
    } else {
        ("element", "node")
    };
    properties.insert("shape".to_string(), Value::String(shape.to_string()));
    if role == "object_node" {
        properties.insert("streaming".to_string(), Value::Bool(true));
    }
    (role.to_string(), properties)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StateDiagramState {
    id: String,
    label: String,
    owner_id: Option<String>,
    parent_state_id: Option<String>,
    is_initial: bool,
    is_final: bool,
    is_orthogonal: bool,
    is_history: bool,
}

fn render_state_machine_diagram(
    graph: &Graph,
    metamodel_registry: &MetamodelAttributeRegistry,
    spec: DiagramSpecDto,
) -> Result<DiagramViewDto, DiagramError> {
    let mut warnings = Vec::new();
    let initial_state_ids = graph
        .elements()
        .iter()
        .filter_map(state_diagram_initial_marker_target)
        .collect::<BTreeSet<_>>();
    let state_index = graph
        .elements()
        .iter()
        .filter(|element| include_element(element, &spec.query))
        .filter(|element| is_state_diagram_state(element))
        .map(|element| {
            let id = element.element_id.clone();
            (
                id.clone(),
                StateDiagramState {
                    id,
                    label: state_diagram_label(element),
                    owner_id: state_diagram_owner_id(element),
                    parent_state_id: state_diagram_parent_state_id(element),
                    is_initial: initial_state_ids.contains(&element.element_id)
                        || state_diagram_bool_property(element, &["is_initial", "initial"])
                        || state_diagram_string_property(
                            element,
                            &["purpose", "state_kind", "kind_role"],
                        )
                        .is_some_and(|value| value.eq_ignore_ascii_case("initial")),
                    is_final: state_diagram_bool_property(element, &["is_final", "final"])
                        || state_diagram_string_property(
                            element,
                            &["purpose", "state_kind", "kind_role"],
                        )
                        .is_some_and(|value| value.eq_ignore_ascii_case("final")),
                    is_orthogonal: state_diagram_bool_property(
                        element,
                        &["is_orthogonal", "orthogonal"],
                    ) || state_diagram_string_property(
                        element,
                        &["state_kind", "kind_role"],
                    )
                    .is_some_and(|value| value.eq_ignore_ascii_case("orthogonal")),
                    is_history: state_diagram_bool_property(element, &["is_history", "history"])
                        || state_diagram_string_property(
                            element,
                            &["purpose", "state_kind", "kind_role"],
                        )
                        .is_some_and(|value| value.eq_ignore_ascii_case("history")),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let root_id = if let Some(root) = spec.root.as_deref().filter(|root| !root.trim().is_empty()) {
        Some(
            resolve_root(graph, root)
                .ok_or_else(|| DiagramError::RootNotFound(root.to_string()))?
                .element_id
                .clone(),
        )
    } else {
        None
    };

    let mut selected_ids = state_index
        .values()
        .filter(|state| {
            root_id
                .as_deref()
                .map(|root_id| {
                    state_diagram_state_matches_root(graph, &state_index, state, root_id)
                })
                .unwrap_or(true)
        })
        .map(|state| state.id.clone())
        .collect::<Vec<_>>();
    selected_ids.sort();
    selected_ids.dedup();

    let max_nodes = effective_max_nodes(&spec.query);
    let selected_total = selected_ids.len();
    selected_ids.truncate(max_nodes);
    if selected_total > selected_ids.len() {
        warnings.push(format!(
            "State-machine diagram node limit reached; showing {} of {selected_total} states.",
            selected_ids.len()
        ));
    }
    let selected_state_ids = selected_ids.into_iter().collect::<BTreeSet<_>>();

    let mut nodes = selected_state_ids
        .iter()
        .filter_map(|state_id| {
            graph
                .element_by_element_id(state_id)
                .zip(state_index.get(state_id))
        })
        .map(|(element, state)| state_diagram_node(graph, metamodel_registry, element, state))
        .collect::<Vec<_>>();
    nodes.sort_by(|left, right| left.id.cmp(&right.id));

    if nodes.is_empty() {
        warnings.push("No state-machine states matched the requested filters.".to_string());
    }

    let mut transitions_seen = 0usize;
    let mut edges = Vec::new();
    let max_edges = effective_max_edges(&spec.query);
    for transition in graph
        .elements()
        .iter()
        .filter(|element| include_element(element, &spec.query))
        .filter(|element| is_state_diagram_transition(element))
    {
        let Some(source_ref) = state_diagram_transition_source(transition) else {
            continue;
        };
        let Some(target_ref) = state_diagram_transition_target(transition) else {
            continue;
        };
        transitions_seen += 1;
        let Some(source) =
            resolve_state_diagram_reference(&source_ref, &selected_state_ids, &state_index)
        else {
            continue;
        };
        let Some(target) =
            resolve_state_diagram_reference(&target_ref, &selected_state_ids, &state_index)
        else {
            continue;
        };
        edges.push(DiagramEdgeDto {
            id: transition.element_id.clone(),
            symbol: symbol_id_for_transition_edge(&transition.element_id),
            source,
            target,
            relation: "transition".to_string(),
            label: state_diagram_transition_label(transition),
        });
        if edges.len() >= max_edges {
            warnings.push(format!(
                "State-machine diagram edge limit reached; showing first {max_edges} transitions."
            ));
            break;
        }
    }
    edges.sort_by(|left, right| left.id.cmp(&right.id));
    edges.dedup_by(|left, right| left.id == right.id);
    if edges.is_empty() && transitions_seen > 0 && !nodes.is_empty() {
        warnings.push("No transitions connected the selected state nodes.".to_string());
    }

    let symbols = nodes
        .iter()
        .map(|node| {
            let mut properties = serde_json::Map::new();
            if let Some(state) = state_index.get(&node.id) {
                properties.insert("shape".to_string(), Value::String("state".to_string()));
                properties.insert("is_initial".to_string(), Value::Bool(state.is_initial));
                properties.insert("is_final".to_string(), Value::Bool(state.is_final));
                properties.insert("is_history".to_string(), Value::Bool(state.is_history));
                if let Some(parent_state_id) = &state.parent_state_id {
                    properties.insert(
                        "parent_state_id".to_string(),
                        Value::String(parent_state_id.clone()),
                    );
                }
            }
            DiagramSymbolDto {
                id: node.symbol.clone(),
                element: node.id.clone(),
                role: "state".to_string(),
                source: None,
                target: None,
                relation: None,
                properties,
            }
        })
        .chain(edges.iter().map(|edge| {
            let transition = graph.element_by_element_id(&edge.id);
            DiagramSymbolDto {
                id: edge.symbol.clone(),
                element: edge.id.clone(),
                role: "transition".to_string(),
                source: Some(symbol_id_for_element(&edge.source)),
                target: Some(symbol_id_for_element(&edge.target)),
                relation: Some(edge.relation.clone()),
                properties: transition
                    .map(state_diagram_transition_symbol_properties)
                    .unwrap_or_else(|| edge_symbol_properties("transition")),
            }
        }))
        .collect();

    Ok(DiagramViewDto {
        spec,
        symbols,
        nodes,
        edges,
        warnings,
    })
}

fn state_diagram_node(
    graph: &Graph,
    metamodel_registry: &MetamodelAttributeRegistry,
    element: &Element,
    state: &StateDiagramState,
) -> DiagramNodeDto {
    let mut node = diagram_node(graph, metamodel_registry, element);
    node.label = state.label.clone();
    node.badges = Vec::new();
    if state.is_initial {
        node.badges.push("initial".to_string());
    }
    if state.is_final {
        node.badges.push("final".to_string());
    }
    if state.is_orthogonal {
        node.badges.push("orthogonal".to_string());
    }
    if state.is_history {
        node.badges.push("history".to_string());
    }
    if node.badges.is_empty() {
        node.badges.push("state".to_string());
    }
    node.properties
        .insert("is_initial".to_string(), Value::Bool(state.is_initial));
    node.properties
        .insert("is_final".to_string(), Value::Bool(state.is_final));
    node.properties.insert(
        "is_orthogonal".to_string(),
        Value::Bool(state.is_orthogonal),
    );
    node.properties
        .insert("is_history".to_string(), Value::Bool(state.is_history));
    if let Some(owner_id) = &state.owner_id {
        node.properties.insert(
            "state_machine_owner".to_string(),
            Value::String(owner_id.clone()),
        );
    }
    if let Some(parent_state_id) = &state.parent_state_id {
        node.properties.insert(
            "parent_state_id".to_string(),
            Value::String(parent_state_id.clone()),
        );
    }
    node
}

fn is_state_diagram_state(element: &Element) -> bool {
    let kind = element.kind.to_ascii_lowercase();
    kind.contains("stateusage")
        || kind.contains("stateaction")
        || state_diagram_string_property(element, &["type", "definition"])
            .is_some_and(|value| value.contains("States::StateAction"))
        || state_diagram_string_property(element, &["metatype"])
            .is_some_and(|value| value.contains("StateUsage"))
}

fn is_state_diagram_transition(element: &Element) -> bool {
    let kind = element.kind.to_ascii_lowercase();
    kind.contains("transition")
        || kind.contains("succession")
        || (kind.contains("acceptaction") && state_diagram_transition_target(element).is_some())
        || (state_diagram_string_property(element, &["metatype", "type", "definition"])
            .is_some_and(|value| {
                value.contains("AcceptAction") || value.contains("SuccessionFlow")
            })
            && state_diagram_transition_target(element).is_some())
        || element.element_id.starts_with("transition.")
}

fn state_diagram_initial_marker_target(element: &Element) -> Option<String> {
    if state_diagram_string_property(element, &["source_is_initial", "sourceIsInitial"])
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
        || state_diagram_bool_property(element, &["source_is_initial", "sourceIsInitial"])
    {
        return state_diagram_transition_target(element)
            .or_else(|| state_diagram_transition_source(element));
    }

    let kind = element.kind.to_ascii_lowercase();
    let initial_completion = (kind.contains("succession")
        || state_diagram_string_property(element, &["metatype", "type", "definition"])
            .is_some_and(|value| value.contains("SuccessionFlow")))
        && state_diagram_string_property(element, &["trigger_kind", "triggerKind"])
            .is_some_and(|value| value.eq_ignore_ascii_case("completion"))
        && state_diagram_transition_source(element).is_none();
    if initial_completion {
        state_diagram_transition_target(element)
    } else {
        None
    }
}

fn state_diagram_state_matches_root(
    graph: &Graph,
    state_index: &BTreeMap<String, StateDiagramState>,
    state: &StateDiagramState,
    root_id: &str,
) -> bool {
    if state.id == root_id {
        return true;
    }

    let mut seen = BTreeSet::new();
    let mut queue = state_diagram_container_ids(state)
        .into_iter()
        .collect::<VecDeque<_>>();
    while let Some(container_id) = queue.pop_front() {
        if container_id == root_id {
            return true;
        }
        if !seen.insert(container_id.clone()) {
            continue;
        }
        if let Some(container_state) = state_index.get(&container_id) {
            queue.extend(state_diagram_container_ids(container_state));
        }
        if let Some(container_element) = graph.element_by_element_id(&container_id) {
            queue.extend(state_diagram_owner_ids(container_element));
        }
    }

    false
}

fn state_diagram_container_ids(state: &StateDiagramState) -> Vec<String> {
    state
        .parent_state_id
        .iter()
        .chain(state.owner_id.iter())
        .cloned()
        .collect()
}

fn state_diagram_owner_ids(element: &Element) -> Vec<String> {
    [
        "owner",
        "owning_type",
        "owningType",
        "owning_definition",
        "owningDefinition",
        "owning_namespace",
        "owningNamespace",
    ]
    .iter()
    .filter_map(|key| state_diagram_string_property(element, &[*key]))
    .collect()
}

fn state_diagram_owner_id(element: &Element) -> Option<String> {
    state_diagram_string_property(
        element,
        &[
            "owner",
            "owning_type",
            "owningType",
            "owning_definition",
            "owningDefinition",
            "owning_namespace",
            "owningNamespace",
        ],
    )
}

fn state_diagram_parent_state_id(element: &Element) -> Option<String> {
    state_diagram_string_property(
        element,
        &[
            "parent_state",
            "parentState",
            "owning_state",
            "owningState",
            "enclosing_state",
            "enclosingState",
        ],
    )
}

fn state_diagram_transition_source(element: &Element) -> Option<String> {
    state_diagram_string_property(
        element,
        &[
            "source",
            "source_state",
            "sourceState",
            "from",
            "transition_source",
            "transitionSource",
        ],
    )
}

fn state_diagram_transition_target(element: &Element) -> Option<String> {
    state_diagram_string_property(
        element,
        &[
            "target",
            "target_state",
            "targetState",
            "to",
            "transition_target",
            "transitionTarget",
        ],
    )
}

fn resolve_state_diagram_reference(
    reference: &str,
    selected_state_ids: &BTreeSet<String>,
    state_index: &BTreeMap<String, StateDiagramState>,
) -> Option<String> {
    let reference = reference.trim();
    if selected_state_ids.contains(reference) {
        return Some(reference.to_string());
    }
    selected_state_ids.iter().find_map(|state_id| {
        let state = state_index.get(state_id)?;
        if state.label.eq_ignore_ascii_case(reference)
            || label_for_id(&state.id).eq_ignore_ascii_case(reference)
            || state.id.ends_with(&format!(".{reference}"))
            || state.id.ends_with(&format!("::{reference}"))
        {
            Some(state.id.clone())
        } else {
            None
        }
    })
}

fn state_diagram_transition_label(element: &Element) -> String {
    state_diagram_string_property(element, &["trigger", "event"])
        .or_else(|| state_diagram_string_property(element, &["trigger_kind", "triggerKind"]))
        .or_else(|| {
            element
                .properties
                .get("guard")
                .map(|guard| format!("[{}]", value_to_text(guard)))
        })
        .filter(|label| !label.trim().is_empty())
        .unwrap_or_else(|| label_for_id(&element.element_id))
}

fn state_diagram_transition_symbol_properties(element: &Element) -> serde_json::Map<String, Value> {
    let mut properties = edge_symbol_properties("transition");
    properties.insert("shape".to_string(), Value::String("transition".to_string()));
    if let Some(trigger) = state_diagram_string_property(element, &["trigger", "event"]) {
        properties.insert("trigger".to_string(), Value::String(trigger));
    }
    if let Some(trigger_kind) =
        state_diagram_string_property(element, &["trigger_kind", "triggerKind"])
    {
        properties.insert("trigger_kind".to_string(), Value::String(trigger_kind));
    }
    if let Some(guard) = element.properties.get("guard") {
        properties.insert("guard".to_string(), guard.clone());
    }
    if let Some(effect) =
        state_diagram_string_property(element, &["effect", "effect_action", "effectAction"])
    {
        properties.insert("effect".to_string(), Value::String(effect));
    }
    properties
}

fn state_diagram_label(element: &Element) -> String {
    state_diagram_string_property(
        element,
        &[
            "declared_name",
            "declaredName",
            "name",
            "qualified_name",
            "qualifiedName",
        ],
    )
    .map(|label| {
        label
            .rsplit(['.', ':'])
            .find(|part| !part.is_empty())
            .unwrap_or(&label)
            .to_string()
    })
    .unwrap_or_else(|| label_for_id(&element.element_id))
}

fn state_diagram_string_property(element: &Element, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        element
            .properties
            .get(*key)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn state_diagram_bool_property(element: &Element, keys: &[&str]) -> bool {
    keys.iter().any(|key| match element.properties.get(*key) {
        Some(Value::Bool(value)) => *value,
        Some(Value::String(value)) => value.eq_ignore_ascii_case("true"),
        _ => false,
    })
}

fn symbol_id_for_transition_edge(id: &str) -> String {
    format!("symbol.transition.{}", sanitize_symbol_segment(id))
}

fn render_structure_diagram(
    graph: &Graph,
    metamodel_registry: &MetamodelAttributeRegistry,
    spec: DiagramSpecDto,
) -> Result<DiagramViewDto, DiagramError> {
    let total_start = Instant::now();
    let mut timings = Vec::new();
    let mut warnings = Vec::new();

    let relation_start = Instant::now();
    let relations = if spec.query.relations.is_empty() {
        default_diagram_relations()
    } else {
        spec.query.relations.clone()
    };
    timings.push(("relations", relation_start.elapsed()));

    let traversal_start = Instant::now();
    let traversal = if let Some(root) = spec.root.as_deref().filter(|root| !root.trim().is_empty())
    {
        let root_start = Instant::now();
        let root = resolve_root(graph, root)
            .ok_or_else(|| DiagramError::RootNotFound(root.to_string()))?;
        timings.push(("root", root_start.elapsed()));
        collect_structure_ids(graph, root.id, &spec.query, &relations)
    } else {
        collect_unrooted_structure_ids(graph, &spec.query)
    };
    timings.push(("traversal", traversal_start.elapsed()));
    warnings.extend(traversal.warnings);

    let node_start = Instant::now();
    let mut nodes = traversal
        .visible_ids
        .iter()
        .filter_map(|node_id| graph.element(*node_id))
        .filter(|element| include_element(element, &spec.query))
        .take(effective_max_nodes(&spec.query))
        .map(|element| diagram_node(graph, metamodel_registry, element))
        .collect::<Vec<_>>();
    nodes.sort_by(|left, right| left.id.cmp(&right.id));
    timings.push(("nodes", node_start.elapsed()));

    if nodes.is_empty() {
        warnings.push("No diagram nodes matched the requested filters.".to_string());
    }
    if traversal.visible_ids.len() > nodes.len() {
        warnings.push(format!(
            "Diagram node limit reached; showing {} of {} traversed nodes.",
            nodes.len(),
            traversal.visible_ids.len()
        ));
    }

    let edge_start = Instant::now();
    let retained_ids = nodes
        .iter()
        .map(|node| node.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut edges = Vec::new();
    let max_edges = effective_max_edges(&spec.query);
    'node_edges: for node_id in &traversal.visible_ids {
        for edge in graph.outgoing_edges(*node_id) {
            if !relations
                .iter()
                .any(|relation| relation.as_str() == edge.relation.as_ref())
            {
                continue;
            }
            let Some(source) = graph.element_id(edge.source) else {
                continue;
            };
            let Some(target) = graph.element_id(edge.target) else {
                continue;
            };
            if retained_ids.contains(source) && retained_ids.contains(target) {
                edges.push(DiagramEdgeDto {
                    id: format!("{}:{}:{}", edge.relation, source, target),
                    symbol: symbol_id_for_edge(&edge.relation, source, target),
                    source: source.to_string(),
                    target: target.to_string(),
                    relation: edge.relation.to_string(),
                    label: edge.relation.to_string(),
                });
                if edges.len() >= max_edges {
                    warnings.push(format!(
                        "Diagram edge limit reached; showing first {max_edges} matching edges."
                    ));
                    break 'node_edges;
                }
            }
        }
    }
    edges.sort_by(|left, right| left.id.cmp(&right.id));
    edges.dedup_by(|left, right| left.id == right.id);
    timings.push(("edges", edge_start.elapsed()));

    timings.push(("total", total_start.elapsed()));
    let slow_phases = timings
        .iter()
        .filter(|(_, elapsed)| elapsed.as_millis() >= TIMING_WARNING_THRESHOLD_MS)
        .map(|(phase, elapsed)| format!("{phase}={}ms", elapsed.as_millis()))
        .collect::<Vec<_>>();
    if !slow_phases.is_empty() {
        warnings.push(format!(
            "Diagram render timing: {}.",
            slow_phases.join(", ")
        ));
    }

    Ok(DiagramViewDto {
        spec,
        symbols: nodes
            .iter()
            .map(|node| DiagramSymbolDto {
                id: node.symbol.clone(),
                element: node.id.clone(),
                role: "element".to_string(),
                source: None,
                target: None,
                relation: None,
                properties: serde_json::Map::new(),
            })
            .chain(edges.iter().map(|edge| DiagramSymbolDto {
                id: edge.symbol.clone(),
                element: edge.id.clone(),
                role: "edge".to_string(),
                source: Some(symbol_id_for_element(&edge.source)),
                target: Some(symbol_id_for_element(&edge.target)),
                relation: Some(edge.relation.to_string()),
                properties: edge_symbol_properties(edge.relation.as_str()),
            }))
            .collect(),
        nodes,
        edges,
        warnings,
    })
}

struct StructureTraversal {
    visible_ids: BTreeSet<NodeId>,
    warnings: Vec<String>,
}

fn collect_structure_ids(
    graph: &Graph,
    root_id: NodeId,
    query: &DiagramQueryOptionsDto,
    relations: &[String],
) -> StructureTraversal {
    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::from([(root_id, 0usize)]);
    let mut warnings = Vec::new();
    let max_depth = query.depth.min(DEFAULT_MAX_DEPTH);
    let max_nodes = effective_max_nodes(query);
    if query.depth > max_depth {
        warnings.push(format!(
            "Diagram depth limit reached; requested depth {} capped at {max_depth}.",
            query.depth
        ));
    }

    while let Some((node_id, depth)) = queue.pop_front() {
        if !visited.insert(node_id) {
            continue;
        }
        if visited.len() >= max_nodes {
            warnings.push(format!(
                "Diagram traversal node limit reached at {max_nodes} nodes."
            ));
            break;
        }
        if depth >= max_depth {
            continue;
        }

        if matches!(
            query.direction,
            DiagramDirectionDto::Parents | DiagramDirectionDto::Both
        ) {
            for relation in relations {
                let adjacent = parent_node_ids(graph, node_id, relation).collect::<Vec<_>>();
                for adjacent_id in adjacent.iter().take(MAX_RELATION_FANOUT_PER_NODE) {
                    queue.push_back((*adjacent_id, depth + 1));
                }
                if adjacent.len() > MAX_RELATION_FANOUT_PER_NODE {
                    warnings.push(format!(
                        "Diagram relation fan-out limit reached for `{relation}`."
                    ));
                }
            }
        }

        if matches!(
            query.direction,
            DiagramDirectionDto::Children | DiagramDirectionDto::Both
        ) {
            for relation in relations {
                let adjacent = child_node_ids(graph, node_id, relation).collect::<Vec<_>>();
                for adjacent_id in adjacent.iter().take(MAX_RELATION_FANOUT_PER_NODE) {
                    queue.push_back((*adjacent_id, depth + 1));
                }
                if adjacent.len() > MAX_RELATION_FANOUT_PER_NODE {
                    warnings.push(format!(
                        "Diagram relation fan-out limit reached for incoming `{relation}`."
                    ));
                }
            }
        }
    }

    StructureTraversal {
        visible_ids: visited,
        warnings,
    }
}

fn parent_node_ids<'a>(
    graph: &'a Graph,
    node_id: NodeId,
    relation: &'a str,
) -> Box<dyn Iterator<Item = NodeId> + 'a> {
    if relation == "part" {
        Box::new(graph.incoming(node_id, relation).map(|edge| edge.source))
    } else {
        Box::new(graph.outgoing(node_id, relation).map(|edge| edge.target))
    }
}

fn child_node_ids<'a>(
    graph: &'a Graph,
    node_id: NodeId,
    relation: &'a str,
) -> Box<dyn Iterator<Item = NodeId> + 'a> {
    if relation == "part" {
        Box::new(graph.outgoing(node_id, relation).map(|edge| edge.target))
    } else {
        Box::new(graph.incoming(node_id, relation).map(|edge| edge.source))
    }
}

fn collect_unrooted_structure_ids(
    graph: &Graph,
    query: &DiagramQueryOptionsDto,
) -> StructureTraversal {
    let max_nodes = effective_max_nodes(query);
    let matching_elements = graph
        .elements()
        .iter()
        .filter(|element| include_element(element, query))
        .collect::<Vec<_>>();
    let mut visible_ids = matching_elements
        .iter()
        .copied()
        .filter(|element| is_top_level_package(graph, element))
        .take(max_nodes)
        .map(|element| element.id)
        .collect::<BTreeSet<_>>();
    if !visible_ids.is_empty() {
        collect_owned_descendant_ids(graph, query, &mut visible_ids, max_nodes);
    }
    if visible_ids.is_empty() {
        visible_ids = matching_elements
            .iter()
            .copied()
            .take(max_nodes)
            .map(|element| element.id)
            .collect::<BTreeSet<_>>();
    }
    let mut warnings = Vec::new();
    if matching_elements.len() > visible_ids.len() {
        warnings.push(format!(
            "Diagram node limit reached; showing first {} matching nodes.",
            visible_ids.len()
        ));
    }

    StructureTraversal {
        visible_ids,
        warnings,
    }
}

fn collect_owned_descendant_ids(
    graph: &Graph,
    query: &DiagramQueryOptionsDto,
    visible_ids: &mut BTreeSet<NodeId>,
    max_nodes: usize,
) {
    let max_depth = query.depth.min(DEFAULT_MAX_DEPTH);
    let ownership_relations = ["owner", "ownedElement", "ownedMember"];
    let mut queue = visible_ids
        .iter()
        .copied()
        .map(|node_id| (node_id, 0usize))
        .collect::<VecDeque<_>>();

    while let Some((node_id, depth)) = queue.pop_front() {
        if visible_ids.len() >= max_nodes || depth >= max_depth {
            continue;
        }

        for relation in ownership_relations {
            for edge in graph
                .incoming(node_id, relation)
                .take(MAX_RELATION_FANOUT_PER_NODE)
            {
                let child_id = edge.source;
                if visible_ids.len() >= max_nodes {
                    return;
                }
                if graph
                    .element(child_id)
                    .is_some_and(|element| include_element(element, query))
                    && visible_ids.insert(child_id)
                {
                    queue.push_back((child_id, depth + 1));
                }
            }
            for edge in graph
                .outgoing(node_id, relation)
                .take(MAX_RELATION_FANOUT_PER_NODE)
            {
                let child_id = edge.target;
                if visible_ids.len() >= max_nodes {
                    return;
                }
                if graph
                    .element(child_id)
                    .is_some_and(|element| include_element(element, query))
                    && visible_ids.insert(child_id)
                {
                    queue.push_back((child_id, depth + 1));
                }
            }
        }
    }
}

fn is_top_level_package(graph: &Graph, element: &Element) -> bool {
    if !element.kind.to_ascii_lowercase().contains("package") {
        return false;
    }
    owner_ids(element).all(|owner| graph.element_by_element_id(owner).is_none())
}

fn owner_ids(element: &Element) -> impl Iterator<Item = &str> {
    element
        .properties
        .get("owner")
        .into_iter()
        .flat_map(|value| match value {
            Value::String(owner) => vec![owner.as_str()],
            Value::Array(values) => values
                .iter()
                .filter_map(|entry| entry.as_str())
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        })
}

fn resolve_root<'a>(graph: &'a Graph, root: &str) -> Option<&'a Element> {
    if let Some(element) = graph.element_by_element_id(root) {
        return Some(element);
    }

    let normalized_root = root.trim().to_ascii_lowercase();
    graph.elements().iter().find(|element| {
        label_for_id(&element.element_id).to_ascii_lowercase() == normalized_root
            || element
                .element_id
                .rsplit("::")
                .next()
                .is_some_and(|name| name.eq_ignore_ascii_case(root))
            || element
                .element_id
                .rsplit('.')
                .next()
                .is_some_and(|name| name.eq_ignore_ascii_case(root))
    })
}

fn include_element(element: &Element, query: &DiagramQueryOptionsDto) -> bool {
    if element.layer < 2 {
        return query.include_libraries;
    }

    query.include_user_model
}

fn diagram_node(
    graph: &Graph,
    metamodel_registry: &MetamodelAttributeRegistry,
    element: &Element,
) -> DiagramNodeDto {
    let attributes =
        mercurio_core::query_element_attributes(graph, metamodel_registry, element.id, None)
            .map(|query| query.rows)
            .unwrap_or_default()
            .into_iter()
            .map(|attribute| DiagramAttributeDto {
                name: attribute.name,
                type_label: attribute
                    .effective_value
                    .as_ref()
                    .map(|value| value_type_label(value).to_string()),
            })
            .collect();

    DiagramNodeDto {
        id: element.element_id.clone(),
        symbol: symbol_id_for_element(&element.element_id),
        label: label_for_id(&element.element_id),
        kind: element.kind.to_string(),
        layer: element.layer,
        badges: vec![format!("L{}", element.layer)],
        attributes,
        properties: element
            .properties
            .iter()
            .map(|(key, value)| (key.to_string(), value.clone()))
            .collect(),
    }
}

fn symbol_id_for_element(id: &str) -> String {
    format!(
        "symbol.{}",
        id.chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() {
                    character
                } else {
                    '_'
                }
            })
            .collect::<String>()
    )
}

fn symbol_id_for_edge(relation: &str, source: &str, target: &str) -> String {
    format!(
        "symbol.edge.{}.{}.{}",
        sanitize_symbol_segment(relation),
        sanitize_symbol_segment(source),
        sanitize_symbol_segment(target)
    )
}

fn sanitize_symbol_segment(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn edge_symbol_properties(relation: &str) -> serde_json::Map<String, Value> {
    let mut properties = serde_json::Map::new();
    properties.insert(
        "route".to_string(),
        Value::String(default_route(relation).to_string()),
    );
    properties.insert(
        "source_decoration".to_string(),
        Value::String(default_source_decoration(relation).to_string()),
    );
    properties.insert(
        "target_decoration".to_string(),
        Value::String(default_target_decoration(relation).to_string()),
    );
    properties.insert(
        "label_placement".to_string(),
        Value::String("above".to_string()),
    );
    properties
}

fn default_route(relation: &str) -> &'static str {
    match relation {
        "part" | "control_flow" | "object_flow" => "orthogonal",
        "source" | "target" | "transition" => "straight",
        _ => "straight",
    }
}

fn default_source_decoration(relation: &str) -> &'static str {
    match relation {
        "part" => "filled_diamond",
        _ => "none",
    }
}

fn default_target_decoration(relation: &str) -> &'static str {
    match relation {
        "specializes" => "hollow_triangle",
        "part" | "source" | "target" | "transition" | "control_flow" | "object_flow" => {
            "open_arrow"
        }
        _ => "open_arrow",
    }
}

fn label_for_id(id: &str) -> String {
    id.rsplit("::")
        .next()
        .and_then(|segment| segment.rsplit('.').next())
        .filter(|segment| !segment.is_empty())
        .unwrap_or(id)
        .to_string()
}

fn default_diagram_relations() -> Vec<String> {
    vec!["specializes".to_string()]
}

fn default_diagram_depth() -> usize {
    3
}

fn default_max_nodes() -> usize {
    DEFAULT_MAX_NODES
}

fn default_max_edges() -> usize {
    DEFAULT_MAX_EDGES
}

fn effective_max_nodes(query: &DiagramQueryOptionsDto) -> usize {
    query.max_nodes.clamp(1, DEFAULT_MAX_NODES)
}

fn effective_max_edges(query: &DiagramQueryOptionsDto) -> usize {
    query.max_edges.clamp(1, DEFAULT_MAX_EDGES)
}

fn default_layout_engine() -> String {
    "dagre".to_string()
}

fn default_layout_direction() -> String {
    "LR".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mercurio_core::{KirDocument, KirElement};
    use serde_json::json;
    use std::collections::BTreeMap;

    fn sample_graph() -> (Graph, MetamodelAttributeRegistry) {
        let document = view_fixture_document();
        let graph = Graph::from_document(document).expect("sample graph should be valid");
        let registry = MetamodelAttributeRegistry::build(&graph);
        (graph, registry)
    }

    fn view_fixture_document() -> KirDocument {
        fn element(
            id: &str,
            kind: &str,
            layer: u8,
            properties: BTreeMap<String, Value>,
        ) -> KirElement {
            KirElement {
                id: id.to_string(),
                kind: kind.to_string(),
                layer,
                properties,
            }
        }

        KirDocument {
            metadata: BTreeMap::new(),
            elements: vec![
                element(
                    "Comment",
                    "Metaclass",
                    1,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("Comment")),
                        (
                            "features".to_string(),
                            json!(["metafeature.Comment.body", "metafeature.Comment.locale"]),
                        ),
                    ]),
                ),
                element(
                    "metafeature.Comment.body",
                    "MetamodelFeature",
                    1,
                    BTreeMap::from([
                        ("owner".to_string(), json!("Comment")),
                        ("kir_property".to_string(), json!("body")),
                        ("type_label".to_string(), json!("String")),
                    ]),
                ),
                element(
                    "metafeature.Comment.locale",
                    "MetamodelFeature",
                    1,
                    BTreeMap::from([
                        ("owner".to_string(), json!("Comment")),
                        ("kir_property".to_string(), json!("locale")),
                        ("type_label".to_string(), json!("String")),
                    ]),
                ),
                element(
                    "Model::Kernel::ComponentDefinition",
                    "PartDefinition",
                    1,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("ComponentDefinition")),
                        (
                            "qualified_name".to_string(),
                            json!("Model::Kernel::ComponentDefinition"),
                        ),
                    ]),
                ),
                element(
                    "pkg.Example",
                    "Package",
                    2,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("Example")),
                        ("qualified_name".to_string(), json!("Example")),
                        (
                            "members".to_string(),
                            json!([
                                "type.Example.Vehicle",
                                "state.Example.DriveMode",
                                "activity.Example.Startup",
                                "req.Example.SafeStart",
                                "comment.Example.Note",
                                "comment.Example.LocalizedNote"
                            ]),
                        ),
                    ]),
                ),
                element(
                    "type.Example.Vehicle",
                    "PartDefinition",
                    2,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("Vehicle")),
                        ("qualified_name".to_string(), json!("Example.Vehicle")),
                        ("owner".to_string(), json!("pkg.Example")),
                        (
                            "specializes".to_string(),
                            json!(["Model::Kernel::ComponentDefinition"]),
                        ),
                    ]),
                ),
                element(
                    "feature.Example.Vehicle.controller",
                    "PartUsage",
                    2,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("controller")),
                        (
                            "qualified_name".to_string(),
                            json!("Example.Vehicle.controller"),
                        ),
                        ("owner".to_string(), json!("type.Example.Vehicle")),
                        ("owning_type".to_string(), json!("type.Example.Vehicle")),
                    ]),
                ),
                element(
                    "state.Example.DriveMode",
                    "StateUsage",
                    2,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("DriveMode")),
                        ("qualified_name".to_string(), json!("Example.DriveMode")),
                        ("owner".to_string(), json!("pkg.Example")),
                        ("source".to_string(), json!("state.Example.Parked")),
                        ("target".to_string(), json!("state.Example.Driving")),
                    ]),
                ),
                element(
                    "state.Example.Parked",
                    "StateUsage",
                    2,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("Parked")),
                        ("qualified_name".to_string(), json!("Example.Parked")),
                        ("owner".to_string(), json!("state.Example.DriveMode")),
                        ("is_initial".to_string(), json!(true)),
                    ]),
                ),
                element(
                    "state.Example.Driving",
                    "StateUsage",
                    2,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("Driving")),
                        ("qualified_name".to_string(), json!("Example.Driving")),
                        ("owner".to_string(), json!("state.Example.DriveMode")),
                        ("is_final".to_string(), json!(true)),
                    ]),
                ),
                element(
                    "transition.Example.DriveMode.ParkedToDriving",
                    "TransitionUsage",
                    2,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("ParkedToDriving")),
                        (
                            "qualified_name".to_string(),
                            json!("Example.DriveMode.ParkedToDriving"),
                        ),
                        ("owner".to_string(), json!("state.Example.DriveMode")),
                        ("source".to_string(), json!("state.Example.Parked")),
                        ("target".to_string(), json!("state.Example.Driving")),
                        ("trigger".to_string(), json!("drive")),
                        ("trigger_kind".to_string(), json!("event")),
                    ]),
                ),
                element(
                    "activity.Example.Startup",
                    "ActivityUsage",
                    2,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("Startup")),
                        ("qualified_name".to_string(), json!("Example.Startup")),
                        ("owner".to_string(), json!("pkg.Example")),
                    ]),
                ),
                element(
                    "action.Example.Startup.Validate",
                    "ActionUsage",
                    2,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("Validate")),
                        (
                            "qualified_name".to_string(),
                            json!("Example.Startup.Validate"),
                        ),
                        ("owner".to_string(), json!("activity.Example.Startup")),
                    ]),
                ),
                element(
                    "flow.Example.Startup.Validate",
                    "ControlFlow",
                    2,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("ValidateFlow")),
                        (
                            "qualified_name".to_string(),
                            json!("Example.Startup.ValidateFlow"),
                        ),
                        ("owner".to_string(), json!("activity.Example.Startup")),
                        ("source".to_string(), json!("activity.Example.Startup")),
                        (
                            "target".to_string(),
                            json!("action.Example.Startup.Validate"),
                        ),
                    ]),
                ),
                element(
                    "req.Example.SafeStart",
                    "RequirementUsage",
                    2,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("SafeStart")),
                        ("qualified_name".to_string(), json!("Example.SafeStart")),
                        ("owner".to_string(), json!("pkg.Example")),
                        ("requirement_id".to_string(), json!("REQ-001")),
                        (
                            "text".to_string(),
                            json!("The vehicle shall prevent unsafe starts."),
                        ),
                        (
                            "metadata".to_string(),
                            json!({
                                "Review": {
                                    "properties": {
                                        "status": "approved",
                                        "owner": "Foundation Team",
                                        "reviewDate": "2026-06-03"
                                    }
                                }
                            }),
                        ),
                    ]),
                ),
                element(
                    "comment.Example.Note",
                    "Comment",
                    2,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("Note")),
                        ("qualified_name".to_string(), json!("Example.Note")),
                        ("owner".to_string(), json!("pkg.Example")),
                        ("metatype".to_string(), json!("Comment")),
                        ("body".to_string(), json!("Primary model note.")),
                        ("locale".to_string(), json!("en-US")),
                    ]),
                ),
                element(
                    "comment.Example.LocalizedNote",
                    "Comment",
                    2,
                    BTreeMap::from([
                        ("declared_name".to_string(), json!("LocalizedNote")),
                        ("qualified_name".to_string(), json!("Example.LocalizedNote")),
                        ("owner".to_string(), json!("pkg.Example")),
                        ("metatype".to_string(), json!("Comment")),
                        ("body".to_string(), json!("Localized model note.")),
                        ("locale".to_string(), json!("fr-FR")),
                    ]),
                ),
            ],
        }
    }

    fn render_sample(spec: DiagramSpecDto) -> DiagramViewDto {
        let (graph, registry) = sample_graph();
        render_diagram(&graph, &registry, spec).expect("sample diagram should render")
    }

    fn render_table_sample(spec: TableSpecDto) -> TableViewDto {
        let (graph, registry) = sample_graph();
        render_table(&graph, &registry, spec).expect("sample table should render")
    }

    fn structure_spec(root: Option<&str>, relations: Vec<&str>) -> DiagramSpecDto {
        DiagramSpecDto {
            version: 1,
            kind: DiagramKindDto::Structure,
            title: "Sample".to_string(),
            description: None,
            root: root.map(str::to_string),
            query: DiagramQueryOptionsDto {
                relations: relations.into_iter().map(str::to_string).collect(),
                direction: DiagramDirectionDto::Children,
                depth: 3,
                include_libraries: false,
                include_user_model: true,
                max_nodes: 350,
                max_edges: 900,
            },
            layout: DiagramLayoutOptionsDto::default(),
            style: DiagramStyleOptionsDto::default(),
        }
    }

    #[test]
    fn structure_diagram_renders_sample_package_containment() {
        let view = render_sample(structure_spec(Some("pkg.Example"), vec!["owner"]));

        assert!(view.nodes.iter().any(|node| node.id == "pkg.Example"));
        assert!(
            view.nodes
                .iter()
                .any(|node| node.id == "type.Example.Vehicle")
        );
        assert!(
            view.symbols
                .iter()
                .any(|symbol| symbol.element == "type.Example.Vehicle"
                    && symbol.id == "symbol.type_Example_Vehicle")
        );
        assert!(
            view.nodes
                .iter()
                .any(|node| node.id == "state.Example.DriveMode")
        );
        assert!(view.edges.iter().any(|edge| {
            edge.source == "type.Example.Vehicle"
                && edge.target == "pkg.Example"
                && edge.relation == "owner"
        }));
        assert!(view.warnings.is_empty());
    }

    #[test]
    fn structure_diagram_preserves_feature_relationship() {
        let view = render_sample(structure_spec(
            Some("type.Example.Vehicle"),
            vec!["owning_type"],
        ));

        assert!(
            view.nodes
                .iter()
                .any(|node| node.id == "type.Example.Vehicle")
        );
        assert!(
            view.nodes
                .iter()
                .any(|node| node.id == "feature.Example.Vehicle.controller")
        );
        let feature_edge = view
            .edges
            .iter()
            .find(|edge| {
                edge.source == "feature.Example.Vehicle.controller"
                    && edge.target == "type.Example.Vehicle"
                    && edge.relation == "owning_type"
            })
            .expect("feature edge should render");
        let feature_symbol = view
            .symbols
            .iter()
            .find(|symbol| symbol.id == feature_edge.symbol)
            .expect("feature edge should have a symbol");
        assert_eq!(feature_symbol.role, "edge");
        assert_eq!(
            feature_symbol.source.as_deref(),
            Some("symbol.feature_Example_Vehicle_controller")
        );
        assert_eq!(
            feature_symbol.target.as_deref(),
            Some("symbol.type_Example_Vehicle")
        );
        assert_eq!(feature_symbol.relation.as_deref(), Some("owning_type"));
        assert_eq!(
            feature_symbol
                .properties
                .get("route")
                .and_then(|value| value.as_str()),
            Some("straight")
        );
        assert_eq!(
            feature_symbol
                .properties
                .get("target_decoration")
                .and_then(|value| value.as_str()),
            Some("open_arrow")
        );
    }

    #[test]
    fn structure_diagram_validates_unknown_root() {
        let (graph, registry) = sample_graph();
        let error = render_diagram(
            &graph,
            &registry,
            structure_spec(Some("Vehicle::Missing"), vec!["owner"]),
        )
        .expect_err("unknown root should fail");

        assert_eq!(
            error,
            DiagramError::RootNotFound("Vehicle::Missing".to_string())
        );
    }

    #[test]
    fn structure_diagram_honors_include_library_filter() {
        let mut spec = structure_spec(
            Some("Model::Kernel::ComponentDefinition"),
            vec!["specializes"],
        );
        spec.query.include_libraries = true;
        spec.query.include_user_model = true;
        let with_libraries = render_sample(spec.clone());
        assert!(
            with_libraries
                .nodes
                .iter()
                .any(|node| node.id == "Model::Kernel::ComponentDefinition")
        );

        spec.query.include_libraries = false;
        let without_libraries = render_sample(spec);
        assert!(
            without_libraries
                .nodes
                .iter()
                .all(|node| node.id != "Model::Kernel::ComponentDefinition")
        );
    }

    #[test]
    fn structure_diagram_reports_edge_limit() {
        let mut spec = structure_spec(
            Some("state.Example.DriveMode"),
            vec!["owner", "source", "target"],
        );
        spec.query.max_edges = 2;
        let view = render_sample(spec);

        assert_eq!(view.edges.len(), 2);
        assert!(
            view.warnings
                .iter()
                .any(|warning| warning.contains("Diagram edge limit reached"))
        );
    }

    #[test]
    fn activity_diagram_assigns_activity_symbol_roles() {
        let view = render_sample(DiagramSpecDto {
            version: 1,
            kind: DiagramKindDto::Activity,
            title: "Startup Activity".to_string(),
            description: None,
            root: Some("activity.Example.Startup".to_string()),
            query: DiagramQueryOptionsDto {
                relations: vec![
                    "owner".to_string(),
                    "source".to_string(),
                    "target".to_string(),
                ],
                direction: DiagramDirectionDto::Children,
                depth: 3,
                include_libraries: false,
                include_user_model: true,
                max_nodes: 350,
                max_edges: 900,
            },
            layout: DiagramLayoutOptionsDto::default(),
            style: DiagramStyleOptionsDto::default(),
        });

        let action = view
            .symbols
            .iter()
            .find(|symbol| symbol.element == "action.Example.Startup.Validate")
            .expect("activity action should have a symbol");
        assert_eq!(action.role, "action");
        assert_eq!(
            action
                .properties
                .get("shape")
                .and_then(|value| value.as_str()),
            Some("action")
        );

        let flow = view
            .symbols
            .iter()
            .find(|symbol| symbol.relation.as_deref() == Some("source"))
            .expect("activity source flow should have a symbol");
        assert_eq!(flow.role, "edge");
        assert_eq!(
            flow.properties
                .get("route")
                .and_then(|value| value.as_str()),
            Some("straight")
        );
        assert_eq!(
            flow.properties
                .get("target_decoration")
                .and_then(|value| value.as_str()),
            Some("open_arrow")
        );
    }

    #[test]
    fn state_machine_diagram_renders_states_and_transitions() {
        let view = render_sample(DiagramSpecDto {
            version: 1,
            kind: DiagramKindDto::StateMachine,
            title: "Drive Mode".to_string(),
            description: None,
            root: Some("state.Example.DriveMode".to_string()),
            query: DiagramQueryOptionsDto {
                relations: Vec::new(),
                direction: DiagramDirectionDto::Children,
                depth: 3,
                include_libraries: false,
                include_user_model: true,
                max_nodes: 350,
                max_edges: 900,
            },
            layout: DiagramLayoutOptionsDto::default(),
            style: DiagramStyleOptionsDto::default(),
        });

        assert_eq!(view.spec.kind, DiagramKindDto::StateMachine);
        assert!(
            view.nodes
                .iter()
                .any(|node| node.id == "state.Example.Parked"
                    && node.label == "Parked"
                    && node.badges.contains(&"initial".to_string()))
        );
        assert!(
            view.nodes
                .iter()
                .any(|node| node.id == "state.Example.Driving"
                    && node.badges.contains(&"final".to_string()))
        );
        let transition = view
            .edges
            .iter()
            .find(|edge| edge.id == "transition.Example.DriveMode.ParkedToDriving")
            .expect("transition edge should render");
        assert_eq!(transition.source, "state.Example.Parked");
        assert_eq!(transition.target, "state.Example.Driving");
        assert_eq!(transition.relation, "transition");
        assert_eq!(transition.label, "drive");

        let state_symbol = view
            .symbols
            .iter()
            .find(|symbol| symbol.element == "state.Example.Parked")
            .expect("state symbol should render");
        assert_eq!(state_symbol.role, "state");
        assert_eq!(
            state_symbol
                .properties
                .get("shape")
                .and_then(|value| value.as_str()),
            Some("state")
        );
        assert_eq!(
            state_symbol
                .properties
                .get("is_initial")
                .and_then(|value| value.as_bool()),
            Some(true)
        );

        let transition_symbol = view
            .symbols
            .iter()
            .find(|symbol| symbol.element == "transition.Example.DriveMode.ParkedToDriving")
            .expect("transition symbol should render");
        assert_eq!(transition_symbol.role, "transition");
        assert_eq!(
            transition_symbol
                .properties
                .get("target_decoration")
                .and_then(|value| value.as_str()),
            Some("open_arrow")
        );
        assert_eq!(
            transition_symbol
                .properties
                .get("trigger_kind")
                .and_then(|value| value.as_str()),
            Some("event")
        );
    }

    #[test]
    fn state_machine_diagram_resolves_shorthand_transition_endpoints() {
        let shorthand = KirElement {
            id: "transition.Example.DriveMode.Short".to_string(),
            kind: "TransitionUsage".to_string(),
            layer: 2,
            properties: BTreeMap::from([
                ("declared_name".to_string(), json!("Short")),
                ("owner".to_string(), json!("state.Example.DriveMode")),
                ("transitionSource".to_string(), json!("Parked")),
                ("transitionTarget".to_string(), json!("Driving")),
                ("trigger".to_string(), json!("short")),
            ]),
        };
        let mut document = view_fixture_document();
        document.elements.push(shorthand);
        let graph = Graph::from_document(document).expect("sample graph should rebuild");
        let registry = MetamodelAttributeRegistry::build(&graph);

        let view = render_diagram(
            &graph,
            &registry,
            DiagramSpecDto {
                version: 1,
                kind: DiagramKindDto::StateMachine,
                title: "Drive Mode".to_string(),
                description: None,
                root: Some("state.Example.DriveMode".to_string()),
                query: DiagramQueryOptionsDto {
                    relations: Vec::new(),
                    direction: DiagramDirectionDto::Children,
                    depth: 3,
                    include_libraries: false,
                    include_user_model: true,
                    max_nodes: 350,
                    max_edges: 900,
                },
                layout: DiagramLayoutOptionsDto::default(),
                style: DiagramStyleOptionsDto::default(),
            },
        )
        .expect("state-machine diagram should render");

        assert!(view.edges.iter().any(|edge| {
            edge.id == "transition.Example.DriveMode.Short"
                && edge.source == "state.Example.Parked"
                && edge.target == "state.Example.Driving"
                && edge.label == "short"
        }));
    }

    #[test]
    fn requirements_table_renders_requirement_rows() {
        let view = render_table_sample(TableSpecDto {
            version: 1,
            kind: TableKindDto::Requirements,
            title: "Requirements".to_string(),
            description: None,
            root: Some("pkg.Example".to_string()),
            target_type: None,
            query: DiagramQueryOptionsDto {
                relations: vec!["owner".to_string()],
                direction: DiagramDirectionDto::Children,
                depth: 2,
                include_libraries: false,
                include_user_model: true,
                max_nodes: 350,
                max_edges: 900,
            },
            columns: Vec::new(),
        });

        assert_eq!(view.columns.len(), 5);
        let row = view
            .rows
            .iter()
            .find(|row| row.element == "req.Example.SafeStart")
            .expect("requirement row should render");
        assert!(
            row.cells
                .iter()
                .any(|cell| { cell.key == "requirement_id" && cell.value == "REQ-001" })
        );
        assert!(
            row.cells
                .iter()
                .any(|cell| cell.key == "text" && cell.value.contains("unsafe starts"))
        );
    }

    #[test]
    fn view_document_round_trips_diagram_spec() {
        let document = ViewDocumentDto::diagram(structure_spec(Some("pkg.Example"), vec!["owner"]));

        validate_view_document(&document).expect("document should validate");
        let json = serde_json::to_string_pretty(&document).unwrap();
        let decoded: ViewDocumentDto = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.schema, VIEW_SCHEMA);
        assert_eq!(decoded.version, VIEW_SPEC_VERSION);
        assert_eq!(decoded.kind, "diagram.structure");
        assert!(decoded.diagram.is_some());
        assert!(decoded.table.is_none());
        assert_eq!(decoded, document);
    }

    #[test]
    fn view_document_round_trips_table_spec() {
        let document = ViewDocumentDto::table(TableSpecDto {
            version: 1,
            kind: TableKindDto::Requirements,
            title: "Requirements".to_string(),
            description: None,
            root: Some("pkg.Example".to_string()),
            target_type: None,
            query: DiagramQueryOptionsDto::default(),
            columns: vec![TableColumnSpecDto {
                key: "requirement_id".to_string(),
                label: "ID".to_string(),
                path: None,
            }],
        });

        validate_view_document(&document).expect("document should validate");
        let decoded: ViewDocumentDto =
            serde_json::from_str(&serde_json::to_string(&document).unwrap()).unwrap();

        assert_eq!(decoded.kind, "table");
        assert!(decoded.diagram.is_none());
        assert!(decoded.table.is_some());
        assert_eq!(decoded, document);
    }

    #[test]
    fn view_document_round_trips_parameterized_view() {
        let mut parameters = BTreeMap::new();
        parameters.insert(
            "seedId".to_string(),
            Value::String("pkg.Example".to_string()),
        );

        let document = ViewDocumentDto {
            schema: VIEW_SCHEMA.to_string(),
            version: VIEW_SPEC_VERSION,
            kind: "explorer.metatype".to_string(),
            mode: ViewModeDto::Visualization,
            parameters,
            diagram: None,
            table: None,
        };

        validate_view_document(&document).expect("document should validate");
        let decoded: ViewDocumentDto =
            serde_json::from_str(&serde_json::to_string(&document).unwrap()).unwrap();

        assert_eq!(decoded.kind, "explorer.metatype");
        assert_eq!(
            decoded.parameters.get("seedId"),
            Some(&Value::String("pkg.Example".to_string()))
        );
        assert!(decoded.diagram.is_none());
        assert!(decoded.table.is_none());
        assert_eq!(decoded, document);
    }

    #[test]
    fn view_document_validation_rejects_mismatched_payload() {
        let mut document =
            ViewDocumentDto::diagram(structure_spec(Some("pkg.Example"), vec!["owner"]));
        document.kind = "table".to_string();

        let diagnostics =
            validate_view_document(&document).expect_err("mismatched kind should fail");

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "view.kind")
        );
    }

    #[test]
    fn view_document_validation_rejects_bad_table_columns() {
        let document = ViewDocumentDto::table(TableSpecDto {
            version: 1,
            kind: TableKindDto::Requirements,
            title: "Requirements".to_string(),
            description: None,
            root: None,
            target_type: None,
            query: DiagramQueryOptionsDto::default(),
            columns: vec![
                TableColumnSpecDto {
                    key: "id".to_string(),
                    label: "ID".to_string(),
                    path: None,
                },
                TableColumnSpecDto {
                    key: "id".to_string(),
                    label: String::new(),
                    path: None,
                },
            ],
        });

        let diagnostics =
            validate_view_document(&document).expect_err("invalid columns should fail");

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.code == "view.table.column.key.duplicate" })
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "view.table.column.label")
        );
    }

    #[test]
    fn requirements_table_supports_attribute_path_columns() {
        let view = render_table_sample(TableSpecDto {
            version: 1,
            kind: TableKindDto::Requirements,
            title: "Requirements".to_string(),
            description: None,
            root: Some("pkg.Example".to_string()),
            target_type: None,
            query: DiagramQueryOptionsDto {
                relations: vec!["owner".to_string()],
                direction: DiagramDirectionDto::Children,
                depth: 2,
                include_libraries: false,
                include_user_model: true,
                max_nodes: 350,
                max_edges: 900,
            },
            columns: vec![
                TableColumnSpecDto {
                    key: "id".to_string(),
                    label: "ID".to_string(),
                    path: Some("requirement_id".to_string()),
                },
                TableColumnSpecDto {
                    key: "owner_name".to_string(),
                    label: "Owner Name".to_string(),
                    path: Some("owner.declared_name".to_string()),
                },
            ],
        });

        let row = view.rows.first().expect("requirement row should render");
        assert!(
            row.cells
                .iter()
                .any(|cell| { cell.key == "id" && cell.value == "REQ-001" })
        );
        assert!(
            row.cells
                .iter()
                .any(|cell| { cell.key == "owner_name" && cell.value == "Example" })
        );
    }

    #[test]
    fn requirements_table_supports_metadata_path_columns() {
        let view = render_table_sample(TableSpecDto {
            version: 1,
            kind: TableKindDto::Requirements,
            title: "Requirement Lifecycle".to_string(),
            description: None,
            root: Some("pkg.Example".to_string()),
            target_type: None,
            query: DiagramQueryOptionsDto {
                relations: vec!["owner".to_string()],
                direction: DiagramDirectionDto::Children,
                depth: 2,
                include_libraries: false,
                include_user_model: true,
                max_nodes: 350,
                max_edges: 900,
            },
            columns: vec![
                TableColumnSpecDto {
                    key: "status".to_string(),
                    label: "Status".to_string(),
                    path: Some("metadata[Review].status".to_string()),
                },
                TableColumnSpecDto {
                    key: "owner".to_string(),
                    label: "Owner".to_string(),
                    path: Some("metadata[Review].owner".to_string()),
                },
                TableColumnSpecDto {
                    key: "review_date".to_string(),
                    label: "Review Date".to_string(),
                    path: Some("metadata[Review].reviewDate".to_string()),
                },
            ],
        });

        let row = view.rows.first().expect("requirement row should render");
        assert!(
            row.cells
                .iter()
                .any(|cell| cell.key == "status" && cell.value == "approved")
        );
        assert!(
            row.cells
                .iter()
                .any(|cell| cell.key == "owner" && cell.value == "Foundation Team")
        );
        assert!(
            row.cells
                .iter()
                .any(|cell| cell.key == "review_date" && cell.value == "2026-06-03")
        );
    }

    #[test]
    fn element_table_filters_by_type_and_subtypes() {
        let view = render_table_sample(TableSpecDto {
            version: 1,
            kind: TableKindDto::Elements,
            title: "Comments".to_string(),
            description: None,
            root: None,
            target_type: Some("Comment".to_string()),
            query: DiagramQueryOptionsDto::default(),
            columns: vec![
                TableColumnSpecDto {
                    key: "name".to_string(),
                    label: "Name".to_string(),
                    path: None,
                },
                TableColumnSpecDto {
                    key: "body".to_string(),
                    label: "Body".to_string(),
                    path: Some("body".to_string()),
                },
                TableColumnSpecDto {
                    key: "locale".to_string(),
                    label: "Locale".to_string(),
                    path: Some("locale".to_string()),
                },
            ],
        });

        assert!(
            view.rows
                .iter()
                .any(|row| row.element == "comment.Example.Note")
        );
        assert!(
            view.rows
                .iter()
                .any(|row| row.element == "comment.Example.LocalizedNote")
        );
        assert!(
            view.rows
                .iter()
                .all(|row| row.element.contains("Comment") || row.element.contains("comment."))
        );
        assert!(
            view.available_columns
                .iter()
                .any(|column| column.key == "body")
        );
        assert!(
            view.available_columns
                .iter()
                .any(|column| column.key == "locale")
        );
    }
}

fn default_true() -> bool {
    true
}

fn diagram_kind_name(kind: &DiagramKindDto) -> &'static str {
    match kind {
        DiagramKindDto::Structure => "structure",
        DiagramKindDto::Activity => "activity",
        DiagramKindDto::StateMachine => "state_machine",
        DiagramKindDto::PackageTree => "package_tree",
        DiagramKindDto::CompositionGraph => "composition_graph",
        DiagramKindDto::ReferenceGraph => "reference_graph",
        DiagramKindDto::DependencyGraph => "dependency_graph",
        DiagramKindDto::MetatypeInstanceMap => "metatype_instance_map",
        DiagramKindDto::ImpactView => "impact_view",
        DiagramKindDto::PropertyInheritance => "property_inheritance",
        DiagramKindDto::ValidationView => "validation_view",
    }
}

fn value_type_label(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
