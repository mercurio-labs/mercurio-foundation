use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::authoring::{MutationResult, QualifiedName};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ElementRef {
    pub qualified_name: String,
}

impl ElementRef {
    pub fn new(qualified_name: impl Into<String>) -> Self {
        Self {
            qualified_name: qualified_name.into(),
        }
    }

    pub fn as_qualified_name(&self) -> QualifiedName {
        QualifiedName::parse(&self.qualified_name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRevision {
    pub fingerprint: String,
}

impl WorkspaceRevision {
    pub fn unchecked() -> Self {
        Self {
            fingerprint: "unchecked".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationProposal {
    pub intent: String,
    pub affected_elements: Vec<ElementRef>,
    pub operations: Vec<SemanticMutation>,
    pub evidence: Vec<MutationEvidence>,
    pub rationale: Option<String>,
    pub workspace_revision: WorkspaceRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationEvidence {
    pub element: Option<ElementRef>,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationPlan {
    pub proposal_id: String,
    pub normalized_operations: Vec<SemanticMutation>,
    pub required_supporting_changes: Vec<SemanticMutation>,
    pub checked_against: WorkspaceRevision,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SemanticMutation {
    AddPackage {
        target_file: String,
        name: String,
    },
    AddDefinition {
        container: ElementRef,
        keyword: String,
        name: String,
        specializes: Vec<ElementRef>,
    },
    AddUsage {
        container: ElementRef,
        keyword: String,
        name: String,
        ty: Option<ElementRef>,
        specializes: Vec<ElementRef>,
    },
    AddRelationship {
        kind: String,
        source: ElementRef,
        target: ElementRef,
    },
    RenameDeclaration {
        element: ElementRef,
        new_name: String,
    },
    UpdateUsageType {
        element: ElementRef,
        ty: Option<ElementRef>,
    },
    SetExpression {
        element: ElementRef,
        expression: Option<SemanticExpression>,
    },
    UpdateSpecializations {
        element: ElementRef,
        specializes: Vec<ElementRef>,
    },
    MoveDeclaration {
        element: ElementRef,
        destination: ElementRef,
    },
    SetAttribute {
        element: ElementRef,
        attribute: String,
        value: Value,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SemanticExpression {
    Text(String),
}

impl SemanticExpression {
    pub fn as_text(&self) -> &str {
        match self {
            Self::Text(value) => value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationApplicationResult {
    pub changed_files: BTreeSet<String>,
    pub edited_files: BTreeMap<String, String>,
    pub changed_declarations: BTreeSet<String>,
    pub semantic_diff: SemanticDiff,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SemanticDiff {
    pub added_elements: Vec<ElementRef>,
    pub removed_elements: Vec<ElementRef>,
    pub renamed_elements: Vec<RenamedElement>,
    pub moved_elements: Vec<MovedElement>,
    pub retyped_usages: Vec<RetypedUsage>,
    pub changed_specializations: Vec<ChangedSpecialization>,
    pub changed_attributes: Vec<ChangedAttribute>,
    pub added_relationships: Vec<RelationshipChange>,
    pub removed_relationships: Vec<RelationshipChange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenamedElement {
    pub from: ElementRef,
    pub to: ElementRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MovedElement {
    pub element: ElementRef,
    pub from: Option<ElementRef>,
    pub to: ElementRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetypedUsage {
    pub element: ElementRef,
    pub ty: Option<ElementRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangedSpecialization {
    pub element: ElementRef,
    pub specializes: Vec<ElementRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangedAttribute {
    pub element: ElementRef,
    pub attribute: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationshipChange {
    pub kind: String,
    pub source: ElementRef,
    pub target: ElementRef,
}

pub(crate) fn diff_for_operation(
    operation: &SemanticMutation,
    result: Option<&MutationResult>,
) -> SemanticDiff {
    let mut diff = SemanticDiff::default();
    match operation {
        SemanticMutation::AddPackage { name, .. } => {
            diff.added_elements.push(ElementRef::new(name.clone()));
        }
        SemanticMutation::AddDefinition {
            container, name, ..
        }
        | SemanticMutation::AddUsage {
            container, name, ..
        } => {
            diff.added_elements.push(ElementRef::new(format!(
                "{}.{name}",
                container.qualified_name
            )));
        }
        SemanticMutation::AddRelationship {
            kind,
            source,
            target,
        } => diff.added_relationships.push(RelationshipChange {
            kind: kind.clone(),
            source: source.clone(),
            target: target.clone(),
        }),
        SemanticMutation::RenameDeclaration { element, new_name } => {
            let parent = element
                .qualified_name
                .rsplit_once('.')
                .map(|(parent, _)| parent.to_string());
            let to = parent
                .map(|parent| format!("{parent}.{new_name}"))
                .unwrap_or_else(|| new_name.clone());
            diff.renamed_elements.push(RenamedElement {
                from: element.clone(),
                to: ElementRef::new(to),
            });
        }
        SemanticMutation::UpdateUsageType { element, ty } => {
            diff.retyped_usages.push(RetypedUsage {
                element: element.clone(),
                ty: ty.clone(),
            });
        }
        SemanticMutation::SetExpression { element, .. } => {
            diff.changed_attributes.push(ChangedAttribute {
                element: element.clone(),
                attribute: "expression".to_string(),
            });
        }
        SemanticMutation::UpdateSpecializations {
            element,
            specializes,
        } => diff.changed_specializations.push(ChangedSpecialization {
            element: element.clone(),
            specializes: specializes.clone(),
        }),
        SemanticMutation::MoveDeclaration {
            element,
            destination,
        } => diff.moved_elements.push(MovedElement {
            element: element.clone(),
            from: None,
            to: destination.clone(),
        }),
        SemanticMutation::SetAttribute {
            element, attribute, ..
        } => diff.changed_attributes.push(ChangedAttribute {
            element: element.clone(),
            attribute: attribute.clone(),
        }),
    }

    if let Some(result) = result {
        for declaration in &result.changed_declarations {
            let element = ElementRef::new(declaration.clone());
            if !diff.added_elements.contains(&element)
                && !diff
                    .changed_attributes
                    .iter()
                    .any(|item| item.element == element)
                && !diff
                    .changed_specializations
                    .iter()
                    .any(|item| item.element == element)
                && !diff
                    .retyped_usages
                    .iter()
                    .any(|item| item.element == element)
            {
                diff.changed_attributes.push(ChangedAttribute {
                    element,
                    attribute: "declaration".to_string(),
                });
            }
        }
    }

    diff
}

pub(crate) fn merge_diff(target: &mut SemanticDiff, source: SemanticDiff) {
    target.added_elements.extend(source.added_elements);
    target.removed_elements.extend(source.removed_elements);
    target.renamed_elements.extend(source.renamed_elements);
    target.moved_elements.extend(source.moved_elements);
    target.retyped_usages.extend(source.retyped_usages);
    target
        .changed_specializations
        .extend(source.changed_specializations);
    target.changed_attributes.extend(source.changed_attributes);
    target
        .added_relationships
        .extend(source.added_relationships);
    target
        .removed_relationships
        .extend(source.removed_relationships);
}
