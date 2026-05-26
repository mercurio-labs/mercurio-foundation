use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::graph::{Element, Graph};
use crate::runtime::Runtime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateMachineModel {
    pub id: String,
    pub label: String,
    pub states: Vec<StateNode>,
    pub transitions: Vec<TransitionNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateNode {
    pub id: String,
    pub label: String,
    pub owner_id: String,
    pub parent_state_id: Option<String>,
    pub is_initial: bool,
    pub is_final: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionNode {
    pub id: String,
    pub owner_id: String,
    pub source: String,
    pub target: String,
    pub trigger: Option<String>,
}

impl StateMachineModel {
    pub fn reachable_state_ids(&self) -> BTreeSet<String> {
        let mut reachable = BTreeSet::new();
        let mut queue = VecDeque::new();
        for state in self.states.iter().filter(|state| state.is_initial) {
            reachable.insert(state.id.clone());
            queue.push_back(state.id.clone());
        }

        while let Some(state_id) = queue.pop_front() {
            for transition in self
                .transitions
                .iter()
                .filter(|transition| transition.source == state_id)
            {
                if reachable.insert(transition.target.clone()) {
                    queue.push_back(transition.target.clone());
                }
            }
        }

        reachable
    }

    pub fn ambiguous_transition_keys(&self) -> Vec<(String, String, usize)> {
        let mut counts = BTreeMap::<(String, String), usize>::new();
        for transition in &self.transitions {
            let trigger = transition
                .trigger
                .clone()
                .unwrap_or_else(|| "<untriggered>".to_string());
            *counts
                .entry((transition.source.clone(), trigger))
                .or_default() += 1;
        }
        counts
            .into_iter()
            .filter_map(|((source, trigger), count)| {
                (count > 1).then_some((source, trigger, count))
            })
            .collect()
    }
}

pub fn project_state_machines(runtime: &Runtime) -> Vec<StateMachineModel> {
    project_state_machines_from_graph(runtime.graph())
}

pub fn project_state_machines_from_graph(graph: &Graph) -> Vec<StateMachineModel> {
    let mut states_by_owner = BTreeMap::<String, Vec<StateNode>>::new();
    let mut transitions_by_owner = BTreeMap::<String, Vec<TransitionNode>>::new();

    for element in graph.elements() {
        if is_state_element(element) {
            let owner = owner_id(element).unwrap_or_else(|| "state_machine.root".to_string());
            states_by_owner
                .entry(owner.clone())
                .or_default()
                .push(StateNode {
                    id: element.element_id.clone(),
                    label: element_label(element),
                    owner_id: owner,
                    parent_state_id: parent_state_id(element),
                    is_initial: bool_property(element, &["is_initial", "initial"])
                        || string_property_any(element, &["purpose", "state_kind", "kind_role"])
                            .is_some_and(|value| value.eq_ignore_ascii_case("initial")),
                    is_final: bool_property(element, &["is_final", "final"])
                        || string_property_any(element, &["purpose", "state_kind", "kind_role"])
                            .is_some_and(|value| value.eq_ignore_ascii_case("final")),
                });
            continue;
        }

        if is_transition_element(element)
            && let (Some(source), Some(target)) = (
                string_property_any(element, &["source", "source_state", "from"]),
                string_property_any(element, &["target", "target_state", "to"]),
            )
        {
            let owner = owner_id(element).unwrap_or_else(|| {
                source
                    .rsplit_once(['.', ':', '/'])
                    .map(|(prefix, _)| prefix.to_string())
                    .unwrap_or_else(|| "state_machine.root".to_string())
            });
            transitions_by_owner
                .entry(owner.clone())
                .or_default()
                .push(TransitionNode {
                    id: element.element_id.clone(),
                    owner_id: owner,
                    source,
                    target,
                    trigger: string_property_any(element, &["trigger", "event", "guard"]),
                });
        }
    }

    let mut owners = states_by_owner.keys().cloned().collect::<BTreeSet<_>>();
    owners.extend(transitions_by_owner.keys().cloned());

    owners
        .into_iter()
        .map(|owner| {
            let mut states = states_by_owner.remove(&owner).unwrap_or_default();
            states.sort_by(|left, right| left.id.cmp(&right.id));
            let mut transitions = transitions_by_owner.remove(&owner).unwrap_or_default();
            transitions.sort_by(|left, right| left.id.cmp(&right.id));
            StateMachineModel {
                label: owner
                    .rsplit(['.', ':', '/'])
                    .find(|part| !part.is_empty())
                    .unwrap_or(&owner)
                    .to_string(),
                id: owner,
                states,
                transitions,
            }
        })
        .collect()
}

fn is_state_element(element: &Element) -> bool {
    let kind = element.kind.to_ascii_lowercase();
    kind.contains("stateusage")
        || kind.contains("stateaction")
        || string_property_any(element, &["type", "definition"])
            .is_some_and(|value| value.contains("States::StateAction"))
}

fn is_transition_element(element: &Element) -> bool {
    let kind = element.kind.to_ascii_lowercase();
    kind.contains("transition")
        || kind.contains("succession")
        || element.element_id.starts_with("transition.")
}

fn owner_id(element: &Element) -> Option<String> {
    string_property_any(
        element,
        &[
            "owner",
            "owning_type",
            "owning_definition",
            "owning_namespace",
        ],
    )
}

fn parent_state_id(element: &Element) -> Option<String> {
    string_property_any(
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

fn element_label(element: &Element) -> String {
    element
        .properties
        .get("declared_name")
        .or_else(|| element.properties.get("name"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| element.element_id.clone())
}

fn string_property_any(element: &Element, keys: &[&str]) -> Option<String> {
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

fn bool_property(element: &Element, keys: &[&str]) -> bool {
    keys.iter().any(|key| {
        element
            .properties
            .get(*key)
            .and_then(Value::as_bool)
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::Value;

    use crate::{KirDocument, KirElement, Runtime};

    use super::*;

    #[test]
    fn projects_flat_state_machine() {
        let runtime = Runtime::from_document(KirDocument {
            metadata: BTreeMap::new(),
            elements: vec![
                state("state.ControllerMode.Off", "ControllerMode", true, false),
                state("state.ControllerMode.On", "ControllerMode", false, false),
                transition(
                    "transition.ControllerMode.start",
                    "ControllerMode",
                    "state.ControllerMode.Off",
                    "state.ControllerMode.On",
                    "start",
                ),
            ],
        })
        .unwrap();

        let machines = project_state_machines(&runtime);

        assert_eq!(machines.len(), 1);
        assert_eq!(machines[0].id, "ControllerMode");
        assert_eq!(machines[0].states.len(), 2);
        assert_eq!(machines[0].transitions.len(), 1);
        assert!(
            machines[0]
                .reachable_state_ids()
                .contains("state.ControllerMode.On")
        );
    }

    #[test]
    fn preserves_nested_state_parent_id() {
        let runtime = Runtime::from_document(KirDocument {
            metadata: BTreeMap::new(),
            elements: vec![
                state("state.Server.Active", "ServerBehavior", true, false),
                nested_state(
                    "state.Server.Active.Waiting",
                    "ServerBehavior",
                    "state.Server.Active",
                    false,
                    false,
                ),
            ],
        })
        .unwrap();

        let machines = project_state_machines(&runtime);
        let nested = machines[0]
            .states
            .iter()
            .find(|state| state.id == "state.Server.Active.Waiting")
            .unwrap();

        assert_eq!(
            nested.parent_state_id.as_deref(),
            Some("state.Server.Active")
        );
    }

    fn state(id: &str, owner: &str, initial: bool, final_state: bool) -> KirElement {
        KirElement {
            id: id.to_string(),
            kind: "StateUsage".to_string(),
            layer: 0,
            properties: BTreeMap::from([
                ("declared_name".to_string(), Value::String(id.to_string())),
                ("owning_type".to_string(), Value::String(owner.to_string())),
                ("is_initial".to_string(), Value::Bool(initial)),
                ("is_final".to_string(), Value::Bool(final_state)),
            ]),
        }
    }

    fn nested_state(
        id: &str,
        owner: &str,
        parent: &str,
        initial: bool,
        final_state: bool,
    ) -> KirElement {
        let mut element = state(id, owner, initial, final_state);
        element.properties.insert(
            "parent_state".to_string(),
            Value::String(parent.to_string()),
        );
        element
    }

    fn transition(id: &str, owner: &str, source: &str, target: &str, trigger: &str) -> KirElement {
        KirElement {
            id: id.to_string(),
            kind: "TransitionUsage".to_string(),
            layer: 0,
            properties: BTreeMap::from([
                ("owning_type".to_string(), Value::String(owner.to_string())),
                ("source".to_string(), Value::String(source.to_string())),
                ("target".to_string(), Value::String(target.to_string())),
                ("trigger".to_string(), Value::String(trigger.to_string())),
            ]),
        }
    }
}
