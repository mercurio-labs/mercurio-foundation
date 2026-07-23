use std::path::PathBuf;

use mercurio_codegen::{generate_python_facades, generate_typed_facades};
use mercurio_kir::KirDocument;

#[test]
fn checked_in_typed_facades_match_the_pinned_sysml_metamodel()
-> Result<(), Box<dyn std::error::Error>> {
    let foundation_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .ok_or("mercurio-codegen must be nested under mercurio-foundation/crates")?
        .to_path_buf();
    let repository_root = foundation_root
        .parent()
        .ok_or("mercurio-foundation must be nested under the repository root")?;
    let metamodel_path = repository_root.join(
        "mercurio-sysml/resources/metamodels/sysml-2.0-metamodel-0.57.0/stdlib/stdlib.full.kir.json",
    );
    let generated_path = foundation_root.join("crates/mercurio-model/src/generated_facades.rs");

    let document: KirDocument = serde_json::from_slice(&std::fs::read(metamodel_path)?)?;
    let expected = generate_typed_facades(&document)?;
    let actual = std::fs::read_to_string(generated_path)?;

    assert_eq!(
        actual.replace("\r\n", "\n"),
        expected.replace("\r\n", "\n"),
        "run generate_typed_facades to refresh the checked-in facade module"
    );
    Ok(())
}
#[test]
fn checked_in_python_facades_match_the_pinned_sysml_metamodel()
-> Result<(), Box<dyn std::error::Error>> {
    let foundation_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .ok_or("mercurio-codegen must be nested under mercurio-foundation/crates")?
        .to_path_buf();
    let repository_root = foundation_root
        .parent()
        .ok_or("mercurio-foundation must be nested under the repository root")?;
    let metamodel_path = repository_root.join(
        "mercurio-sysml/resources/metamodels/sysml-2.0-metamodel-0.57.0/stdlib/stdlib.full.kir.json",
    );
    let python_root = repository_root.join("mercurio-host-adapters/python");
    let document: KirDocument = serde_json::from_slice(&std::fs::read(metamodel_path)?)?;
    let generated = generate_python_facades(&document, "mercurio._generated");

    for (relative_path, expected) in generated.files {
        let actual = std::fs::read_to_string(python_root.join(&relative_path))?;
        assert_eq!(
            actual.replace("\r\n", "\n"),
            expected.replace("\r\n", "\n"),
            "run generate_python_facades to refresh {relative_path}"
        );
    }
    Ok(())
}
