use std::collections::BTreeSet;

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
        validate_lowering_rules_against_mappings(lowering_rules, mappings)?;
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
        validate_lowering_rules_against_mappings(lowering_rules, mappings)?;
        Ok(Self {
            id,
            language: SourceLanguage::Sysml,
            mappings,
            lowering_rules,
        })
    }
}

fn validate_lowering_rules_against_mappings(
    lowering_rules: Option<&LoweringRuleSeed>,
    mappings: &MappingBundle,
) -> Result<(), Diagnostic> {
    let Some(lowering_rules) = lowering_rules else {
        return Ok(());
    };

    for rule in &lowering_rules.rules {
        let emission = mappings.emission_for(&rule.metaclass)?;
        if rule.emit.id_template != emission.id_template {
            return Err(Diagnostic::new(
                format!(
                    "lowering rule `{}` id template `{}` does not match emission mapping `{}` template `{}`",
                    rule.construct, rule.emit.id_template, rule.metaclass, emission.id_template
                ),
                None,
            ));
        }
        let emission_properties = emission
            .emit
            .properties
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for property in rule.emit.properties.keys() {
            if !emission_properties.contains(property.as_str()) {
                return Err(Diagnostic::new(
                    format!(
                        "lowering rule `{}` property `{}` is missing from emission mapping `{}`",
                        rule.construct, property, rule.metaclass
                    ),
                    None,
                ));
            }
        }
    }

    Ok(())
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
    fn sysml_mappings_expose_reviewed_package_lowering_rule() {
        let profile = LanguageProfile::load_for_profile("sysml-2.0-pilot-0.57.0").unwrap();
        let rule = profile
            .mappings
            .lowering_rule_for_construct("Package")
            .expect("package lowering rule");

        assert_eq!(rule.status.as_deref(), Some("reviewed"));
        assert_eq!(rule.metaclass, "SysML::Package");
        assert_eq!(rule.emit.id_template, "pkg.{qualified_name}");
    }

    #[test]
    fn sysml_mappings_expose_reviewed_import_lowering_rule() {
        let profile = LanguageProfile::load_for_profile("sysml-2.0-pilot-0.57.0").unwrap();
        let rule = profile
            .mappings
            .lowering_rule_for_construct("Import")
            .expect("import lowering rule");

        assert_eq!(rule.status.as_deref(), Some("reviewed"));
        assert_eq!(rule.metaclass, "SysML::Import");
        assert_eq!(rule.emit.id_template, "import.{owner_id}.{ordinal}");
    }

    #[test]
    fn sysml_mappings_expose_reviewed_definition_lowering_rule() {
        let profile = LanguageProfile::load_for_profile("sysml-2.0-pilot-0.57.0").unwrap();
        let rule = profile
            .mappings
            .lowering_rule_for_construct("PartDefinition")
            .expect("part definition lowering rule");

        assert_eq!(rule.status.as_deref(), Some("reviewed"));
        assert_eq!(rule.metaclass, "SysML::PartDefinition");
        assert_eq!(rule.emit.id_template, "type.{qualified_name}");
    }

    #[test]
    fn sysml_mappings_expose_reviewed_usage_lowering_rule() {
        let profile = LanguageProfile::load_for_profile("sysml-2.0-pilot-0.57.0").unwrap();
        let rule = profile
            .mappings
            .lowering_rule_for_construct("PartUsage")
            .expect("part usage lowering rule");

        assert_eq!(rule.status.as_deref(), Some("reviewed"));
        assert_eq!(rule.metaclass, "SysML::PartUsage");
        assert_eq!(
            rule.emit.id_template,
            "feature.{owner_path}.{declared_name}"
        );
    }

    #[test]
    fn kerml_profile_has_no_sysml_lowering_rules() {
        let profile = LanguageProfile::load(SourceLanguage::Kerml).unwrap();

        assert!(profile.lowering_rules.is_none());
    }
}
