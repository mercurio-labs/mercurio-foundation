use std::collections::BTreeMap;

use crate::language::concepts::SemanticConcept;
use crate::language::profile::{LanguageProfile, LanguageProfileError, default_language_profile};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetamodelConceptRegistry {
    canonical_kinds: BTreeMap<SemanticConcept, String>,
    aliases: BTreeMap<String, String>,
}

impl MetamodelConceptRegistry {
    pub fn from_profile(profile: &LanguageProfile) -> Self {
        Self {
            canonical_kinds: profile.canonical_kinds.clone(),
            aliases: profile.aliases.clone(),
        }
    }

    pub fn canonical_kind(&self, concept: SemanticConcept) -> Option<&str> {
        self.canonical_kinds.get(&concept).map(String::as_str)
    }

    pub fn normalize_kind<'a>(&'a self, kind: &'a str) -> &'a str {
        self.aliases.get(kind).map(String::as_str).unwrap_or(kind)
    }

    pub fn is_kind(&self, kind: &str, concept: SemanticConcept) -> bool {
        self.canonical_kind(concept)
            .map(|canonical| self.normalize_kind(kind) == canonical)
            .unwrap_or(false)
    }
}

pub fn default_metamodel_registry() -> Result<MetamodelConceptRegistry, LanguageProfileError> {
    default_language_profile().map(|profile| MetamodelConceptRegistry::from_profile(&profile))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_accepts_current_aliases() {
        let registry = default_metamodel_registry().unwrap();

        assert!(registry.is_kind(
            "SysML::Systems::PartDefinition",
            SemanticConcept::PartDefinition
        ));
        assert!(registry.is_kind("sysml.PartDefinition", SemanticConcept::PartDefinition));
        assert!(!registry.is_kind("SysML::Systems::PartUsage", SemanticConcept::PartDefinition));
    }
}
