use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::graph::Graph;
use crate::ir::{KirDocument, KirError};
use crate::metamodel::{
    MetamodelFeatureRegistry, MetamodelValidationDiagnostic, validate_derived_metamodel_semantics,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticValidationSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticValidationDiagnostic {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    pub severity: SemanticValidationSeverity,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticValidationReport {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<SemanticValidationDiagnostic>,
}

impl SemanticValidationReport {
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity == SemanticValidationSeverity::Warning)
            .count()
    }
}

pub fn validate_kir_semantics(
    document: &KirDocument,
) -> Result<SemanticValidationReport, KirError> {
    validate_kir_semantics_with_context(document, None)
}

pub fn validate_kir_semantics_with_context(
    document: &KirDocument,
    library_context: Option<&KirDocument>,
) -> Result<SemanticValidationReport, KirError> {
    let target_element_ids = document
        .elements
        .iter()
        .map(|element| element.id.clone())
        .collect::<BTreeSet<_>>();
    let validation_document = match library_context {
        Some(library_context) => KirDocument::merge([library_context.clone(), document.clone()])?,
        None => document.clone(),
    };
    let graph = Graph::from_document(validation_document).map_err(|err| {
        KirError::Model(format!(
            "failed to build graph for semantic validation: {err}"
        ))
    })?;

    Ok(validate_kir_semantics_for_graph_targets(
        &graph,
        Some(&target_element_ids),
    ))
}

pub fn validate_kir_semantics_for_graph(graph: &Graph) -> SemanticValidationReport {
    validate_kir_semantics_for_graph_targets(graph, None)
}

fn validate_kir_semantics_for_graph_targets(
    graph: &Graph,
    target_element_ids: Option<&BTreeSet<String>>,
) -> SemanticValidationReport {
    let registry = MetamodelFeatureRegistry::build(graph);
    let diagnostics = validate_derived_metamodel_semantics(graph, &registry)
        .into_iter()
        .filter(|diagnostic| {
            target_element_ids
                .map(|ids| ids.contains(&diagnostic.element_id))
                .unwrap_or(true)
        })
        .map(semantic_warning_from_metamodel)
        .collect();

    SemanticValidationReport { diagnostics }
}

fn semantic_warning_from_metamodel(
    diagnostic: MetamodelValidationDiagnostic,
) -> SemanticValidationDiagnostic {
    SemanticValidationDiagnostic {
        code: diagnostic.code.to_string(),
        message: diagnostic.message,
        element_id: Some(diagnostic.element_id),
        severity: SemanticValidationSeverity::Warning,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::*;
    use crate::ir::{KIR_SCHEMA_VERSION, KirElement};

    #[test]
    fn metamodel_validation_reports_warnings_against_document_elements() {
        let document = KirDocument {
            metadata: BTreeMap::from([(
                "kir_schema_version".to_string(),
                json!(KIR_SCHEMA_VERSION),
            )]),
            elements: vec![KirElement {
                id: "transition.Demo.start".to_string(),
                kind: "SysML::Systems::TransitionUsage".to_string(),
                layer: 2,
                properties: BTreeMap::from([("source".to_string(), json!("state.Demo.Initial"))]),
            }],
        };
        let library_context = validation_library_context();

        let report =
            validate_kir_semantics_with_context(&document, Some(&library_context)).unwrap();

        assert_eq!(report.warning_count(), 1);
        assert_eq!(
            report.diagnostics[0].code,
            "kir.metamodel.endpoints.incomplete"
        );
        assert_eq!(
            report.diagnostics[0].element_id.as_deref(),
            Some("transition.Demo.start")
        );
    }

    #[test]
    fn validation_context_warnings_are_filtered_out() {
        let document = KirDocument {
            metadata: BTreeMap::from([(
                "kir_schema_version".to_string(),
                json!(KIR_SCHEMA_VERSION),
            )]),
            elements: Vec::new(),
        };
        let mut library_context = validation_library_context();
        library_context.elements.push(KirElement {
            id: "transition.Context.bad".to_string(),
            kind: "SysML::Systems::TransitionUsage".to_string(),
            layer: 1,
            properties: BTreeMap::from([("source".to_string(), json!("context.source"))]),
        });

        let report =
            validate_kir_semantics_with_context(&document, Some(&library_context)).unwrap();

        assert!(report.is_empty());
    }

    #[test]
    fn generic_kir_feature_ownership_without_metamodel_contract_is_quiet() {
        let document = KirDocument {
            metadata: BTreeMap::from([(
                "kir_schema_version".to_string(),
                json!(KIR_SCHEMA_VERSION),
            )]),
            elements: vec![
                KirElement {
                    id: "type.Demo.Vehicle".to_string(),
                    kind: "model.Type".to_string(),
                    layer: 2,
                    properties: BTreeMap::from([(
                        "features".to_string(),
                        json!(["feature.Demo.Vehicle.mass"]),
                    )]),
                },
                KirElement {
                    id: "feature.Demo.Vehicle.mass".to_string(),
                    kind: "model.Feature".to_string(),
                    layer: 2,
                    properties: BTreeMap::from([
                        ("owner".to_string(), json!("type.Demo.Vehicle")),
                        ("declared_name".to_string(), json!("mass")),
                    ]),
                },
            ],
        };

        let report = validate_kir_semantics(&document).unwrap();

        assert!(report.is_empty());
    }

    pub(crate) fn validation_library_context() -> KirDocument {
        KirDocument {
            metadata: BTreeMap::from([(
                "kir_schema_version".to_string(),
                json!(KIR_SCHEMA_VERSION),
            )]),
            elements: vec![
                KirElement {
                    id: "SysML::Systems::TransitionUsage".to_string(),
                    kind: "Metaclass".to_string(),
                    layer: 1,
                    properties: BTreeMap::new(),
                },
                metamodel_feature(
                    "metafeature.TransitionUsage.source",
                    "SysML::Systems::TransitionUsage",
                    "source",
                ),
                metamodel_feature(
                    "metafeature.TransitionUsage.target",
                    "SysML::Systems::TransitionUsage",
                    "target",
                ),
            ],
        }
    }

    fn metamodel_feature(id: &str, owner: &str, kir_property: &str) -> KirElement {
        KirElement {
            id: id.to_string(),
            kind: "MetamodelFeature".to_string(),
            layer: 1,
            properties: BTreeMap::from([
                ("owner".to_string(), json!(owner)),
                ("kir_property".to_string(), json!(kir_property)),
                ("feature_kind".to_string(), json!("reference")),
            ]),
        }
    }
}
