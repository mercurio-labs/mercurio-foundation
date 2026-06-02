use std::fs;
use std::path::{Path, PathBuf};

use mercurio_core::{
    BaselineLibrary, KIR_SCHEMA_VERSION, KirDocument, PROJECT_DESCRIPTOR_FILE_NAME, SourceLanguage,
    default_kernel_library_path, default_sysml_delta_library_path, default_sysml_library_path,
    load_model_stack, load_model_stack_with_language, resolve_project_context_for_language,
};
use serde_json::json;

#[test]
fn descriptorless_kerml_project_uses_kernel_baseline() {
    let root = temp_dir("descriptorless_kerml_project_uses_kernel_baseline");
    let model_path = root.join("models").join("demo.kerml");
    fs::create_dir_all(model_path.parent().unwrap()).unwrap();
    fs::write(&model_path, "package Demo { classifier Vehicle; }\n").unwrap();

    let context = resolve_project_context_for_language(&model_path, Some(SourceLanguage::Kerml))
        .expect("KerML project context should resolve");

    assert!(context.descriptor_path.is_none());
    assert_eq!(context.resolved_libraries.len(), 1);
    assert_eq!(context.resolved_libraries[0].id, "kernel");
    assert_eq!(
        context.resolved_libraries[0].source_path.as_deref(),
        Some(default_kernel_library_path().as_path())
    );
    assert!(context.library_context_document.elements.len() > 1000);
    assert_eq!(
        merged_library_id(&context.library_context_document),
        Some("org.omg/kerml-kernel")
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn descriptorless_sysml_project_uses_sysml_baseline() {
    let root = temp_dir("descriptorless_sysml_project_uses_sysml_baseline");
    let model_path = root.join("models").join("demo.sysml");
    fs::create_dir_all(model_path.parent().unwrap()).unwrap();
    fs::write(&model_path, "package Demo { part def Vehicle; }\n").unwrap();

    let context = resolve_project_context_for_language(&model_path, Some(SourceLanguage::Sysml))
        .expect("SysML project context should resolve");

    assert!(context.descriptor_path.is_none());
    assert_eq!(context.resolved_libraries.len(), 1);
    assert_eq!(context.resolved_libraries[0].id, "stdlib");
    assert_eq!(
        context.resolved_libraries[0].source_path.as_deref(),
        Some(default_sysml_delta_library_path().as_path())
    );
    assert!(context.library_context_document.elements.len() > 100);
    assert!(
        context
            .library_context_document
            .elements
            .iter()
            .any(|element| element.id == "KerML::Core::Type")
    );
    assert!(
        context
            .library_context_document
            .elements
            .iter()
            .any(|element| element.id == "SysML::Systems::PartDefinition")
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn project_descriptor_baseline_overrides_language_default() {
    let root = temp_dir("project_descriptor_baseline_overrides_language_default");
    let baseline_path = root.join("baseline.kir.json");
    write_kir(
        &baseline_path,
        "local/baseline",
        vec![json!({
            "id": "type.LocalBaseline",
            "kind": "KerML::Core::Type",
            "layer": 0,
            "properties": {}
        })],
    );
    fs::write(
        root.join(PROJECT_DESCRIPTOR_FILE_NAME),
        format!(
            r#"{{
  "libraries": [
    {{
      "id": "local",
      "role": "baseline",
      "provider": {{ "kind": "local_kir_file", "path": "{}" }}
    }}
  ]
}}"#,
            baseline_path.file_name().unwrap().to_string_lossy()
        ),
    )
    .unwrap();

    let model_path = root.join("demo.kerml");
    fs::write(&model_path, "package Demo { classifier Vehicle; }\n").unwrap();

    let context = resolve_project_context_for_language(&model_path, Some(SourceLanguage::Kerml))
        .expect("descriptor baseline should resolve");

    assert!(context.descriptor_path.is_some());
    assert_eq!(context.resolved_libraries.len(), 1);
    assert_eq!(context.resolved_libraries[0].id, "local");
    assert_eq!(
        merged_library_id(&context.library_context_document),
        Some("local/baseline")
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn kir_json_loads_without_language_baseline_merge() {
    let root = temp_dir("kir_json_loads_without_language_baseline_merge");
    let kir_path = root.join("model.kir.json");
    write_kir(
        &kir_path,
        "test/raw",
        vec![json!({
            "id": "type.RawOnly",
            "kind": "KerML::Core::Type",
            "layer": 2,
            "properties": {}
        })],
    );

    let document =
        load_model_stack_with_language(&kir_path, SourceLanguage::Sysml).expect("raw KIR loads");

    assert_eq!(document.elements.len(), 1);
    assert!(
        document
            .elements
            .iter()
            .any(|element| element.id == "type.RawOnly")
    );
    assert!(
        !document
            .elements
            .iter()
            .any(|element| element.id == "SysML::Systems::PartDefinition")
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn legacy_json_model_stack_still_merges_sysml_baseline() {
    let document = load_model_stack(&repo_path("test_files/examples/vehicle_model.json"))
        .expect("legacy JSON model stack loads");

    assert!(
        document
            .elements
            .iter()
            .any(|element| element.id == "SysML::Systems::PartDefinition")
    );
}

#[test]
fn kernel_baseline_contains_kerml_core_content() {
    let document = KirDocument::from_path(&default_kernel_library_path())
        .expect("Kernel baseline should load");

    assert!(document.elements.len() > 1000);
    assert!(
        document
            .elements
            .iter()
            .any(|element| element.id == "KerML")
    );
    assert!(
        document
            .elements
            .iter()
            .any(|element| element.id == "KerML::Core::Type")
    );
    assert!(
        !document.elements.iter().any(|element| element.id == "SysML"
            || element.id.starts_with("SysML::")
            || pilot_library_group(element) != Some("Kernel Libraries"))
    );
}

#[test]
fn sysml_delta_excludes_kerml_core_content() {
    let document = KirDocument::from_path(&default_sysml_delta_library_path())
        .expect("SysML delta should load");

    assert!(document.elements.len() > 1000);
    assert!(
        document
            .elements
            .iter()
            .any(|element| element.id == "SysML::Systems::PartDefinition")
    );
    assert!(
        !document.elements.iter().any(|element| element.id == "KerML"
            || element.id.starts_with("KerML::")
            || pilot_library_group(element) == Some("Kernel Libraries"))
    );
}

#[test]
fn sysml_baseline_merges_kernel_and_sysml_delta() {
    let document = BaselineLibrary::Sysml
        .load()
        .expect("SysML baseline should merge Kernel and SysML delta");

    assert!(
        document
            .elements
            .iter()
            .any(|element| element.id == "KerML::Core::Type")
    );
    assert!(
        document
            .elements
            .iter()
            .any(|element| element.id == "SysML::Systems::PartDefinition")
    );
    assert!(document.elements.len() > 10262);
    assert!(
        document
            .elements
            .iter()
            .any(|element| element.kind == "MetamodelFeature")
    );
}

#[test]
fn bundled_sysml_library_loads_as_raw_kir() {
    let document = load_model_stack(&default_sysml_library_path()).expect("SysML library loads");

    assert!(document.elements.len() > 100);
    assert_eq!(
        document
            .metadata
            .get("merged_sources")
            .and_then(|value| value.as_array()),
        None
    );
}

#[test]
fn bundled_kir_resources_satisfy_persisted_contract() {
    for path in [
        default_kernel_library_path(),
        default_sysml_delta_library_path(),
        default_sysml_library_path(),
        repo_path("resources/stdlib-sources/sysml-2.0-pilot-0.57.0/stdlib.kir.json"),
    ] {
        KirDocument::from_path(&path).unwrap_or_else(|error| {
            panic!(
                "bundled KIR resource should satisfy strict persisted contract: {}\n{}",
                path.display(),
                error
            )
        });
    }
}

fn pilot_library_group(element: &mercurio_core::KirElement) -> Option<&str> {
    element
        .properties
        .get("pilot_library_group")
        .and_then(|value| value.as_str())
        .or_else(|| {
            element
                .properties
                .get("metadata")
                .and_then(|value| value.as_object())
                .and_then(|metadata| metadata.get("pilot_library_group"))
                .and_then(|value| value.as_str())
        })
}

fn write_kir(path: &Path, library_id: &str, mut elements: Vec<serde_json::Value>) {
    for element in &mut elements {
        let Some(id) = element.get("id").and_then(|value| value.as_str()) else {
            continue;
        };
        let Some(qualified_name) = id.split_once('.').map(|(_, value)| value.to_string()) else {
            continue;
        };
        if let Some(properties) = element
            .get_mut("properties")
            .and_then(|value| value.as_object_mut())
        {
            properties
                .entry("qualified_name".to_string())
                .or_insert_with(|| json!(qualified_name));
        }
    }

    let document = json!({
        "metadata": {
            "kir_schema_version": KIR_SCHEMA_VERSION,
            "library_id": library_id
        },
        "elements": elements
    });
    fs::write(path, serde_json::to_string_pretty(&document).unwrap()).unwrap();
}

fn merged_library_id(document: &KirDocument) -> Option<&str> {
    document
        .metadata
        .get("merged_sources")
        .and_then(|value| value.as_array())
        .and_then(|sources| sources.first())
        .and_then(|source| source.get("library_id"))
        .and_then(|value| value.as_str())
}

fn repo_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(relative)
}

fn temp_dir(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("mercurio-{name}-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    fs::create_dir_all(&root).unwrap();
    root
}
