use std::error::Error;

use mercurio_core::{CachePerformanceConfig, run_cache_performance};

fn main() -> Result<(), Box<dyn Error>> {
    let config = parse_args()?;
    let report = run_cache_performance(config)?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn parse_args() -> Result<CachePerformanceConfig, Box<dyn Error>> {
    let mut config = CachePerformanceConfig::default();
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
        "Usage: cargo run -p mercurio-foundation --bin cache_performance -- [--sizes 1000,10000,100000] [--output-dir target/cache-performance] [--keep-files]"
    );
}
