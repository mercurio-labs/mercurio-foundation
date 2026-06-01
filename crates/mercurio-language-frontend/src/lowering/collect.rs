//! AST collection phase.

use std::collections::{BTreeMap, BTreeSet};

use mercurio_language_contracts::ast::{
    AliasDecl, Declaration, Expr, GenericDefinitionDecl, GenericUsageDecl, ImportDecl,
    MultiplicityRange, PackageDecl, PartDefinitionDecl, PartUsageDecl, QualifiedName, SourceSpan,
    SysmlModule,
};
use mercurio_language_contracts::diagnostics::Diagnostic;

use crate::lowering::elaborate::should_annotate_connection_end_direction;
use crate::lowering::emit::MappingBundle;
use crate::lowering::ir::ResolvedPackage;

#[derive(Debug, Clone, Default)]
pub(crate) struct CollectedModule {
    pub(crate) packages: Vec<ResolvedPackage>,
    pub(crate) imports: Vec<CollectedImport>,
    pub(crate) definitions: Vec<CollectedDefinition>,
    pub(crate) usages: Vec<CollectedUsage>,
    pub(crate) aliases: Vec<CollectedAlias>,
}

#[derive(Debug, Clone)]
pub(crate) struct CollectedImport {
    pub(crate) owner_package_qualified_name: Option<String>,
    pub(crate) decl: ImportDecl,
}

#[derive(Debug, Clone)]
pub(crate) struct CollectedDefinition {
    pub(crate) construct: String,
    pub(crate) qualified_name: String,
    pub(crate) declared_name: String,
    pub(crate) is_abstract: bool,
    pub(crate) specializes: Vec<QualifiedName>,
    pub(crate) members: Vec<CollectedUsage>,
    pub(crate) docs: Vec<String>,
    pub(crate) span: SourceSpan,
}

#[derive(Debug, Clone)]
pub(crate) struct CollectedUsage {
    pub(crate) construct: String,
    pub(crate) owner_construct: String,
    pub(crate) owner_qualified_name: String,
    pub(crate) qualified_name: String,
    pub(crate) declared_name: String,
    pub(crate) is_implicit_name: bool,
    pub(crate) ty: Option<QualifiedName>,
    pub(crate) additional_types: Vec<QualifiedName>,
    pub(crate) reference_target: Option<QualifiedName>,
    pub(crate) allocation_source: Option<QualifiedName>,
    pub(crate) allocation_target: Option<QualifiedName>,
    pub(crate) metadata_properties: BTreeMap<String, String>,
    pub(crate) multiplicity: Option<MultiplicityRange>,
    pub(crate) expression: Option<Expr>,
    pub(crate) specializes: Vec<QualifiedName>,
    pub(crate) subsets: Vec<QualifiedName>,
    pub(crate) redefines: Vec<QualifiedName>,
    pub(crate) members: Vec<CollectedUsage>,
    pub(crate) modifiers: Vec<String>,
    pub(crate) docs: Vec<String>,
    pub(crate) span: SourceSpan,
}

#[derive(Debug, Clone)]
pub(crate) struct CollectedAlias {
    pub(crate) qualified_name: String,
    pub(crate) declared_name: String,
    pub(crate) target: QualifiedName,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ImportAliases {
    pub(crate) value_aliases: BTreeMap<String, String>,
    pub(crate) namespace_aliases: BTreeMap<String, QualifiedName>,
    pub(crate) ambiguous_value_aliases: BTreeSet<String>,
    pub(crate) ambiguous_namespace_aliases: BTreeSet<String>,
}

pub(crate) fn collect_module(
    module: &SysmlModule,
    mappings: &MappingBundle,
) -> Result<CollectedModule, Diagnostic> {
    let mut packages = Vec::new();
    let mut imports = Vec::new();
    let mut definitions = Vec::new();
    let mut usages = Vec::new();
    let mut aliases = Vec::new();

    let root_members = if !module.members.is_empty() {
        module.members.clone()
    } else if let Some(package) = &module.package {
        vec![Declaration::Package(package.clone())]
    } else {
        Vec::new()
    };

    collect_declarations(
        &root_members,
        &[],
        None,
        &mut packages,
        &mut imports,
        &mut definitions,
        &mut usages,
        &mut aliases,
        mappings,
    )?;
    collect_nested_aliases(&root_members, &[], None, &mut aliases);

    Ok(CollectedModule {
        packages,
        imports,
        definitions,
        usages,
        aliases,
    })
}

pub(crate) fn collect_modules(
    modules: &[SysmlModule],
    mappings: &MappingBundle,
) -> Result<CollectedModule, Diagnostic> {
    let mut collected = CollectedModule::default();

    for module in modules {
        let module = collect_module(module, mappings)?;
        collected.packages.extend(module.packages);
        collected.imports.extend(module.imports);
        collected.definitions.extend(module.definitions);
        collected.usages.extend(module.usages);
        collected.aliases.extend(module.aliases);
    }

    Ok(collected)
}

#[allow(clippy::too_many_arguments)]
fn collect_declarations(
    declarations: &[Declaration],
    owner_package_segments: &[String],
    owner_package_qualified_name: Option<&str>,
    packages: &mut Vec<ResolvedPackage>,
    imports: &mut Vec<CollectedImport>,
    definitions: &mut Vec<CollectedDefinition>,
    usages: &mut Vec<CollectedUsage>,
    aliases: &mut Vec<CollectedAlias>,
    mappings: &MappingBundle,
) -> Result<(), Diagnostic> {
    for declaration in declarations {
        match declaration {
            Declaration::Package(package) => collect_package(
                package,
                owner_package_segments,
                packages,
                imports,
                definitions,
                usages,
                aliases,
                mappings,
            )?,
            Declaration::Import(import_decl) => imports.push(CollectedImport {
                owner_package_qualified_name: owner_package_qualified_name.map(str::to_string),
                decl: import_decl.clone(),
            }),
            Declaration::PartDefinition(definition) => {
                let qualified_segments =
                    qualify_segments(owner_package_segments, &[definition.name.clone()]);
                definitions.push(collect_part_definition(
                    definition,
                    owner_package_segments,
                    mappings,
                )?);
                collect_nested_owned_definitions(
                    &definition.members,
                    &qualified_segments,
                    definitions,
                    mappings,
                )?;
                collect_nested_member_imports(
                    &definition.members,
                    &qualified_segments.join("."),
                    imports,
                );
                collect_nested_owned_packages(
                    &definition.members,
                    &qualified_segments,
                    packages,
                    imports,
                    definitions,
                    usages,
                    aliases,
                    mappings,
                )?;
            }
            Declaration::GenericDefinition(definition) => {
                let qualified_segments =
                    qualify_segments(owner_package_segments, &[definition.name.clone()]);
                definitions.push(collect_generic_definition(
                    definition,
                    owner_package_segments,
                    mappings,
                )?);
                collect_nested_owned_definitions(
                    &definition.members,
                    &qualified_segments,
                    definitions,
                    mappings,
                )?;
                collect_nested_member_imports(
                    &definition.members,
                    &qualified_segments.join("."),
                    imports,
                );
                collect_nested_owned_packages(
                    &definition.members,
                    &qualified_segments,
                    packages,
                    imports,
                    definitions,
                    usages,
                    aliases,
                    mappings,
                )?;
            }
            Declaration::PartUsage(usage) => {
                let owner = owner_package_qualified_name.unwrap_or("root");
                usages.push(collect_part_usage(usage, owner, "Package", mappings));
                let qualified_name = usage_qualified_name(owner, &usage.name);
                collect_nested_member_imports(&usage.body_members, &qualified_name, imports);
            }
            Declaration::GenericUsage(usage) => {
                let owner = owner_package_qualified_name.unwrap_or("root");
                usages.push(collect_generic_usage(usage, owner, "Package", mappings));
                let qualified_name = usage_qualified_name(owner, &usage.name);
                collect_nested_member_imports(&usage.body_members, &qualified_name, imports);
            }
            Declaration::Alias(alias) => aliases.push(collect_alias(alias, owner_package_segments)),
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn collect_nested_owned_packages(
    declarations: &[Declaration],
    owner_package_segments: &[String],
    packages: &mut Vec<ResolvedPackage>,
    imports: &mut Vec<CollectedImport>,
    definitions: &mut Vec<CollectedDefinition>,
    usages: &mut Vec<CollectedUsage>,
    aliases: &mut Vec<CollectedAlias>,
    mappings: &MappingBundle,
) -> Result<(), Diagnostic> {
    for declaration in declarations {
        if let Declaration::Package(package) = declaration {
            collect_package(
                package,
                owner_package_segments,
                packages,
                imports,
                definitions,
                usages,
                aliases,
                mappings,
            )?;
        }
    }

    Ok(())
}

fn collect_nested_owned_definitions(
    declarations: &[Declaration],
    owner_package_segments: &[String],
    definitions: &mut Vec<CollectedDefinition>,
    mappings: &MappingBundle,
) -> Result<(), Diagnostic> {
    for declaration in declarations {
        match declaration {
            Declaration::PartDefinition(definition) => {
                definitions.push(collect_part_definition(
                    definition,
                    owner_package_segments,
                    mappings,
                )?);
                collect_nested_owned_definitions(
                    &definition.members,
                    &qualify_segments(owner_package_segments, &[definition.name.clone()]),
                    definitions,
                    mappings,
                )?;
            }
            Declaration::GenericDefinition(definition) => {
                definitions.push(collect_generic_definition(
                    definition,
                    owner_package_segments,
                    mappings,
                )?);
                collect_nested_owned_definitions(
                    &definition.members,
                    &qualify_segments(owner_package_segments, &[definition.name.clone()]),
                    definitions,
                    mappings,
                )?;
            }
            Declaration::Package(_) => {}
            _ => {}
        }
    }

    Ok(())
}

fn collect_nested_member_imports(
    declarations: &[Declaration],
    owner_qualified_name: &str,
    imports: &mut Vec<CollectedImport>,
) {
    for declaration in declarations {
        match declaration {
            Declaration::Import(import_decl) => imports.push(CollectedImport {
                owner_package_qualified_name: Some(owner_qualified_name.to_string()),
                decl: import_decl.clone(),
            }),
            Declaration::PartUsage(usage) => {
                let qualified_name = usage_qualified_name(owner_qualified_name, &usage.name);
                collect_nested_member_imports(&usage.body_members, &qualified_name, imports);
            }
            Declaration::GenericUsage(usage) => {
                let qualified_name = usage_qualified_name(owner_qualified_name, &usage.name);
                collect_nested_member_imports(&usage.body_members, &qualified_name, imports);
            }
            Declaration::PartDefinition(definition) => {
                let qualified_name = usage_qualified_name(owner_qualified_name, &definition.name);
                collect_nested_member_imports(&definition.members, &qualified_name, imports);
            }
            Declaration::GenericDefinition(definition) => {
                let qualified_name = usage_qualified_name(owner_qualified_name, &definition.name);
                collect_nested_member_imports(&definition.members, &qualified_name, imports);
            }
            Declaration::Package(package) => {
                let qualified_name =
                    usage_qualified_name(owner_qualified_name, &package.name.as_dot_string());
                collect_nested_member_imports(&package.members, &qualified_name, imports);
            }
            Declaration::Alias(_) => {}
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_package(
    package: &PackageDecl,
    owner_package_segments: &[String],
    packages: &mut Vec<ResolvedPackage>,
    imports: &mut Vec<CollectedImport>,
    definitions: &mut Vec<CollectedDefinition>,
    usages: &mut Vec<CollectedUsage>,
    aliases: &mut Vec<CollectedAlias>,
    mappings: &MappingBundle,
) -> Result<(), Diagnostic> {
    let package_segments = qualify_segments(owner_package_segments, &package.name.segments);
    let qualified_name = package_segments.join(".");

    packages.push(ResolvedPackage {
        owner_package_qualified_name: (!owner_package_segments.is_empty())
            .then(|| owner_package_segments.join(".")),
        qualified_name: qualified_name.clone(),
        declared_name: package
            .name
            .segments
            .last()
            .cloned()
            .unwrap_or_else(|| qualified_name.clone()),
        docs: package.docs.clone(),
        span: package.span.clone(),
    });

    collect_declarations(
        &package.members,
        &package_segments,
        Some(&qualified_name),
        packages,
        imports,
        definitions,
        usages,
        aliases,
        mappings,
    )
}

fn collect_part_definition(
    definition: &PartDefinitionDecl,
    owner_package_segments: &[String],
    mappings: &MappingBundle,
) -> Result<CollectedDefinition, Diagnostic> {
    let qualified_name = qualify_name(owner_package_segments, &definition.name);
    let members = definition
        .members
        .iter()
        .filter_map(|member| match member {
            Declaration::PartUsage(usage) => Some(collect_part_usage(
                usage,
                &qualified_name,
                "PartDefinition",
                mappings,
            )),
            Declaration::GenericUsage(usage) => Some(collect_generic_usage(
                usage,
                &qualified_name,
                "PartDefinition",
                mappings,
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    let specializes = definition_specializations_with_default(
        "PartDefinition",
        &definition.specializes,
        mappings,
    );

    Ok(CollectedDefinition {
        construct: "PartDefinition".to_string(),
        qualified_name,
        declared_name: definition.name.clone(),
        is_abstract: definition
            .modifiers
            .iter()
            .any(|modifier| modifier == "abstract"),
        specializes,
        members,
        docs: definition.docs.clone(),
        span: definition.span.clone(),
    })
}

fn collect_generic_definition(
    definition: &GenericDefinitionDecl,
    owner_package_segments: &[String],
    mappings: &MappingBundle,
) -> Result<CollectedDefinition, Diagnostic> {
    let qualified_name = qualify_name(owner_package_segments, &definition.name);
    let construct = mappings.definition_construct_for(&definition.keyword);
    let mut members = definition
        .members
        .iter()
        .filter_map(|member| match member {
            Declaration::PartUsage(usage) => Some(collect_part_usage(
                usage,
                &qualified_name,
                &construct,
                mappings,
            )),
            Declaration::GenericUsage(usage) => Some(collect_generic_usage(
                usage,
                &qualified_name,
                &construct,
                mappings,
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    annotate_connection_definition_members(&construct, &mut members, mappings);
    let specializes =
        definition_specializations_with_default(&construct, &definition.specializes, mappings);

    Ok(CollectedDefinition {
        construct,
        qualified_name,
        declared_name: definition.name.clone(),
        is_abstract: definition
            .modifiers
            .iter()
            .any(|modifier| modifier == "abstract"),
        specializes,
        members,
        docs: definition.docs.clone(),
        span: definition.span.clone(),
    })
}

fn definition_specializations_with_default(
    construct: &str,
    explicit: &[QualifiedName],
    mappings: &MappingBundle,
) -> Vec<QualifiedName> {
    if !explicit.is_empty() {
        return explicit.to_vec();
    }

    let zero_span = SourceSpan {
        start_line: 0,
        start_col: 0,
        end_line: 0,
        end_col: 0,
    };
    let mut specializations = Vec::new();
    for semantic_specialization in mappings.semantic_specializations_for_definition(construct) {
        specializations.push(QualifiedName {
            segments: semantic_specialization
                .split("::")
                .map(str::to_string)
                .collect(),
            span: zero_span.clone(),
        });
    }
    specializations
}

fn collect_part_usage(
    usage: &PartUsageDecl,
    owner_qualified_name: &str,
    owner_construct: &str,
    mappings: &MappingBundle,
) -> CollectedUsage {
    let qualified_name = usage_qualified_name(owner_qualified_name, &usage.name);
    let members = usage
        .body_members
        .iter()
        .filter_map(|member| match member {
            Declaration::PartUsage(usage) => Some(collect_part_usage(
                usage,
                &qualified_name,
                "PartUsage",
                mappings,
            )),
            Declaration::GenericUsage(usage) => Some(collect_generic_usage(
                usage,
                &qualified_name,
                "PartUsage",
                mappings,
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    CollectedUsage {
        construct: "PartUsage".to_string(),
        owner_construct: owner_construct.to_string(),
        owner_qualified_name: owner_qualified_name.to_string(),
        qualified_name,
        declared_name: usage.name.clone(),
        is_implicit_name: usage.is_implicit_name,
        ty: usage.ty.clone(),
        additional_types: usage.additional_types.clone(),
        reference_target: None,
        allocation_source: None,
        allocation_target: None,
        metadata_properties: BTreeMap::new(),
        multiplicity: usage.multiplicity.clone(),
        expression: usage.expression.clone(),
        specializes: usage.specializes.clone(),
        subsets: usage.subsets.clone(),
        redefines: usage.redefines.clone(),
        members,
        modifiers: usage.modifiers.clone(),
        docs: usage.docs.clone(),
        span: usage.span.clone(),
    }
}

fn collect_generic_usage(
    usage: &GenericUsageDecl,
    owner_qualified_name: &str,
    owner_construct: &str,
    mappings: &MappingBundle,
) -> CollectedUsage {
    let construct = mappings.usage_construct_for(&usage.keyword);
    let qualified_name = usage_qualified_name(owner_qualified_name, &usage.name);
    let members = usage
        .body_members
        .iter()
        .filter_map(|member| match member {
            Declaration::PartUsage(usage) => Some(collect_part_usage(
                usage,
                &qualified_name,
                &construct,
                mappings,
            )),
            Declaration::GenericUsage(usage) => Some(collect_generic_usage(
                usage,
                &qualified_name,
                &construct,
                mappings,
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    CollectedUsage {
        construct,
        owner_construct: owner_construct.to_string(),
        owner_qualified_name: owner_qualified_name.to_string(),
        qualified_name,
        declared_name: usage.name.clone(),
        is_implicit_name: usage.is_implicit_name,
        ty: usage.ty.clone(),
        additional_types: usage.additional_types.clone(),
        reference_target: usage.reference_target.clone(),
        allocation_source: usage.allocation_source.clone(),
        allocation_target: usage.allocation_target.clone(),
        metadata_properties: usage.metadata_properties.clone(),
        multiplicity: usage.multiplicity.clone(),
        expression: usage.expression.clone(),
        specializes: usage.specializes.clone(),
        subsets: usage.subsets.clone(),
        redefines: usage.redefines.clone(),
        members,
        modifiers: usage.modifiers.clone(),
        docs: usage.docs.clone(),
        span: usage.span.clone(),
    }
}

fn annotate_connection_definition_members(
    definition_construct: &str,
    members: &mut [CollectedUsage],
    mappings: &MappingBundle,
) {
    if !should_annotate_connection_end_direction(mappings, definition_construct) {
        return;
    }

    let mut end_index = 0usize;
    for member in members {
        if member.construct == "PartUsage"
            && member.modifiers.iter().any(|modifier| modifier == "end")
        {
            let directional_modifier = if end_index == 0 {
                "end-source"
            } else {
                "end-target"
            };
            member.modifiers.push(directional_modifier.to_string());
            end_index += 1;
        }
    }
}

fn collect_alias(alias: &AliasDecl, owner_package_segments: &[String]) -> CollectedAlias {
    let target = if alias.target.segments.len() == 1 && !owner_package_segments.is_empty() {
        QualifiedName {
            segments: qualify_segments(owner_package_segments, &alias.target.segments),
            span: alias.target.span.clone(),
        }
    } else {
        alias.target.clone()
    };
    CollectedAlias {
        qualified_name: qualify_name(owner_package_segments, &alias.name),
        declared_name: alias.name.clone(),
        target,
    }
}

fn collect_alias_in_owner(alias: &AliasDecl, owner_qualified_name: &str) -> CollectedAlias {
    let target = if alias.target.segments.len() == 1 && owner_qualified_name != "root" {
        let mut segments = owner_qualified_name
            .split('.')
            .map(str::to_string)
            .collect::<Vec<_>>();
        segments.extend(alias.target.segments.clone());
        QualifiedName {
            segments,
            span: alias.target.span.clone(),
        }
    } else {
        alias.target.clone()
    };
    CollectedAlias {
        qualified_name: usage_qualified_name(owner_qualified_name, &alias.name),
        declared_name: alias.name.clone(),
        target,
    }
}

fn collect_nested_aliases(
    declarations: &[Declaration],
    owner_package_segments: &[String],
    owner_qualified_name: Option<&str>,
    aliases: &mut Vec<CollectedAlias>,
) {
    for declaration in declarations {
        match declaration {
            Declaration::Package(package) => {
                let package_segments =
                    qualify_segments(owner_package_segments, &package.name.segments);
                let package_qualified_name = package_segments.join(".");
                collect_nested_aliases(
                    &package.members,
                    &package_segments,
                    Some(&package_qualified_name),
                    aliases,
                );
            }
            Declaration::PartDefinition(definition) => {
                let qualified_name = qualify_name(owner_package_segments, &definition.name);
                collect_nested_member_aliases(&definition.members, &qualified_name, aliases);
            }
            Declaration::GenericDefinition(definition) => {
                let qualified_name = qualify_name(owner_package_segments, &definition.name);
                collect_nested_member_aliases(&definition.members, &qualified_name, aliases);
            }
            Declaration::PartUsage(usage) => {
                let qualified_name =
                    usage_qualified_name(owner_qualified_name.unwrap_or("root"), &usage.name);
                collect_nested_member_aliases(&usage.body_members, &qualified_name, aliases);
            }
            Declaration::GenericUsage(usage) => {
                let qualified_name =
                    usage_qualified_name(owner_qualified_name.unwrap_or("root"), &usage.name);
                collect_nested_member_aliases(&usage.body_members, &qualified_name, aliases);
            }
            Declaration::Import(_) | Declaration::Alias(_) => {}
        }
    }
}

fn collect_nested_member_aliases(
    declarations: &[Declaration],
    owner_qualified_name: &str,
    aliases: &mut Vec<CollectedAlias>,
) {
    for declaration in declarations {
        match declaration {
            Declaration::Alias(alias) => {
                aliases.push(collect_alias_in_owner(alias, owner_qualified_name))
            }
            Declaration::PartUsage(usage) => {
                let qualified_name = usage_qualified_name(owner_qualified_name, &usage.name);
                collect_nested_member_aliases(&usage.body_members, &qualified_name, aliases);
            }
            Declaration::GenericUsage(usage) => {
                let qualified_name = usage_qualified_name(owner_qualified_name, &usage.name);
                collect_nested_member_aliases(&usage.body_members, &qualified_name, aliases);
            }
            Declaration::PartDefinition(definition) => {
                let qualified_name = usage_qualified_name(owner_qualified_name, &definition.name);
                collect_nested_member_aliases(&definition.members, &qualified_name, aliases);
            }
            Declaration::GenericDefinition(definition) => {
                let qualified_name = usage_qualified_name(owner_qualified_name, &definition.name);
                collect_nested_member_aliases(&definition.members, &qualified_name, aliases);
            }
            Declaration::Package(package) => {
                let qualified_name =
                    usage_qualified_name(owner_qualified_name, &package.name.as_dot_string());
                collect_nested_member_aliases(&package.members, &qualified_name, aliases);
            }
            Declaration::Import(_) => {}
        }
    }
}

fn qualify_name(owner_package_segments: &[String], name: &str) -> String {
    let mut segments = owner_package_segments.to_vec();
    segments.push(name.to_string());
    segments.join(".")
}

fn usage_qualified_name(owner_qualified_name: &str, declared_name: &str) -> String {
    if owner_qualified_name == "root" {
        declared_name.to_string()
    } else {
        format!("{owner_qualified_name}.{declared_name}")
    }
}

fn qualify_segments(
    owner_package_segments: &[String],
    declared_segments: &[String],
) -> Vec<String> {
    let mut segments = owner_package_segments.to_vec();
    segments.extend(declared_segments.iter().cloned());
    segments
}
