use serde::{Deserialize, Serialize};

use crate::mutation::{
    ElementRef, SemanticReasoningContext, SemanticRelationshipContext, WorkspaceRevision,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticGoalSpec {
    pub policy: GoalPolicy,
    pub checks: Vec<SemanticGoalCheck>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GoalPolicy {
    All,
    Any,
    ScoreAtLeast(f64),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticGoalCheck {
    ElementExists {
        element: ElementRef,
        kind: Option<String>,
    },
    RelationshipExists {
        source: ElementRef,
        kind: String,
        target: ElementRef,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoalEvaluation {
    pub satisfied: bool,
    pub score: f64,
    pub policy: GoalPolicy,
    pub checked_against: WorkspaceRevision,
    pub results: Vec<GoalCheckEvaluation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoalCheckEvaluation {
    pub check: SemanticGoalCheck,
    pub satisfied: bool,
    pub evidence: Vec<String>,
}

pub fn evaluate_semantic_goal(
    context: &SemanticReasoningContext,
    goal: &SemanticGoalSpec,
) -> GoalEvaluation {
    let results = goal
        .checks
        .iter()
        .cloned()
        .map(|check| evaluate_goal_check(context, check))
        .collect::<Vec<_>>();
    let satisfied_count = results.iter().filter(|result| result.satisfied).count();
    let score = if results.is_empty() {
        1.0
    } else {
        satisfied_count as f64 / results.len() as f64
    };
    let satisfied = match goal.policy {
        GoalPolicy::All => results.iter().all(|result| result.satisfied),
        GoalPolicy::Any => results.iter().any(|result| result.satisfied),
        GoalPolicy::ScoreAtLeast(threshold) => score >= threshold,
    };

    GoalEvaluation {
        satisfied,
        score,
        policy: goal.policy,
        checked_against: context.workspace_revision.clone(),
        results,
    }
}

fn evaluate_goal_check(
    context: &SemanticReasoningContext,
    check: SemanticGoalCheck,
) -> GoalCheckEvaluation {
    match &check {
        SemanticGoalCheck::ElementExists { element, kind } => {
            let mut evidence = Vec::new();
            let satisfied = context.elements.iter().any(|candidate| {
                if candidate.element != *element {
                    return false;
                }
                evidence.push(format!("found element `{}`", element.qualified_name));
                kind.as_ref().is_none_or(|expected_kind| {
                    let direct_kind_matches = candidate.kind.eq_ignore_ascii_case(expected_kind);
                    let keyword_matches = candidate
                        .attributes
                        .get("keyword")
                        .and_then(serde_json::Value::as_str)
                        .is_some_and(|keyword| keyword.eq_ignore_ascii_case(expected_kind));
                    let kir_kind_matches = candidate
                        .attributes
                        .get("kirKind")
                        .and_then(serde_json::Value::as_str)
                        .is_some_and(|kir_kind| kir_kind.eq_ignore_ascii_case(expected_kind));
                    direct_kind_matches || keyword_matches || kir_kind_matches
                })
            });
            if !satisfied {
                evidence.push(format!("missing element `{}`", element.qualified_name));
            }
            GoalCheckEvaluation {
                check,
                satisfied,
                evidence,
            }
        }
        SemanticGoalCheck::RelationshipExists {
            source,
            kind,
            target,
        } => {
            let wanted = SemanticRelationshipContext {
                kind: kind.clone(),
                source: source.clone(),
                target: target.clone(),
            };
            let mut evidence = Vec::new();
            let satisfied = context.relationships.iter().any(|relationship| {
                relationship.source == wanted.source
                    && relationship.target == wanted.target
                    && relationship_kind_matches(&relationship.kind, &wanted.kind)
            });
            if satisfied {
                evidence.push(format!(
                    "found relationship `{}` --{}--> `{}`",
                    source.qualified_name, kind, target.qualified_name
                ));
            } else {
                evidence.push(format!(
                    "missing relationship `{}` --{}--> `{}`",
                    source.qualified_name, kind, target.qualified_name
                ));
            }
            GoalCheckEvaluation {
                check,
                satisfied,
                evidence,
            }
        }
    }
}

fn relationship_kind_matches(actual: &str, expected: &str) -> bool {
    actual.eq_ignore_ascii_case(expected)
        || actual
            .strip_prefix("kir.")
            .is_some_and(|kind| kind.eq_ignore_ascii_case(expected))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutation::{
        SemanticElementContext, SemanticFactContext, SemanticRelationshipContext,
    };
    use std::collections::BTreeMap;

    #[test]
    fn evaluates_required_elements_and_relationships() {
        let context = SemanticReasoningContext {
            metamodel_version: "test".to_string(),
            workspace_revision: WorkspaceRevision::unchecked(),
            focus: Vec::new(),
            elements: vec![
                SemanticElementContext {
                    element: ElementRef::new("HybridVehicle.HybridVehicle"),
                    kind: "definition".to_string(),
                    label: "HybridVehicle".to_string(),
                    owner: Some(ElementRef::new("HybridVehicle")),
                    attributes: BTreeMap::from([(
                        "keyword".to_string(),
                        serde_json::Value::String("part".to_string()),
                    )]),
                },
                SemanticElementContext {
                    element: ElementRef::new("HybridVehicle.ImproveEfficiency"),
                    kind: "definition".to_string(),
                    label: "ImproveEfficiency".to_string(),
                    owner: Some(ElementRef::new("HybridVehicle")),
                    attributes: BTreeMap::from([(
                        "keyword".to_string(),
                        serde_json::Value::String("requirement".to_string()),
                    )]),
                },
            ],
            relationships: vec![SemanticRelationshipContext {
                kind: "satisfy".to_string(),
                source: ElementRef::new("HybridVehicle.HybridVehicle"),
                target: ElementRef::new("HybridVehicle.ImproveEfficiency"),
            }],
            facts: Vec::<SemanticFactContext>::new(),
            affordances: Vec::new(),
            source_files: Vec::new(),
            truncated: false,
        };
        let goal = SemanticGoalSpec {
            policy: GoalPolicy::All,
            checks: vec![
                SemanticGoalCheck::ElementExists {
                    element: ElementRef::new("HybridVehicle.HybridVehicle"),
                    kind: Some("part".to_string()),
                },
                SemanticGoalCheck::RelationshipExists {
                    source: ElementRef::new("HybridVehicle.HybridVehicle"),
                    kind: "satisfy".to_string(),
                    target: ElementRef::new("HybridVehicle.ImproveEfficiency"),
                },
            ],
        };

        let evaluation = evaluate_semantic_goal(&context, &goal);

        assert!(evaluation.satisfied);
        assert_eq!(evaluation.score, 1.0);
        assert_eq!(evaluation.results.len(), 2);
    }
}
