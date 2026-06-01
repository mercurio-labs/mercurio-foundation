use mercurio_language_contracts::SourceLanguage;
use mercurio_language_contracts::diagnostics::Diagnostic;

pub use crate::lowering::emit::{
    DefaultSpecializationAnchorsSeed, EmissionRule, EmissionSpec, KirEmissionSeed, MappingBundle,
    PilotConstructEntry, PilotConstructSeed, SemanticSpecializationDefaultsSeed, StdlibAliasSeed,
    UsageSemanticSpecializationOverrideSeed,
};
pub use crate::lowering::rules::{
    LoweringAstPattern, LoweringCollectRule, LoweringElaborationRule, LoweringEmitRule,
    LoweringPilotSources, LoweringRule, LoweringRuleSeed,
};

#[derive(Clone)]
pub struct LanguageProfile {
    pub id: String,
    pub language: SourceLanguage,
    pub mappings: &'static MappingBundle,
    pub lowering_rules: Option<&'static LoweringRuleSeed>,
}

impl LanguageProfile {
    pub fn load(language: SourceLanguage) -> Result<Self, Diagnostic> {
        let id = match language {
            SourceLanguage::Kerml => "kerml-bootstrap".to_string(),
            SourceLanguage::Sysml => "sysml-2.0-pilot-0.57.0".to_string(),
        };
        let mappings = MappingBundle::load_for_language(language)?;
        let lowering_rules = LoweringRuleSeed::load_for_language(language)?;
        Ok(Self {
            id,
            language,
            mappings,
            lowering_rules,
        })
    }

    pub fn load_for_profile(id: impl Into<String>) -> Result<Self, Diagnostic> {
        let id = id.into();
        let mappings = MappingBundle::load_for_profile(&id)?;
        let lowering_rules = LoweringRuleSeed::load_for_profile(&id)?;
        Ok(Self {
            id,
            language: SourceLanguage::Sysml,
            mappings,
            lowering_rules,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sysml_profile_loads_declarative_lowering_rules() {
        let profile = LanguageProfile::load_for_profile("sysml-2.0-pilot-0.57.0").unwrap();
        let rules = profile.lowering_rules.expect("sysml lowering rules");

        assert_eq!(rules.schema_version, 1);
        assert!(rules.rules.iter().any(|rule| rule.construct == "PartUsage"));
    }

    #[test]
    fn kerml_profile_has_no_sysml_lowering_rules() {
        let profile = LanguageProfile::load(SourceLanguage::Kerml).unwrap();

        assert!(profile.lowering_rules.is_none());
    }
}
