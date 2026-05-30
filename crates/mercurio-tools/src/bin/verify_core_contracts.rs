use mercurio_core::{
    CURRENT_DEFAULT_PROFILE_ID, Graph, KirDocument, LocalPackageRepository, SemanticConcept,
    default_language_profile, default_metamodel_registry, default_stdlib_path, diff_kir_documents,
    generate_python_wrappers, load_language_profile, workspace_revision_for_kir_document,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("verify_core_contracts: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let profile = default_language_profile()?;
    check(
        profile.id == CURRENT_DEFAULT_PROFILE_ID,
        format!("default profile is {}", profile.id),
    )?;

    let loaded_by_id = load_language_profile(&profile.id)?;
    check(
        loaded_by_id == profile,
        "profile loads reproducibly by id".to_string(),
    )?;

    let stdlib_path = default_stdlib_path();
    let stdlib = KirDocument::from_path(&stdlib_path)?;
    check(
        stdlib.elements.len() > 10_000,
        format!("stdlib loads with {} elements", stdlib.elements.len()),
    )?;

    let graph = Graph::from_document(stdlib.clone())?;
    let registry = default_metamodel_registry()?;
    check(
        registry
            .canonical_kind(SemanticConcept::RequirementUsage)
            .is_some(),
        "requirement usage concept is registered".to_string(),
    )?;
    check(
        graph.edge_count() > 10_000,
        format!("stdlib graph builds with {} edges", graph.edge_count()),
    )?;

    let revision_a = workspace_revision_for_kir_document(&stdlib)?;
    let revision_b = workspace_revision_for_kir_document(&stdlib)?;
    check(
        revision_a == revision_b,
        format!("stdlib revision is stable ({})", revision_a.fingerprint),
    )?;

    let wrappers_a = generate_python_wrappers(&stdlib, &profile, "mercurio_sysml");
    let wrappers_b = generate_python_wrappers(&stdlib, &profile, "mercurio_sysml");
    check(
        wrappers_a == wrappers_b,
        format!(
            "python wrappers generate deterministically ({} files)",
            wrappers_a.files.len()
        ),
    )?;
    check(
        wrappers_a.profile_id == profile.id && wrappers_a.stdlib_version == profile.stdlib_version,
        "python wrapper provenance matches language profile".to_string(),
    )?;

    let empty_diff = diff_kir_documents(&stdlib, &stdlib);
    check(
        empty_diff.added_elements.is_empty()
            && empty_diff.removed_elements.is_empty()
            && empty_diff.changed_attributes.is_empty()
            && empty_diff.added_relationships.is_empty()
            && empty_diff.removed_relationships.is_empty(),
        "semantic diff is empty for identical documents".to_string(),
    )?;

    let package =
        LocalPackageRepository::bundled().verify_package("org.omg/sysml-stdlib", "2.0.0")?;
    check(
        package.has_precompiled_kir,
        format!(
            "bundled stdlib package verifies ({} source files, digest {})",
            package.source_count, package.digest
        ),
    )?;

    println!("core contracts: ok");
    Ok(())
}

fn check(condition: bool, message: String) -> Result<(), Box<dyn std::error::Error>> {
    if condition {
        println!("OK    {message}");
        Ok(())
    } else {
        Err(format!("FAILED {message}").into())
    }
}
