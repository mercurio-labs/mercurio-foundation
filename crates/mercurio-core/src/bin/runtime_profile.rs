use std::error::Error;
use std::path::PathBuf;

use mercurio_core::{KirDocument, Runtime};

fn main() -> Result<(), Box<dyn Error>> {
    let path = parse_args()?;
    let document = KirDocument::from_path(&path)?;
    let profile = Runtime::profile_from_document(document)?;
    println!("{}", serde_json::to_string_pretty(&profile)?);
    Ok(())
}

fn parse_args() -> Result<PathBuf, Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 1 || matches!(args.first().map(String::as_str), Some("--help" | "-h")) {
        print_help();
        if args.len() == 1 {
            std::process::exit(0);
        }
        return Err("runtime_profile requires one KIR JSON path".into());
    }
    Ok(PathBuf::from(&args[0]))
}

fn print_help() {
    println!(
        "Usage: cargo run --release -p mercurio-core --bin runtime_profile -- path/to/model.kir.json"
    );
}
