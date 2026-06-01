//! Semantic elaboration phase.

use std::collections::BTreeSet;

use crate::lowering::ir::ResolvedUsage;

#[derive(Debug, Clone, Default)]
pub(crate) struct ReferenceUsageSemantics {
    pub(crate) type_refs: Vec<String>,
    pub(crate) semantic_specializations: Vec<String>,
    pub(crate) subsetted_feature_refs: Vec<String>,
    pub(crate) specialized_feature_refs: Vec<String>,
    pub(crate) redefined_feature_refs: Vec<String>,
    pub(crate) direction: Option<String>,
}

pub(crate) fn reference_usage_semantics(usage: &ResolvedUsage) -> Option<ReferenceUsageSemantics> {
    if usage.construct != "ReferenceUsage" {
        return None;
    }

    let type_refs = usage_all_type_refs(usage);
    let mut semantics = ReferenceUsageSemantics {
        type_refs: type_refs.clone(),
        semantic_specializations: type_refs.clone(),
        ..ReferenceUsageSemantics::default()
    };

    if usage.modifiers.iter().any(|modifier| modifier == "payload") {
        semantics
            .subsetted_feature_refs
            .push("Objects::objects".to_string());
        semantics.redefined_feature_refs.push("payload".to_string());
        semantics.direction = Some("in".to_string());
        return Some(semantics);
    }

    if usage
        .modifiers
        .iter()
        .any(|modifier| modifier == "source-output")
    {
        if semantics.type_refs.is_empty() {
            semantics.type_refs.push("Ports::Port".to_string());
        }
        semantics.semantic_specializations.clear();
        semantics
            .redefined_feature_refs
            .push(usage.declared_name.clone());
        semantics
            .redefined_feature_refs
            .push("Transfers::sourceOutput".to_string());
        return Some(semantics);
    }

    if usage
        .modifiers
        .iter()
        .any(|modifier| modifier == "target-input")
    {
        if semantics.type_refs.is_empty() {
            semantics
                .type_refs
                .push("Occurrences::Occurrence".to_string());
        }
        semantics.semantic_specializations.clear();
        semantics
            .redefined_feature_refs
            .push(usage.declared_name.clone());
        semantics
            .redefined_feature_refs
            .push("Transfers::targetInput".to_string());
        if usage.modifiers.iter().any(|modifier| modifier == "in") {
            semantics.direction = Some("in".to_string());
        }
        return Some(semantics);
    }

    if usage
        .modifiers
        .iter()
        .any(|modifier| modifier == "receiver")
    {
        if semantics.type_refs.is_empty() {
            semantics
                .type_refs
                .push("Occurrences::Occurrence".to_string());
        }
        semantics.semantic_specializations.clear();
        semantics
            .redefined_feature_refs
            .push("receiver".to_string());
        if usage.modifiers.iter().any(|modifier| modifier == "in") {
            semantics.direction = Some("in".to_string());
        }
        return Some(semantics);
    }

    if !semantics.type_refs.is_empty() && all_data_value_like_refs(&semantics.type_refs) {
        semantics
            .subsetted_feature_refs
            .push("Base::dataValues".to_string());
        return Some(semantics);
    }

    if !semantics.type_refs.is_empty() {
        semantics
            .subsetted_feature_refs
            .push("Objects::objects".to_string());
        if usage.modifiers.iter().any(|modifier| modifier == "in") {
            semantics.direction = Some("in".to_string());
        } else if usage.modifiers.iter().any(|modifier| modifier == "out") {
            semantics.direction = Some("out".to_string());
        } else if usage.modifiers.iter().any(|modifier| modifier == "inout") {
            semantics.direction = Some("inout".to_string());
        }
        return Some(semantics);
    }

    None
}

pub(crate) fn usage_all_type_refs(usage: &ResolvedUsage) -> Vec<String> {
    let mut refs = Vec::new();
    if let Some(type_ref) = &usage.type_ref {
        refs.push(type_ref.clone());
    }
    refs.extend(usage.additional_type_refs.clone());
    dedupe_refs(refs)
}

fn all_data_value_like_refs(type_refs: &[String]) -> bool {
    !type_refs.is_empty()
        && type_refs
            .iter()
            .all(|type_ref| is_data_value_like_ref(type_ref))
}

fn is_data_value_like_ref(type_ref: &str) -> bool {
    let tail = type_ref
        .rsplit("::")
        .next()
        .unwrap_or(type_ref)
        .rsplit('.')
        .next()
        .unwrap_or(type_ref);
    matches!(
        tail,
        "Boolean" | "Integer" | "Natural" | "Real" | "Rational" | "String" | "UnlimitedNatural"
    ) || tail.ends_with("Value")
}

pub(crate) struct UsageFamilyDefaults {
    pub(crate) type_ref: &'static str,
    pub(crate) subsetted_feature_refs: &'static [&'static str],
    pub(crate) is_variable: bool,
}

pub(crate) fn usage_family_defaults(usage: &ResolvedUsage) -> Option<UsageFamilyDefaults> {
    match usage.construct.as_str() {
        "ActionUsage" => Some(UsageFamilyDefaults {
            type_ref: "Actions::Action",
            subsetted_feature_refs: action_usage_subsetted_feature_refs(usage),
            is_variable: false,
        }),
        "PerformActionUsage" => Some(UsageFamilyDefaults {
            type_ref: "Actions::Action",
            subsetted_feature_refs: &["Actions::performedActions"],
            is_variable: true,
        }),
        "AcceptActionUsage" => Some(UsageFamilyDefaults {
            type_ref: "Actions::AcceptAction",
            subsetted_feature_refs: &["Actions::acceptSubactions"],
            is_variable: false,
        }),
        "StateUsage" => Some(UsageFamilyDefaults {
            type_ref: "States::StateAction",
            subsetted_feature_refs: &["States::ownedStates"],
            is_variable: false,
        }),
        "ExhibitStateUsage" => Some(UsageFamilyDefaults {
            type_ref: "States::StateAction",
            subsetted_feature_refs: &["States::exhibitedStates"],
            is_variable: true,
        }),
        "SuccessionUsage" => Some(UsageFamilyDefaults {
            type_ref: "Flows::SuccessionFlow",
            subsetted_feature_refs: &["Actions::ownedActions", "Flows::successionFlows"],
            is_variable: false,
        }),
        "FlowUsage" => Some(UsageFamilyDefaults {
            type_ref: "Flows::Flow",
            subsetted_feature_refs: &["Actions::Action::subactions", "Flows::flows"],
            is_variable: false,
        }),
        _ => None,
    }
}

fn action_usage_subsetted_feature_refs(usage: &ResolvedUsage) -> &'static [&'static str] {
    match usage.owner_construct.as_str() {
        "Package" => &["Actions::actions"],
        "ActionDefinition" | "ActionUsage" | "PerformActionUsage" => {
            &["Actions::Action::subactions"]
        }
        _ => &["Parts::Part::ownedActions"],
    }
}

pub(crate) fn dedupe_refs(values: Vec<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    values
        .into_iter()
        .filter(|value| seen.insert(value.clone()))
        .collect()
}
