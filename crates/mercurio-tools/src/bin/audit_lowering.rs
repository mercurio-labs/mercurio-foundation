use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use mercurio_core::frontend::lowering::pilot_evidence::PilotLoweringEvidence;
use mercurio_core::frontend::lowering::rules::{
    LoweringAstPattern, LoweringCollectRule, LoweringEmitRule, LoweringPilotSources, LoweringRule,
    LoweringRuleSeed, has_runtime_collect_expression, has_runtime_elaboration_hook,
};
use serde_json::{Value, json};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse()?;
    let constructs = read_json(&args.constructs)?;
    let emission = read_json(&args.emission)?;
    let lowering_rules = args.rules.as_deref().map(read_lowering_rules).transpose()?;

    let construct_metaclasses = construct_metaclasses(&constructs);
    let emission_metaclasses = emission_metaclasses(&emission);
    let emission_properties = emission_properties(&emission);
    let missing_emission = construct_metaclasses
        .difference(&emission_metaclasses)
        .cloned()
        .collect::<Vec<_>>();
    let unused_emission = emission_metaclasses
        .difference(&construct_metaclasses)
        .cloned()
        .collect::<Vec<_>>();

    println!("Lowering audit");
    println!("  constructs: {}", construct_metaclasses.len());
    println!("  emission rules: {}", emission_metaclasses.len());
    println!("  missing emission rules: {}", missing_emission.len());
    println!(
        "  emission rules without construct evidence: {}",
        unused_emission.len()
    );

    if !missing_emission.is_empty() {
        println!();
        println!("Missing emission rules:");
        for metaclass in &missing_emission {
            println!("  {metaclass}");
        }
    }

    if !unused_emission.is_empty() {
        println!();
        println!("Emission rules without construct evidence:");
        for metaclass in &unused_emission {
            println!("  {metaclass}");
        }
    }

    if let Some(lowering_rules) = &lowering_rules {
        let rule_constructs = lowering_rule_constructs(lowering_rules);
        let rule_metaclasses = lowering_rule_metaclasses(lowering_rules);
        let rule_status_counts = lowering_rule_status_counts(lowering_rules);
        let reviewed_rule_count = rule_status_counts.get("reviewed").copied().unwrap_or(0);
        let constructs_missing_rules = construct_metaclasses
            .difference(&rule_metaclasses)
            .cloned()
            .collect::<Vec<_>>();
        let rules_missing_construct = rule_metaclasses
            .difference(&construct_metaclasses)
            .cloned()
            .collect::<Vec<_>>();
        let rules_missing_emission = rule_metaclasses
            .difference(&emission_metaclasses)
            .cloned()
            .collect::<Vec<_>>();
        let rule_id_template_gaps = lowering_rule_id_template_gaps(lowering_rules, &emission);
        let rule_property_gaps = lowering_rule_property_gaps(lowering_rules, &emission_properties);
        let emission_property_gaps =
            emission_property_gaps(lowering_rules, &emission_properties, &construct_metaclasses);
        let collect_expression_count = lowering_collect_expression_count(lowering_rules);
        let unsupported_collect_expressions = unsupported_collect_expressions(lowering_rules);
        let elaboration_rule_count = lowering_elaboration_rule_count(lowering_rules);
        let unimplemented_elaboration_rules = unimplemented_elaboration_rules(lowering_rules);

        println!();
        println!("Declarative lowering rules");
        println!("  rules: {}", lowering_rules.rules.len());
        println!("  constructs covered: {}", rule_constructs.len());
        for (status, count) in &rule_status_counts {
            println!("  {status} rules: {count}");
        }
        println!(
            "  construct mappings without declarative rules: {}",
            constructs_missing_rules.len()
        );
        println!(
            "  rule metaclasses missing construct mappings: {}",
            rules_missing_construct.len()
        );
        println!(
            "  rule metaclasses missing emission rules: {}",
            rules_missing_emission.len()
        );
        println!(
            "  rule id templates different from emission templates: {}",
            rule_id_template_gaps.len()
        );
        println!(
            "  rule properties missing emission properties: {}",
            rule_property_gaps.len()
        );
        println!(
            "  emission properties missing declarative rule properties: {}",
            emission_property_gaps.len()
        );
        println!("  collect expressions: {collect_expression_count}");
        println!(
            "  collect expressions without runtime support: {}",
            unsupported_collect_expressions.len()
        );
        println!("  elaboration rules: {elaboration_rule_count}");
        println!(
            "  elaboration rules without runtime hook: {}",
            unimplemented_elaboration_rules.len()
        );

        if !rules_missing_construct.is_empty() {
            println!();
            println!("Rule metaclasses missing construct mappings:");
            for metaclass in &rules_missing_construct {
                println!("  {metaclass}");
            }
        }

        if !rules_missing_emission.is_empty() {
            println!();
            println!("Rule metaclasses missing emission rules:");
            for metaclass in &rules_missing_emission {
                println!("  {metaclass}");
            }
        }

        if !rule_id_template_gaps.is_empty() {
            println!();
            println!("Rule id templates different from emission templates:");
            for gap in &rule_id_template_gaps {
                println!(
                    "  {}: rule=`{}` emission=`{}`",
                    gap.metaclass, gap.rule_template, gap.emission_template
                );
            }
        }

        if !rule_property_gaps.is_empty() {
            println!();
            println!("Rule properties missing emission properties:");
            for gap in &rule_property_gaps {
                println!("  {}.{}", gap.metaclass, gap.property);
            }
        }

        if !emission_property_gaps.is_empty() && args.verbose_rules {
            println!();
            println!("Emission properties missing declarative rule properties:");
            for gap in &emission_property_gaps {
                println!("  {}.{}", gap.metaclass, gap.property);
            }
        }

        if !unsupported_collect_expressions.is_empty() {
            println!();
            println!("Collect expressions without runtime support:");
            for gap in &unsupported_collect_expressions {
                println!("  {}.{}: {}", gap.construct, gap.slot, gap.expression);
            }
        }

        if !unimplemented_elaboration_rules.is_empty() {
            println!();
            println!("Elaboration rules without runtime hook:");
            for gap in &unimplemented_elaboration_rules {
                println!("  {}: {}", gap.construct, gap.rule_id);
            }
        }

        if !constructs_missing_rules.is_empty() && args.verbose_rules {
            println!();
            println!("Construct mappings without declarative rules:");
            for metaclass in &constructs_missing_rules {
                println!("  {metaclass}");
            }
        }

        if reviewed_rule_count < args.min_reviewed_rules {
            return Err(format!(
                "reviewed lowering rule count {reviewed_rule_count} is below required minimum {}",
                args.min_reviewed_rules
            )
            .into());
        }
    }

    if let Some(evidence_path) = args.evidence.as_deref() {
        let evidence = read_pilot_evidence(evidence_path)?;
        let grammar_returns = grammar_returns(&evidence);
        let ecore_classes = ecore_classes(&evidence);
        let grammar_metaclasses = grammar_returns.values().cloned().collect::<BTreeSet<_>>();
        let grammar_missing_emission = grammar_metaclasses
            .difference(&emission_metaclasses)
            .cloned()
            .collect::<Vec<_>>();
        let evidence_missing_construct = grammar_metaclasses
            .difference(&construct_metaclasses)
            .cloned()
            .collect::<Vec<_>>();
        let ecore_missing_emission = ecore_classes
            .difference(&emission_metaclasses)
            .cloned()
            .collect::<Vec<_>>();

        let ecore_feature_gaps = ecore_feature_gaps(&evidence, &emission_properties);
        let rule_evidence_gaps = lowering_rules
            .as_ref()
            .map(|rules| lowering_rule_evidence_gaps(rules, &evidence))
            .unwrap_or_default();
        let grammar_missing_lowering_rules = lowering_rules
            .as_ref()
            .map(|rules| {
                let rule_metaclasses = lowering_rule_metaclasses(rules);
                grammar_metaclasses
                    .difference(&rule_metaclasses)
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let actual_transform_observation_count = evidence.transform_observations.len();
        println!();
        println!("Pilot evidence");
        println!("  grammar rules: {}", grammar_returns.len());
        println!("  ecore classes: {}", ecore_classes.len());
        println!("  transform observations: {actual_transform_observation_count}");
        println!(
            "  grammar returns missing emission rules: {}",
            grammar_missing_emission.len()
        );
        println!(
            "  grammar returns missing construct mappings: {}",
            evidence_missing_construct.len()
        );
        println!(
            "  ecore classes missing emission rules: {}",
            ecore_missing_emission.len()
        );
        println!(
            "  ecore exact-name features missing emission properties: {}",
            ecore_feature_gaps.len()
        );
        if lowering_rules.is_some() {
            println!(
                "  grammar returns missing declarative lowering rules: {}",
                grammar_missing_lowering_rules.len()
            );
            println!(
                "  lowering rule pilot sources missing evidence: {}",
                rule_evidence_gaps.len()
            );
        }

        if !grammar_missing_emission.is_empty() {
            println!();
            println!("Grammar returns missing emission rules:");
            for metaclass in &grammar_missing_emission {
                println!("  {metaclass}");
            }
        }

        if !evidence_missing_construct.is_empty() {
            println!();
            println!("Grammar returns missing construct mappings:");
            for metaclass in &evidence_missing_construct {
                println!("  {metaclass}");
            }
        }

        if !ecore_missing_emission.is_empty() {
            println!();
            println!("Ecore classes missing emission rules:");
            for metaclass in &ecore_missing_emission {
                println!("  {metaclass}");
            }
        }

        if !ecore_feature_gaps.is_empty() {
            println!();
            println!("Ecore exact-name features missing emission properties:");
            for gap in ecore_feature_gaps.iter().take(50) {
                println!("  {}.{}", gap.metaclass, gap.feature);
            }
            if ecore_feature_gaps.len() > 50 {
                println!("  ... {} more", ecore_feature_gaps.len() - 50);
            }
        }

        if !rule_evidence_gaps.is_empty() {
            println!();
            println!("Lowering rule pilot sources missing evidence:");
            for gap in &rule_evidence_gaps {
                println!("  {}: {}", gap.construct, gap.source);
            }
        }

        if !grammar_missing_lowering_rules.is_empty() && args.verbose_rules {
            println!();
            println!("Grammar returns missing declarative lowering rules:");
            for metaclass in &grammar_missing_lowering_rules {
                println!("  {metaclass}");
            }
        }
    }

    if let Some(output_path) = args.write_rule_draft.as_deref() {
        let draft = generate_lowering_rule_draft(lowering_rules.as_ref(), &constructs, &emission);
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(output_path, serde_json::to_string_pretty(&draft)?)?;
        println!();
        println!(
            "Wrote declarative lowering rule draft: {} rules -> {}",
            draft.rules.len(),
            output_path.display()
        );
    }

    Ok(())
}

struct Args {
    constructs: PathBuf,
    emission: PathBuf,
    rules: Option<PathBuf>,
    verbose_rules: bool,
    write_rule_draft: Option<PathBuf>,
    min_reviewed_rules: usize,
    evidence: Option<PathBuf>,
}

impl Args {
    fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let mut constructs = PathBuf::from(
            "resources/language-profiles/sysml-2.0-pilot-0.57.0/mappings/pilot_constructs.seed.json",
        );
        let mut emission = PathBuf::from(
            "resources/language-profiles/sysml-2.0-pilot-0.57.0/mappings/kir_emission.seed.json",
        );
        let mut rules = Some(PathBuf::from(
            "resources/language-profiles/sysml-2.0-pilot-0.57.0/mappings/lowering_rules.seed.json",
        ));
        let mut verbose_rules = false;
        let mut write_rule_draft = None;
        let mut min_reviewed_rules = 0usize;
        let mut evidence = None;
        let args = std::env::args().skip(1).collect::<Vec<_>>();
        let mut index = 0;

        while index < args.len() {
            match args[index].as_str() {
                "--constructs" => {
                    index += 1;
                    constructs =
                        PathBuf::from(args.get(index).ok_or("missing --constructs value")?);
                }
                "--emission" => {
                    index += 1;
                    emission = PathBuf::from(args.get(index).ok_or("missing --emission value")?);
                }
                "--evidence" => {
                    index += 1;
                    evidence = Some(PathBuf::from(
                        args.get(index).ok_or("missing --evidence value")?,
                    ));
                }
                "--rules" => {
                    index += 1;
                    rules = Some(PathBuf::from(
                        args.get(index).ok_or("missing --rules value")?,
                    ));
                }
                "--no-rules" => {
                    rules = None;
                }
                "--verbose-rules" => {
                    verbose_rules = true;
                }
                "--write-rule-draft" => {
                    index += 1;
                    write_rule_draft = Some(PathBuf::from(
                        args.get(index).ok_or("missing --write-rule-draft value")?,
                    ));
                }
                "--min-reviewed-rules" => {
                    index += 1;
                    min_reviewed_rules = args
                        .get(index)
                        .ok_or("missing --min-reviewed-rules value")?
                        .parse()?;
                }
                "--help" | "-h" => {
                    print_usage();
                    std::process::exit(0);
                }
                unknown => return Err(format!("unknown argument: {unknown}").into()),
            }
            index += 1;
        }

        Ok(Self {
            constructs,
            emission,
            rules,
            verbose_rules,
            write_rule_draft,
            min_reviewed_rules,
            evidence,
        })
    }
}

fn print_usage() {
    println!(
        "Usage: cargo run -p mercurio-tools --bin audit_lowering -- [--constructs PATH] [--emission PATH] [--rules PATH|--no-rules] [--verbose-rules] [--write-rule-draft PATH] [--min-reviewed-rules N] [--evidence PATH]"
    );
}

fn read_json(path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&input)?)
}

fn read_pilot_evidence(path: &Path) -> Result<PilotLoweringEvidence, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&input)?)
}

fn read_lowering_rules(path: &Path) -> Result<LoweringRuleSeed, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&input)?)
}

fn construct_metaclasses(document: &Value) -> BTreeSet<String> {
    document
        .get("constructs")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("metaclass"))
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

fn emission_metaclasses(document: &Value) -> BTreeSet<String> {
    document
        .get("metaclasses")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|metaclasses| metaclasses.keys())
        .cloned()
        .collect()
}

fn emission_properties(document: &Value) -> BTreeMap<String, BTreeSet<String>> {
    document
        .get("metaclasses")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|metaclasses| metaclasses.iter())
        .map(|(metaclass, rule)| {
            let properties = rule
                .get("emit")
                .and_then(|emit| emit.get("properties"))
                .and_then(Value::as_object)
                .into_iter()
                .flat_map(|properties| properties.keys())
                .cloned()
                .collect::<BTreeSet<_>>();
            (metaclass.clone(), properties)
        })
        .collect()
}

fn construct_entries(document: &Value) -> Vec<(String, String)> {
    document
        .get("constructs")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let construct = entry.get("construct")?.as_str()?.to_string();
            let metaclass = entry.get("metaclass")?.as_str()?.to_string();
            Some((construct, metaclass))
        })
        .collect()
}

fn construct_keywords(document: &Value) -> BTreeMap<String, String> {
    let mut keywords = BTreeMap::new();
    for registry_name in ["definitions", "usages"] {
        let Some(registry) = document
            .get("keyword_registry")
            .and_then(|registry| registry.get(registry_name))
            .and_then(Value::as_object)
        else {
            continue;
        };
        for (keyword, construct) in registry {
            if let Some(construct) = construct.as_str() {
                keywords
                    .entry(construct.to_string())
                    .or_insert_with(|| keyword.clone());
            }
        }
    }
    keywords
}

fn emission_id_template(document: &Value, metaclass: &str) -> Option<String> {
    document
        .get("metaclasses")?
        .get(metaclass)?
        .get("id_template")?
        .as_str()
        .map(str::to_string)
}

fn generate_lowering_rule_draft(
    existing: Option<&LoweringRuleSeed>,
    constructs: &Value,
    emission: &Value,
) -> LoweringRuleSeed {
    let mut draft = existing.cloned().unwrap_or_else(|| LoweringRuleSeed {
        schema_version: 1,
        source: BTreeMap::new(),
        rules: Vec::new(),
    });
    draft.source.insert(
        "kind".to_string(),
        json!("mercurio-generated-lowering-rule-draft"),
    );
    draft.source.insert(
        "note".to_string(),
        json!("Generated by audit_lowering --write-rule-draft from construct and emission seeds. Review before promoting."),
    );

    let mut seen_metaclasses = lowering_rule_metaclasses(&draft);
    let keywords = construct_keywords(constructs);
    let emission_properties = emission_properties(emission);

    for (construct, metaclass) in construct_entries(constructs) {
        if seen_metaclasses.contains(&metaclass) {
            continue;
        }
        let Some(properties) = emission_properties.get(&metaclass) else {
            continue;
        };
        let Some(id_template) = emission_id_template(emission, &metaclass) else {
            continue;
        };
        draft.rules.push(generated_lowering_rule(
            &construct,
            &metaclass,
            keywords.get(&construct).cloned(),
            id_template,
            properties,
        ));
        seen_metaclasses.insert(metaclass);
    }

    draft.rules.sort_by(|left, right| {
        left.metaclass
            .cmp(&right.metaclass)
            .then_with(|| left.construct.cmp(&right.construct))
    });
    draft
}

fn generated_lowering_rule(
    construct: &str,
    metaclass: &str,
    keyword: Option<String>,
    id_template: String,
    properties: &BTreeSet<String>,
) -> LoweringRule {
    let element = infer_collect_element(construct, metaclass);
    LoweringRule {
        construct: construct.to_string(),
        metaclass: metaclass.to_string(),
        ast: LoweringAstPattern {
            node: infer_ast_node(&element).to_string(),
            keyword,
        },
        status: Some("generated-draft".to_string()),
        collect: LoweringCollectRule {
            element,
            name: "$ast.name".to_string(),
            owner: "$scope.owner".to_string(),
            fields: inferred_collect_fields(construct),
        },
        elaborate: Vec::new(),
        emit: LoweringEmitRule {
            id_template,
            properties: properties
                .iter()
                .map(|property| (property.clone(), lowering_property_value(property)))
                .collect(),
        },
        pilot_sources: LoweringPilotSources {
            grammar_rules: vec![construct.to_string()],
            ecore_class: Some(metaclass.to_string()),
            transform_observations: Vec::new(),
        },
    }
}

fn lowering_property_value(property: &str) -> String {
    match property {
        "declared_name" => "$declared_name",
        "name" => "$name",
        "owner" => "$owner_id",
        "type" => "$type_ref",
        "featuring_type" => "$featuring_type_ref",
        "direction" => "$direction",
        "members" | "member_ids" => "$member_ids",
        "features" | "owned_feature_ids" => "$owned_feature_ids",
        "specializes" => "$specializes_refs",
        "specialized_features" => "$specialized_feature_refs",
        "subsetted_features" => "$subsetted_feature_refs",
        "redefined_features" => "$redefined_feature_refs",
        "metatype" => "$metatype_ref",
        "is_abstract" => "$is_abstract",
        "is_derived" => "$is_derived",
        "is_end" => "$is_end",
        "is_ordered" => "$is_ordered",
        "is_unique" => "$is_unique",
        "is_variable" => "$is_variable",
        other => return format!("${other}"),
    }
    .to_string()
}

fn infer_collect_element(construct: &str, metaclass: &str) -> String {
    if construct == "Package" || metaclass.ends_with("::Package") {
        "package".to_string()
    } else if construct.contains("Import") || metaclass.contains("Import") {
        "import".to_string()
    } else if construct.ends_with("Definition")
        || construct.ends_with("Classifier")
        || metaclass.ends_with("Definition")
    {
        "definition".to_string()
    } else {
        "usage".to_string()
    }
}

fn infer_ast_node(element: &str) -> &'static str {
    match element {
        "package" => "PackageDecl",
        "import" => "ImportDecl",
        "definition" => "GenericDefinitionDecl",
        _ => "GenericUsageDecl",
    }
}

fn inferred_collect_fields(construct: &str) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    if construct.ends_with("Definition") {
        fields.insert(
            "is_abstract".to_string(),
            "$ast.modifiers contains abstract".to_string(),
        );
        fields.insert("members".to_string(), "$ast.members[usage]".to_string());
        fields.insert(
            "specializes".to_string(),
            "$ast.specializes or semantic_default".to_string(),
        );
    } else if construct.ends_with("Usage") {
        fields.insert(
            "members".to_string(),
            "$ast.body_members[usage]".to_string(),
        );
        fields.insert(
            "specializes".to_string(),
            "$ast.specializes or semantic_default".to_string(),
        );
        fields.insert("type".to_string(), "$ast.ty".to_string());
    }
    fields
}

fn grammar_returns(document: &PilotLoweringEvidence) -> BTreeMap<String, String> {
    document
        .grammar_rules
        .iter()
        .map(|rule| (rule.rule.clone(), rule.returns.clone()))
        .collect()
}

fn ecore_classes(document: &PilotLoweringEvidence) -> BTreeSet<String> {
    document
        .ecore_classes
        .iter()
        .map(|class| format!("{}::{}", class.package, class.name))
        .collect()
}

fn lowering_rule_constructs(document: &LoweringRuleSeed) -> BTreeSet<String> {
    document
        .rules
        .iter()
        .map(|rule| rule.construct.clone())
        .collect()
}

fn lowering_rule_metaclasses(document: &LoweringRuleSeed) -> BTreeSet<String> {
    document
        .rules
        .iter()
        .map(|rule| rule.metaclass.clone())
        .collect()
}

fn lowering_rule_status_counts(document: &LoweringRuleSeed) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for rule in &document.rules {
        let status = rule.status.as_deref().unwrap_or("unspecified").to_string();
        *counts.entry(status).or_insert(0) += 1;
    }
    counts
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RulePropertyGap {
    metaclass: String,
    property: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CollectExpressionGap {
    construct: String,
    slot: String,
    expression: String,
}

fn lowering_collect_expression_count(rules: &LoweringRuleSeed) -> usize {
    rules
        .rules
        .iter()
        .map(|rule| 3 + rule.collect.fields.len())
        .sum()
}

fn unsupported_collect_expressions(rules: &LoweringRuleSeed) -> Vec<CollectExpressionGap> {
    let mut gaps = Vec::new();
    for rule in &rules.rules {
        for (slot, expression) in collect_rule_expressions(rule) {
            if !has_runtime_collect_expression(&expression) {
                gaps.push(CollectExpressionGap {
                    construct: rule.construct.clone(),
                    slot,
                    expression,
                });
            }
        }
    }
    gaps.sort();
    gaps
}

fn collect_rule_expressions(rule: &LoweringRule) -> Vec<(String, String)> {
    let mut expressions = vec![
        ("element".to_string(), rule.collect.element.clone()),
        ("name".to_string(), rule.collect.name.clone()),
        ("owner".to_string(), rule.collect.owner.clone()),
    ];
    expressions.extend(
        rule.collect
            .fields
            .iter()
            .map(|(field, expression)| (format!("fields.{field}"), expression.clone())),
    );
    expressions
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ElaborationHookGap {
    construct: String,
    rule_id: String,
}

fn lowering_elaboration_rule_count(rules: &LoweringRuleSeed) -> usize {
    rules.rules.iter().map(|rule| rule.elaborate.len()).sum()
}

fn unimplemented_elaboration_rules(rules: &LoweringRuleSeed) -> Vec<ElaborationHookGap> {
    let mut gaps = Vec::new();
    for rule in &rules.rules {
        for step in &rule.elaborate {
            if !has_runtime_elaboration_hook(&step.id) {
                gaps.push(ElaborationHookGap {
                    construct: rule.construct.clone(),
                    rule_id: step.id.clone(),
                });
            }
        }
    }
    gaps.sort();
    gaps
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RuleIdTemplateGap {
    metaclass: String,
    rule_template: String,
    emission_template: String,
}

fn lowering_rule_id_template_gaps(
    rules: &LoweringRuleSeed,
    emission: &Value,
) -> Vec<RuleIdTemplateGap> {
    let mut gaps = Vec::new();
    for rule in &rules.rules {
        let Some(emission_template) = emission_id_template(emission, &rule.metaclass) else {
            continue;
        };
        if rule.emit.id_template != emission_template {
            gaps.push(RuleIdTemplateGap {
                metaclass: rule.metaclass.clone(),
                rule_template: rule.emit.id_template.clone(),
                emission_template,
            });
        }
    }
    gaps.sort();
    gaps
}

fn lowering_rule_property_gaps(
    rules: &LoweringRuleSeed,
    emission_properties: &BTreeMap<String, BTreeSet<String>>,
) -> Vec<RulePropertyGap> {
    let mut gaps = Vec::new();
    for rule in &rules.rules {
        let Some(properties) = emission_properties.get(&rule.metaclass) else {
            continue;
        };
        for property in rule.emit.properties.keys() {
            if !properties.contains(property) {
                gaps.push(RulePropertyGap {
                    metaclass: rule.metaclass.clone(),
                    property: property.clone(),
                });
            }
        }
    }
    gaps.sort();
    gaps
}

fn emission_property_gaps(
    rules: &LoweringRuleSeed,
    emission_properties: &BTreeMap<String, BTreeSet<String>>,
    construct_metaclasses: &BTreeSet<String>,
) -> Vec<RulePropertyGap> {
    let rule_properties = rules
        .rules
        .iter()
        .map(|rule| {
            (
                rule.metaclass.clone(),
                rule.emit
                    .properties
                    .keys()
                    .cloned()
                    .collect::<BTreeSet<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut gaps = Vec::new();
    for metaclass in construct_metaclasses {
        let Some(emitted_properties) = emission_properties.get(metaclass) else {
            continue;
        };
        let Some(rule_properties) = rule_properties.get(metaclass) else {
            continue;
        };
        for property in emitted_properties {
            if !rule_properties.contains(property) {
                gaps.push(RulePropertyGap {
                    metaclass: metaclass.clone(),
                    property: property.clone(),
                });
            }
        }
    }
    gaps.sort();
    gaps
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RuleEvidenceGap {
    construct: String,
    source: String,
}

fn lowering_rule_evidence_gaps(
    rules: &LoweringRuleSeed,
    evidence: &PilotLoweringEvidence,
) -> Vec<RuleEvidenceGap> {
    let grammar_rules = evidence
        .grammar_rules
        .iter()
        .map(|rule| rule.rule.clone())
        .collect::<BTreeSet<_>>();
    let ecore_classes = ecore_classes(evidence);
    let transform_observations = evidence
        .transform_observations
        .iter()
        .map(|observation| observation.construct.clone())
        .collect::<BTreeSet<_>>();

    let mut gaps = Vec::new();
    for rule in &rules.rules {
        for grammar_rule in &rule.pilot_sources.grammar_rules {
            if !grammar_rules.contains(grammar_rule) {
                gaps.push(RuleEvidenceGap {
                    construct: rule.construct.clone(),
                    source: format!("grammar:{grammar_rule}"),
                });
            }
        }
        if let Some(ecore_class) = &rule.pilot_sources.ecore_class
            && !ecore_classes.contains(ecore_class)
        {
            gaps.push(RuleEvidenceGap {
                construct: rule.construct.clone(),
                source: format!("ecore:{ecore_class}"),
            });
        }
        for observation in &rule.pilot_sources.transform_observations {
            if !transform_observations.contains(observation) {
                gaps.push(RuleEvidenceGap {
                    construct: rule.construct.clone(),
                    source: format!("transform:{observation}"),
                });
            }
        }
    }

    gaps.sort();
    gaps
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct EcoreFeatureGap {
    metaclass: String,
    feature: String,
}

fn ecore_feature_gaps(
    evidence: &PilotLoweringEvidence,
    emission_properties: &BTreeMap<String, BTreeSet<String>>,
) -> Vec<EcoreFeatureGap> {
    let mut gaps = Vec::new();

    for class in &evidence.ecore_classes {
        let metaclass = format!("{}::{}", class.package, class.name);
        let Some(properties) = emission_properties.get(&metaclass) else {
            continue;
        };

        for feature in &class.structural_features {
            if feature.derived || feature.transient || feature.volatile {
                continue;
            }
            if properties.contains(&feature.name) {
                continue;
            }
            gaps.push(EcoreFeatureGap {
                metaclass: metaclass.clone(),
                feature: feature.name.clone(),
            });
        }
    }

    gaps.sort();
    gaps
}
