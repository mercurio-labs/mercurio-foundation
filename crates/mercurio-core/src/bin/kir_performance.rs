use std::error::Error;

use mercurio_core::{KirPerformanceConfig, run_kir_performance};

fn main() -> Result<(), Box<dyn Error>> {
    let config = parse_args()?;
    let report = run_kir_performance(config)?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn parse_args() -> Result<KirPerformanceConfig, Box<dyn Error>> {
    let mut config = KirPerformanceConfig::default();
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--sizes" => {
                index += 1;
                let raw = args
                    .get(index)
                    .ok_or("--sizes requires a comma-separated value")?;
                config.model_sizes = raw
                    .split(',')
                    .filter(|value| !value.trim().is_empty())
                    .map(|value| value.trim().parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()?;
            }
            "--edits" => {
                index += 1;
                config.edit_count = args.get(index).ok_or("--edits requires a value")?.parse()?;
            }
            "--output-dir" => {
                index += 1;
                config.output_dir = args
                    .get(index)
                    .ok_or("--output-dir requires a value")?
                    .into();
            }
            "--keep-files" => {
                config.keep_files = true;
            }
            "--emf-command" => {
                index += 1;
                config.emf_command = Some(
                    args.get(index)
                        .ok_or("--emf-command requires a value")?
                        .clone(),
                );
            }
            "--max-runtime-size" => {
                index += 1;
                config.max_runtime_size = Some(
                    args.get(index)
                        .ok_or("--max-runtime-size requires a value")?
                        .parse()?,
                );
            }
            "--no-runtime" => {
                config.max_runtime_size = Some(0);
            }
            "--max-diff-size" => {
                index += 1;
                config.max_diff_size = Some(
                    args.get(index)
                        .ok_or("--max-diff-size requires a value")?
                        .parse()?,
                );
            }
            "--no-diff" => {
                config.max_diff_size = Some(0);
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            unknown => return Err(format!("unknown argument `{unknown}`").into()),
        }
        index += 1;
    }

    if config.model_sizes.is_empty() {
        return Err("--sizes must include at least one model size".into());
    }

    Ok(config)
}

fn print_help() {
    println!(
        "Usage: cargo run -p mercurio-core --bin kir_performance -- [--sizes 100,1000,10000,100000,1000000] [--edits 100] [--output-dir target/kir-performance] [--keep-files] [--max-runtime-size 100000] [--max-diff-size 100000] [--emf-command \"java -jar emf-benchmark.jar\"]"
    );
}
