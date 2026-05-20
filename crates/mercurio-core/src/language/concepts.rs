use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceLanguage {
    Kerml,
    Sysml,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticConcept {
    AttributeUsage,
    ConstraintUsage,
    Feature,
    ItemDefinition,
    ItemUsage,
    Package,
    PartDefinition,
    PartUsage,
    RequirementUsage,
    Type,
    VerificationCaseUsage,
    View,
    Viewpoint,
}
