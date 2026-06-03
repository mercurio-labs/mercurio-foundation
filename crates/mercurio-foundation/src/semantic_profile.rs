#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityAnswer {
    Allowed,
    Denied(String),
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributePolicyAnswer {
    pub writable: bool,
    pub reason: Option<String>,
}

pub trait SemanticCapabilityOracle {
    fn can_contain(&self, container_kind: &str, child_kind: &str) -> CapabilityAnswer;
    fn can_specialize(&self, source_kind: &str, target_kind: &str) -> CapabilityAnswer;
    fn can_type_usage(&self, usage_kind: &str, definition_kind: &str) -> CapabilityAnswer;
    fn can_relate(
        &self,
        relationship_kind: &str,
        source_kind: &str,
        target_kind: &str,
    ) -> CapabilityAnswer;
    fn attribute_policy(&self, kind: &str, attribute: &str) -> AttributePolicyAnswer;

    fn relationship_uses_owner_as_source(&self, _relationship_kind: &str) -> bool {
        false
    }

    fn doc_id_attribute_aliases(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn supporting_definition_keyword_for_usage(&self, _usage_kind: &str) -> Option<String> {
        None
    }

    fn normalize_definition_keyword(&self, keyword: &str) -> String {
        keyword.trim().to_string()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConservativeSemanticCapabilityOracle;

impl SemanticCapabilityOracle for ConservativeSemanticCapabilityOracle {
    fn can_contain(&self, container_kind: &str, child_kind: &str) -> CapabilityAnswer {
        if container_kind.is_empty() || child_kind.is_empty() {
            CapabilityAnswer::Unknown("missing kind information".to_string())
        } else if container_kind.eq_ignore_ascii_case("package")
            || container_kind.to_ascii_lowercase().contains("container")
            || container_kind.to_ascii_lowercase().contains("def")
            || container_kind.to_ascii_lowercase().contains("usage")
        {
            CapabilityAnswer::Allowed
        } else {
            CapabilityAnswer::Unknown(format!(
                "no language profile governs whether `{container_kind}` can own `{child_kind}`"
            ))
        }
    }

    fn can_specialize(&self, source_kind: &str, target_kind: &str) -> CapabilityAnswer {
        if source_kind.is_empty() || target_kind.is_empty() {
            CapabilityAnswer::Unknown("missing kind information".to_string())
        } else {
            CapabilityAnswer::Allowed
        }
    }

    fn can_type_usage(&self, usage_kind: &str, definition_kind: &str) -> CapabilityAnswer {
        if usage_kind.is_empty() || definition_kind.is_empty() {
            CapabilityAnswer::Unknown("missing kind information".to_string())
        } else if !definition_kind.to_ascii_lowercase().contains("def") {
            CapabilityAnswer::Denied(format!("`{definition_kind}` is not a definition-like type"))
        } else {
            CapabilityAnswer::Allowed
        }
    }

    fn can_relate(
        &self,
        relationship_kind: &str,
        source_kind: &str,
        target_kind: &str,
    ) -> CapabilityAnswer {
        if source_kind.trim().is_empty() {
            return CapabilityAnswer::Denied(format!(
                "relationship source `{source_kind}` is not element-like"
            ));
        }
        if target_kind.trim().is_empty() {
            return CapabilityAnswer::Denied(format!(
                "relationship target `{target_kind}` is not element-like"
            ));
        }
        if relationship_kind.trim().is_empty() {
            return CapabilityAnswer::Unknown(format!(
                "relationship kind `{relationship_kind}` is not yet governed"
            ));
        }
        CapabilityAnswer::Allowed
    }

    fn attribute_policy(&self, kind: &str, attribute: &str) -> AttributePolicyAnswer {
        let attribute = attribute.to_ascii_lowercase();
        let writable = matches!(
            attribute.as_str(),
            "declared_name"
                | "specializes"
                | "type"
                | "is_abstract"
                | "is_end"
                | "direction"
                | "target"
                | "imports"
                | "expression"
                | "doc"
        );
        AttributePolicyAnswer {
            writable,
            reason: (!writable).then(|| {
                format!("attribute `{attribute}` is not writable on `{kind}` by this service")
            }),
        }
    }
}
