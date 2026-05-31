use std::collections::BTreeMap;

use mercurio_core::{
    KirDocument, KirElement, default_kernel_library_path, default_sysml_delta_library_path,
    default_sysml_library_path,
};
use serde_json::{Value, json};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source_path = default_sysml_library_path();
    let source = KirDocument::from_path(&source_path)?;
    let (kernel_elements, sysml_elements) = split_library_elements(source.elements);

    let kernel = split_document(
        "org.omg/kerml-kernel",
        "KerML/Kernel baseline extracted from the bundled SysML pilot stdlib.",
        &source_path.display().to_string(),
        kernel_elements,
    );
    let sysml = split_document(
        "org.omg/sysml-library",
        "SysML library delta extracted from the bundled SysML pilot stdlib. KerML/Kernel elements are intentionally excluded.",
        &source_path.display().to_string(),
        sysml_elements,
    );

    let kernel_path = default_kernel_library_path();
    kernel.write_pretty_to_path(&kernel_path)?;
    let sysml_path = default_sysml_delta_library_path();
    sysml.write_pretty_to_path(&sysml_path)?;

    println!("wrote {}", kernel_path.display());
    println!("wrote {}", sysml_path.display());
    Ok(())
}

fn split_library_elements(elements: Vec<KirElement>) -> (Vec<KirElement>, Vec<KirElement>) {
    let mut kernel_elements = Vec::new();
    let mut sysml_elements = Vec::new();

    for element in elements {
        if is_kernel_element(&element) {
            kernel_elements.push(element);
        } else {
            sysml_elements.push(element);
        }
    }

    (kernel_elements, sysml_elements)
}

fn is_kernel_element(element: &KirElement) -> bool {
    element
        .properties
        .get("metadata")
        .and_then(Value::as_object)
        .and_then(|metadata| metadata.get("pilot_library_group"))
        .and_then(Value::as_str)
        == Some("Kernel Libraries")
}

fn split_document(
    library_id: &str,
    note: &str,
    source_path: &str,
    elements: Vec<KirElement>,
) -> KirDocument {
    let metadata = BTreeMap::from([
        ("element_count".to_string(), json!(elements.len())),
        (
            "generator".to_string(),
            json!("cargo run -p mercurio-tools --bin generate_kernel_baseline"),
        ),
        (
            "kir_schema_version".to_string(),
            json!(mercurio_core::KIR_SCHEMA_VERSION),
        ),
        ("library_id".to_string(), json!(library_id)),
        ("library_version".to_string(), json!("0.0.0-bootstrap")),
        ("note".to_string(), json!(note)),
        ("source_path".to_string(), json!(source_path)),
    ]);

    KirDocument { metadata, elements }
}
