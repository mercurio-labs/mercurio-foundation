//! Explicit strict comparison profiles. No legacy language-specific waivers apply.
use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SemanticCompareProfile {
    #[serde(default)]
    pub included_attributes: BTreeSet<String>,
    #[serde(default)]
    pub tolerances: Vec<SemanticCompareTolerance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticCompareTolerance {
    pub id: String,
    pub classification: String,
    pub reason: String,
    #[serde(default)]
    pub tracking_ref: Option<String>,
    #[serde(default)]
    pub scope: Value,
    pub action: Value,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SemanticCompareCoverage {
    pub total_elements: usize,
    pub compared_elements: usize,
    pub compared_fraction: f64,
    pub excluded_elements: usize,
    pub match_key_disambiguations: usize,
}

impl SemanticCompareCoverage {
    pub(super) fn from_counts(total: usize, compared: usize) -> Self {
        Self {
            total_elements: total,
            compared_elements: compared,
            compared_fraction: if total == 0 {
                1.0
            } else {
                compared as f64 / total as f64
            },
            excluded_elements: 0,
            match_key_disambiguations: 0,
        }
    }
}

impl SemanticCompareProfile {
    fn validate(&self) -> Result<(), SemanticCompareError> {
        // Fail closed. Applying the historical profile's waivers here would hide
        // compiler gaps; those actions need a separately reviewed implementation.
        if let Some(tolerance) = self.tolerances.first() {
            return Err(SemanticCompareError::InvalidProfile(format!(
                "tolerance `{}` is not supported by the strict comparator",
                tolerance.id
            )));
        }
        Ok(())
    }
}

pub fn build_semantic_snapshot_with_profile(
    document: KirDocument,
    focus_source_file: &str,
    mode: SnapshotMode,
    profile: &SemanticCompareProfile,
) -> Result<SemanticSnapshot, SemanticCompareError> {
    profile.validate()?;
    let graph = Graph::from_document(document)?;
    let registry = MetamodelAttributeRegistry::build(&graph);
    strict_snapshot(&graph, &registry, focus_source_file, mode)
}

pub fn build_semantic_snapshot_with_registry_and_profile(
    document: KirDocument,
    focus_source_file: &str,
    mode: SnapshotMode,
    registry: &MetamodelAttributeRegistry,
    profile: &SemanticCompareProfile,
) -> Result<SemanticSnapshot, SemanticCompareError> {
    profile.validate()?;
    let graph = Graph::from_document(document)?;
    strict_snapshot(&graph, registry, focus_source_file, mode)
}

fn strict_snapshot(
    graph: &Graph,
    registry: &MetamodelAttributeRegistry,
    focus: &str,
    mode: SnapshotMode,
) -> Result<SemanticSnapshot, SemanticCompareError> {
    let mut elements = Vec::new();
    for element in graph.elements() {
        let Some(metadata) = element
            .properties
            .get("metadata")
            .and_then(Value::as_object)
        else {
            continue;
        };
        let Some(source) = metadata.get("source_file").and_then(Value::as_str) else {
            continue;
        };
        if !source_file_matches_relative_path(source, focus) {
            continue;
        }
        let source_span = metadata.get("source_span").map(|span| SemanticSourceSpan {
            start_line: span.get("start_line").and_then(Value::as_u64),
            end_line: span.get("end_line").and_then(Value::as_u64),
        });
        let declared_name = element
            .properties
            .get("declared_name")
            .and_then(Value::as_str)
            .map(str::to_string);
        let raw_metatype = element.properties.get("metatype").and_then(Value::as_str);
        let query = query_element_attributes(
            graph,
            registry,
            element.id,
            metatype_override_for(mode, graph, element, raw_metatype).as_ref(),
        )
        .unwrap_or_else(|| ElementAttributeQuery {
            metatype: None,
            metatype_specialization_chain: Vec::new(),
            rows: Vec::new(),
        });
        let metatype = raw_metatype
            .map(canonical_identifier)
            .or_else(|| query.metatype.as_ref().map(|m| canonical_identifier(&m.id)));
        let label = declared_name
            .clone()
            .unwrap_or_else(|| canonical_identifier(&element.kind));
        let mut attributes = BTreeMap::new();
        for row in query.rows {
            attributes.insert(
                row.name,
                SemanticSnapshotAttribute {
                    declared_by: row.declared_by.map(|d| d.id),
                    origin_kind: row.origin_kind,
                    has_direct_value: row.has_direct_value,
                    direct_value: row.direct_value,
                    has_effective_value: row.has_effective_value,
                    effective_value: row.effective_value,
                },
            );
        }
        // Preserve every direct semantic property, including properties missing
        // from the current registry. Metadata is provenance, not model semantics.
        for (name, value) in element.properties.to_btree_map() {
            if name == "metadata" {
                continue;
            }
            let attribute = attributes
                .entry(name)
                .or_insert_with(|| SemanticSnapshotAttribute {
                    declared_by: None,
                    origin_kind: "direct".to_string(),
                    has_direct_value: true,
                    direct_value: None,
                    has_effective_value: true,
                    effective_value: Some(value.clone()),
                });
            attribute.has_direct_value = true;
            attribute.direct_value = Some(value);
        }
        elements.push(SemanticSnapshotElement {
            match_key: build_match_key(focus, source_span.as_ref(), &label),
            id: element.element_id.clone(),
            label,
            kind: element.kind.to_string(),
            layer: element.layer,
            declared_name,
            source_span,
            metatype,
            metatype_specialization_chain: sorted_strings(
                query
                    .metatype_specialization_chain
                    .iter()
                    .map(|m| canonical_identifier(&m.id))
                    .collect(),
            ),
            declared_attributes: attributes,
        });
    }
    elements.sort_by(|a, b| a.match_key.cmp(&b.match_key).then_with(|| a.id.cmp(&b.id)));
    disambiguate_duplicate_match_keys(&mut elements);
    ensure_unique_match_keys(&elements)?;
    Ok(SemanticSnapshot {
        focus_source_file: focus.to_string(),
        mode: match mode {
            SnapshotMode::Mercurio => "mercurio",
            SnapshotMode::Pilot => "pilot",
        }
        .to_string(),
        elements,
    })
}

pub fn compare_snapshots_with_profile(
    mercurio: SemanticSnapshot,
    pilot: SemanticSnapshot,
    options: SemanticCompareOptions,
    profile: &SemanticCompareProfile,
) -> Result<SemanticComparisonReport, SemanticCompareError> {
    profile.validate()?;
    let left = snapshot_index(&mercurio.elements)?;
    let right = snapshot_index(&pilot.elements)?;
    let keys = left
        .keys()
        .chain(right.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut mismatches = Vec::new();
    let mut mercurio_only = Vec::new();
    let mut pilot_only = Vec::new();
    let mut exact_match_count = 0;
    let attributes = |element: &SemanticSnapshotElement| {
        element
            .declared_attributes
            .iter()
            .filter(|(name, attribute)| {
                options.include_all_attributes
                    || profile.included_attributes.contains(*name)
                    || (options.include_derived_attributes && attribute.origin_kind == "derived")
            })
            .map(|(name, attribute)| (name.clone(), attribute.clone()))
            .collect::<BTreeMap<_, _>>()
    };
    for key in keys {
        match (left.get(&key), right.get(&key)) {
            (Some(a), Some(b)) => {
                let metatype = (a.metatype != b.metatype).then(|| SemanticValueMismatch {
                    mercurio: a.metatype.clone().unwrap_or_default(),
                    pilot: b.metatype.clone().unwrap_or_default(),
                });
                let metatype_specialization_chain = (a.metatype_specialization_chain
                    != b.metatype_specialization_chain)
                    .then(|| SemanticValueMismatch {
                        mercurio: a.metatype_specialization_chain.clone(),
                        pilot: b.metatype_specialization_chain.clone(),
                    });
                let aa = attributes(a);
                let bb = attributes(b);
                let declared_attributes = (!attributes_are_semantically_equal(&aa, &bb)).then_some(
                    SemanticValueMismatch {
                        mercurio: aa,
                        pilot: bb,
                    },
                );
                if metatype.is_none()
                    && metatype_specialization_chain.is_none()
                    && declared_attributes.is_none()
                {
                    exact_match_count += 1;
                } else {
                    mismatches.push(SemanticElementMismatch {
                        match_key: key,
                        mercurio_id: a.id.clone(),
                        pilot_id: b.id.clone(),
                        metatype,
                        metatype_specialization_chain,
                        declared_attributes,
                    });
                }
            }
            (Some(a), None) => mercurio_only.push((*a).clone()),
            (None, Some(b)) => pilot_only.push((*b).clone()),
            _ => {}
        }
    }
    let mut coverage = SemanticCompareCoverage::from_counts(
        mercurio.elements.len() + pilot.elements.len(),
        2 * (exact_match_count + mismatches.len()),
    );
    coverage.match_key_disambiguations = mercurio
        .elements
        .iter()
        .chain(&pilot.elements)
        .filter(|e| {
            e.match_key
                .rsplit_once('#')
                .is_some_and(|(_, suffix)| suffix.parse::<usize>().is_ok())
        })
        .count();
    Ok(SemanticComparisonReport {
        focus_source_file: mercurio.focus_source_file,
        mercurio_count: mercurio.elements.len(),
        pilot_count: pilot.elements.len(),
        exact_match_count,
        mercurio_only,
        pilot_only,
        mismatches,
        coverage,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kir::KirElement;

    fn document(kind: &str, value: Value) -> KirDocument {
        KirDocument {
            metadata: BTreeMap::new(),
            elements: vec![KirElement {
                id: "example.element".to_string(),
                kind: kind.to_string(),
                layer: 2,
                properties: BTreeMap::from([
                    ("declared_name".to_string(), json!("element")),
                    ("expression_ir".to_string(), value),
                    (
                        "metadata".to_string(),
                        json!({"source_file":"example.sysml", "source_span":{"start_line":1,"end_line":1}}),
                    ),
                ]),
            }],
        }
    }

    #[test]
    fn strict_profile_keeps_all_kinds_and_expression_differences() {
        let profile = SemanticCompareProfile {
            included_attributes: BTreeSet::from(["expression_ir".to_string()]),
            ..Default::default()
        };
        let left = build_semantic_snapshot_with_profile(
            document("Feature", json!({"kind":"literal","value":1})),
            "example.sysml",
            SnapshotMode::Mercurio,
            &profile,
        )
        .unwrap();
        let right = build_semantic_snapshot_with_profile(
            document("Feature", json!({"kind":"literal","value":2})),
            "example.sysml",
            SnapshotMode::Pilot,
            &profile,
        )
        .unwrap();
        assert_eq!(left.elements.len(), 1);
        let report = compare_snapshots_with_profile(
            left,
            right,
            SemanticCompareOptions::default(),
            &profile,
        )
        .unwrap();
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(report.coverage.total_elements, 2);
        assert_eq!(report.coverage.compared_elements, 2);
        assert_eq!(report.coverage.excluded_elements, 0);
    }

    #[test]
    fn strict_profile_exposes_missing_elements_and_preserves_array_order() {
        let profile = SemanticCompareProfile::default();
        let left = build_semantic_snapshot_with_profile(
            document("Comment", json!([1, 2])),
            "example.sysml",
            SnapshotMode::Mercurio,
            &profile,
        )
        .unwrap();
        let right = build_semantic_snapshot_with_profile(
            document("Comment", json!([2, 1])),
            "example.sysml",
            SnapshotMode::Pilot,
            &profile,
        )
        .unwrap();
        let report = compare_snapshots_with_profile(
            left.clone(),
            right,
            SemanticCompareOptions {
                include_all_attributes: true,
                ..Default::default()
            },
            &profile,
        )
        .unwrap();
        assert_eq!(report.mismatches.len(), 1);
        let empty = SemanticSnapshot {
            focus_source_file: "example.sysml".to_string(),
            mode: "pilot".to_string(),
            elements: Vec::new(),
        };
        let report = compare_snapshots_with_profile(
            left,
            empty,
            SemanticCompareOptions::default(),
            &profile,
        )
        .unwrap();
        assert_eq!(report.mercurio_only.len(), 1);
        assert_eq!(report.coverage.compared_fraction, 0.0);
    }

    #[test]
    fn strict_profile_rejects_unimplemented_tolerances() {
        let profile = SemanticCompareProfile {
            included_attributes: BTreeSet::new(),
            tolerances: vec![SemanticCompareTolerance {
                id: "unimplemented".to_string(),
                classification: "mercurio-gap".to_string(),
                reason: "negative control".to_string(),
                tracking_ref: Some("test".to_string()),
                scope: json!({}),
                action: json!({"kind":"excludeElement"}),
            }],
        };
        let error = build_semantic_snapshot_with_profile(
            document("Feature", json!(1)),
            "example.sysml",
            SnapshotMode::Mercurio,
            &profile,
        )
        .unwrap_err();
        assert!(error.to_string().contains("unimplemented"));
    }
}
