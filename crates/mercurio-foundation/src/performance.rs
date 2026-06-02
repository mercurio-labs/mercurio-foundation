use std::collections::BTreeMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::Serialize;
use serde_json::{Value, json};

use crate::graph::Graph;
use crate::ir::{KIR_SCHEMA_VERSION, KirDocument, KirElement};
use crate::mutation::diff_kir_documents;
use crate::runtime::Runtime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoreScalabilityCreationStrategy {
    SessionOverlay,
    Mutators,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreScalabilityMetricConfig {
    pub model_sizes: Vec<usize>,
    pub edit_count: usize,
    pub target_file: String,
    pub package_name: String,
    pub creation_strategy: CoreScalabilityCreationStrategy,
}

impl Default for CoreScalabilityMetricConfig {
    fn default() -> Self {
        Self {
            model_sizes: vec![100, 1_000, 10_000],
            edit_count: 100,
            target_file: "scalability.model".to_string(),
            package_name: "Scalability".to_string(),
            creation_strategy: CoreScalabilityCreationStrategy::SessionOverlay,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CoreScalabilityReport {
    pub generated_at_unix_seconds: u64,
    pub target_file: String,
    pub package_name: String,
    pub creation_strategy: CoreScalabilityCreationStrategy,
    pub edit_count_requested: usize,
    pub scenarios: Vec<CoreScalabilityScenarioReport>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoreScalabilityScenarioReport {
    pub model_size: usize,
    pub edit_count: usize,
    pub source_file: String,
    pub source_bytes: usize,
    pub edited_source_bytes: usize,
    pub kir_elements_before: usize,
    pub kir_elements_after: usize,
    pub diff_summary: SemanticDiffSummary,
    pub timings: CoreScalabilityTimings,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoreScalabilityTimings {
    pub create_model: TimingMetric,
    pub save_source: TimingMetric,
    pub reload_source: TimingMetric,
    pub compile_before_kir: TimingMetric,
    pub prepare_edits: TimingMetric,
    pub apply_edits: TimingMetric,
    pub reload_edited_source: TimingMetric,
    pub compile_after_kir: TimingMetric,
    pub diff_kir: TimingMetric,
    pub total: TimingMetric,
}

#[derive(Debug, Clone, Serialize)]
pub struct TimingMetric {
    pub millis: f64,
}

impl TimingMetric {
    pub fn zero() -> Self {
        Self { millis: 0.0 }
    }

    fn from_duration(duration: Duration) -> Self {
        Self {
            millis: duration.as_secs_f64() * 1_000.0,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SemanticDiffSummary {
    pub added_elements: usize,
    pub removed_elements: usize,
    pub renamed_elements: usize,
    pub moved_elements: usize,
    pub retyped_usages: usize,
    pub changed_specializations: usize,
    pub changed_attributes: usize,
    pub added_relationships: usize,
    pub removed_relationships: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KirPerformanceConfig {
    pub model_sizes: Vec<usize>,
    pub edit_count: usize,
    pub output_dir: PathBuf,
    pub keep_files: bool,
    pub emf_command: Option<String>,
    pub max_runtime_size: Option<usize>,
    pub max_diff_size: Option<usize>,
}

impl Default for KirPerformanceConfig {
    fn default() -> Self {
        Self {
            model_sizes: vec![100, 1_000, 10_000, 100_000, 1_000_000],
            edit_count: 100,
            output_dir: std::env::temp_dir().join("mercurio-kir-performance"),
            keep_files: false,
            emf_command: None,
            max_runtime_size: Some(100_000),
            max_diff_size: Some(100_000),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct KirPerformanceReport {
    pub generated_at_unix_seconds: u64,
    pub output_dir: String,
    pub keep_files: bool,
    pub edit_count_requested: usize,
    pub scenarios: Vec<KirPerformanceScenarioReport>,
    pub emf_comparison: EmfComparisonReport,
}

#[derive(Debug, Clone, Serialize)]
pub struct KirPerformanceScenarioReport {
    pub model_size: usize,
    pub edit_count: usize,
    pub file_path: String,
    pub json_bytes: u64,
    pub timings: KirPerformanceTimings,
    pub memory: KirPerformanceMemory,
    pub diff_summary: SemanticDiffSummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct KirPerformanceTimings {
    pub create_kir: TimingMetric,
    pub validate_created: TimingMetric,
    pub write_json: TimingMetric,
    pub load_json: TimingMetric,
    pub build_graph: TimingMetric,
    pub build_runtime: Option<TimingMetric>,
    pub mutate_kir: TimingMetric,
    pub diff_mutation: Option<TimingMetric>,
    pub total: TimingMetric,
}

#[derive(Debug, Clone, Serialize)]
pub struct KirPerformanceMemory {
    pub after_create: MemoryMetric,
    pub after_load: MemoryMetric,
    pub after_graph: MemoryMetric,
    pub after_runtime: Option<MemoryMetric>,
    pub after_mutate: MemoryMetric,
    pub after_diff: Option<MemoryMetric>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct MemoryMetric {
    pub current_rss_bytes: Option<u64>,
    pub peak_rss_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum EmfComparisonReport {
    NotConfigured,
    Completed {
        command: String,
        elapsed: TimingMetric,
        exit_code: Option<i32>,
        stdout: String,
        stderr: String,
    },
    Failed {
        command: String,
        message: String,
    },
}

pub fn run_core_scalability_metric(
    config: CoreScalabilityMetricConfig,
) -> Result<CoreScalabilityReport, Box<dyn Error>> {
    let generated_at_unix_seconds = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let performance = run_kir_performance(KirPerformanceConfig {
        model_sizes: config.model_sizes.clone(),
        edit_count: config.edit_count,
        output_dir: std::env::temp_dir().join("mercurio-core-scalability"),
        keep_files: false,
        emf_command: None,
        max_runtime_size: Some(100_000),
        max_diff_size: Some(100_000),
    })?;
    Ok(CoreScalabilityReport {
        generated_at_unix_seconds,
        target_file: config.target_file,
        package_name: config.package_name,
        creation_strategy: config.creation_strategy,
        edit_count_requested: config.edit_count,
        scenarios: performance
            .scenarios
            .into_iter()
            .map(|scenario| CoreScalabilityScenarioReport {
                model_size: scenario.model_size,
                edit_count: scenario.edit_count,
                source_file: scenario.file_path,
                source_bytes: scenario.json_bytes as usize,
                edited_source_bytes: scenario.json_bytes as usize,
                kir_elements_before: scenario.model_size + 1,
                kir_elements_after: scenario.model_size + 1,
                diff_summary: scenario.diff_summary,
                timings: CoreScalabilityTimings {
                    create_model: scenario.timings.create_kir,
                    save_source: scenario.timings.write_json,
                    reload_source: scenario.timings.load_json,
                    compile_before_kir: scenario.timings.build_graph,
                    prepare_edits: TimingMetric::zero(),
                    apply_edits: scenario.timings.mutate_kir,
                    reload_edited_source: TimingMetric::zero(),
                    compile_after_kir: scenario
                        .timings
                        .build_runtime
                        .unwrap_or_else(TimingMetric::zero),
                    diff_kir: scenario
                        .timings
                        .diff_mutation
                        .unwrap_or_else(TimingMetric::zero),
                    total: scenario.timings.total,
                },
            })
            .collect(),
    })
}

pub fn run_kir_performance(
    config: KirPerformanceConfig,
) -> Result<KirPerformanceReport, Box<dyn Error>> {
    std::fs::create_dir_all(&config.output_dir)?;
    let generated_at_unix_seconds = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let mut scenarios = Vec::with_capacity(config.model_sizes.len());

    for model_size in &config.model_sizes {
        scenarios.push(run_kir_performance_scenario(
            *model_size,
            config.edit_count,
            &config.output_dir,
            config.keep_files,
            config.max_runtime_size,
            config.max_diff_size,
        )?);
    }

    let emf_comparison = run_emf_comparison(config.emf_command.as_deref(), &config.output_dir);

    Ok(KirPerformanceReport {
        generated_at_unix_seconds,
        output_dir: config.output_dir.display().to_string(),
        keep_files: config.keep_files,
        edit_count_requested: config.edit_count,
        scenarios,
        emf_comparison,
    })
}

fn run_kir_performance_scenario(
    model_size: usize,
    edit_count_requested: usize,
    output_dir: &Path,
    keep_files: bool,
    max_runtime_size: Option<usize>,
    max_diff_size: Option<usize>,
) -> Result<KirPerformanceScenarioReport, Box<dyn Error>> {
    let total_timer = Instant::now();
    let file_path = output_dir.join(format!("kir-{model_size}.json"));

    let create_timer = Instant::now();
    let document = synthetic_kir_document(model_size);
    let create_kir = TimingMetric::from_duration(create_timer.elapsed());
    let after_create = current_memory();

    let validate_timer = Instant::now();
    document.validate_persisted()?;
    let validate_created = TimingMetric::from_duration(validate_timer.elapsed());

    let write_timer = Instant::now();
    document.write_pretty_to_path(&file_path)?;
    let write_json = TimingMetric::from_duration(write_timer.elapsed());
    let json_bytes = std::fs::metadata(&file_path)?.len();
    drop(document);

    let load_timer = Instant::now();
    let graph_document = KirDocument::from_path(&file_path)?;
    let load_json = TimingMetric::from_duration(load_timer.elapsed());
    let after_load = current_memory();

    let graph_timer = Instant::now();
    let graph = Graph::from_document(graph_document)?;
    let build_graph = TimingMetric::from_duration(graph_timer.elapsed());
    let after_graph = current_memory();
    drop(graph);

    let (build_runtime, after_runtime) = if max_runtime_size.is_none_or(|limit| model_size <= limit)
    {
        let runtime_document = KirDocument::from_path(&file_path)?;
        let runtime_timer = Instant::now();
        let runtime = Runtime::from_document(runtime_document)?;
        let timing = TimingMetric::from_duration(runtime_timer.elapsed());
        let memory = current_memory();
        drop(runtime);
        (Some(timing), Some(memory))
    } else {
        (None, None)
    };

    let edit_count = edit_count_requested.min(model_size);
    let loaded = KirDocument::from_path(&file_path)?;
    let mutate_timer = Instant::now();
    let mutated = mutate_document(loaded.clone(), edit_count);
    let mutate_kir = TimingMetric::from_duration(mutate_timer.elapsed());
    let after_mutate = current_memory();

    let (diff_mutation, after_diff, diff_summary) =
        if max_diff_size.is_none_or(|limit| model_size <= limit) {
            let diff_timer = Instant::now();
            let diff = diff_kir_documents(&loaded, &mutated);
            let timing = TimingMetric::from_duration(diff_timer.elapsed());
            let memory = current_memory();
            (
                Some(timing),
                Some(memory),
                SemanticDiffSummary {
                    added_elements: diff.added_elements.len(),
                    removed_elements: diff.removed_elements.len(),
                    renamed_elements: diff.renamed_elements.len(),
                    moved_elements: diff.moved_elements.len(),
                    retyped_usages: diff.retyped_usages.len(),
                    changed_specializations: diff.changed_specializations.len(),
                    changed_attributes: diff.changed_attributes.len(),
                    added_relationships: diff.added_relationships.len(),
                    removed_relationships: diff.removed_relationships.len(),
                },
            )
        } else {
            (
                None,
                None,
                SemanticDiffSummary {
                    added_elements: 0,
                    removed_elements: 0,
                    renamed_elements: 0,
                    moved_elements: 0,
                    retyped_usages: 0,
                    changed_specializations: 0,
                    changed_attributes: edit_count,
                    added_relationships: 0,
                    removed_relationships: 0,
                },
            )
        };

    if !keep_files {
        let _ = std::fs::remove_file(&file_path);
    }

    Ok(KirPerformanceScenarioReport {
        model_size,
        edit_count,
        file_path: file_path.display().to_string(),
        json_bytes,
        timings: KirPerformanceTimings {
            create_kir,
            validate_created,
            write_json,
            load_json,
            build_graph,
            build_runtime,
            mutate_kir,
            diff_mutation,
            total: TimingMetric::from_duration(total_timer.elapsed()),
        },
        memory: KirPerformanceMemory {
            after_create,
            after_load,
            after_graph,
            after_runtime,
            after_mutate,
            after_diff,
        },
        diff_summary,
    })
}

fn synthetic_kir_document(model_size: usize) -> KirDocument {
    let mut elements = Vec::with_capacity(model_size + 1);
    let member_ids = (0..model_size)
        .map(|index| format!("type.Perf.Element{index}"))
        .collect::<Vec<_>>();
    elements.push(KirElement {
        id: "pkg.Perf".to_string(),
        kind: "model.Package".to_string(),
        layer: 2,
        properties: BTreeMap::from([
            ("qualified_name".to_string(), json!("Perf")),
            ("declared_name".to_string(), json!("Perf")),
            ("members".to_string(), json!(member_ids)),
        ]),
    });

    for index in 0..model_size {
        let id = format!("type.Perf.Element{index}");
        let mut properties = BTreeMap::from([
            (
                "qualified_name".to_string(),
                json!(format!("Perf.Element{index}")),
            ),
            (
                "declared_name".to_string(),
                json!(format!("Element{index}")),
            ),
            ("owner".to_string(), json!("pkg.Perf")),
        ]);
        if index > 0 {
            properties.insert(
                "specializes".to_string(),
                json!([format!("type.Perf.Element{}", index - 1)]),
            );
        }
        elements.push(KirElement {
            id,
            kind: "model.Type".to_string(),
            layer: 2,
            properties,
        });
    }

    KirDocument {
        metadata: BTreeMap::from([("kir_schema_version".to_string(), json!(KIR_SCHEMA_VERSION))]),
        elements,
    }
}

fn mutate_document(mut document: KirDocument, edit_count: usize) -> KirDocument {
    for index in 0..edit_count {
        if let Some(element) = document.elements.get_mut(index + 1) {
            element.properties.insert(
                "description".to_string(),
                Value::String(format!("edited {index}")),
            );
        }
    }
    document
}

fn run_emf_comparison(command: Option<&str>, output_dir: &Path) -> EmfComparisonReport {
    let Some(command) = command else {
        return EmfComparisonReport::NotConfigured;
    };
    let timer = Instant::now();
    let output = if cfg!(windows) {
        Command::new("cmd")
            .args(["/C", command])
            .env("MERCURIO_PERF_OUTPUT_DIR", output_dir)
            .output()
    } else {
        Command::new("sh")
            .args(["-c", command])
            .env("MERCURIO_PERF_OUTPUT_DIR", output_dir)
            .output()
    };

    match output {
        Ok(output) => EmfComparisonReport::Completed {
            command: command.to_string(),
            elapsed: TimingMetric::from_duration(timer.elapsed()),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        },
        Err(err) => EmfComparisonReport::Failed {
            command: command.to_string(),
            message: err.to_string(),
        },
    }
}

#[cfg(target_os = "windows")]
fn current_memory() -> MemoryMetric {
    #[repr(C)]
    struct ProcessMemoryCounters {
        cb: u32,
        page_fault_count: u32,
        peak_working_set_size: usize,
        working_set_size: usize,
        quota_peak_paged_pool_usage: usize,
        quota_paged_pool_usage: usize,
        quota_peak_non_paged_pool_usage: usize,
        quota_non_paged_pool_usage: usize,
        pagefile_usage: usize,
        peak_pagefile_usage: usize,
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetCurrentProcess() -> *mut std::ffi::c_void;
    }

    #[link(name = "psapi")]
    unsafe extern "system" {
        fn GetProcessMemoryInfo(
            process: *mut std::ffi::c_void,
            counters: *mut ProcessMemoryCounters,
            size: u32,
        ) -> i32;
    }

    let mut counters = ProcessMemoryCounters {
        cb: std::mem::size_of::<ProcessMemoryCounters>() as u32,
        page_fault_count: 0,
        peak_working_set_size: 0,
        working_set_size: 0,
        quota_peak_paged_pool_usage: 0,
        quota_paged_pool_usage: 0,
        quota_peak_non_paged_pool_usage: 0,
        quota_non_paged_pool_usage: 0,
        pagefile_usage: 0,
        peak_pagefile_usage: 0,
    };

    let ok = unsafe {
        GetProcessMemoryInfo(
            GetCurrentProcess(),
            &mut counters,
            std::mem::size_of::<ProcessMemoryCounters>() as u32,
        )
    };
    if ok == 0 {
        return MemoryMetric {
            current_rss_bytes: None,
            peak_rss_bytes: None,
        };
    }

    MemoryMetric {
        current_rss_bytes: Some(counters.working_set_size as u64),
        peak_rss_bytes: Some(counters.peak_working_set_size as u64),
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
fn current_memory() -> MemoryMetric {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let current = parse_proc_status_kb(&status, "VmRSS:").map(|kb| kb * 1024);
    let peak = parse_proc_status_kb(&status, "VmHWM:").map(|kb| kb * 1024);
    MemoryMetric {
        current_rss_bytes: current,
        peak_rss_bytes: peak,
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
fn parse_proc_status_kb(status: &str, key: &str) -> Option<u64> {
    status.lines().find_map(|line| {
        let rest = line.strip_prefix(key)?;
        rest.split_whitespace().next()?.parse::<u64>().ok()
    })
}

#[cfg(not(any(target_os = "windows", all(unix, not(target_os = "macos")))))]
fn current_memory() -> MemoryMetric {
    MemoryMetric {
        current_rss_bytes: None,
        peak_rss_bytes: None,
    }
}

#[cfg(test)]
mod tests {
    use super::{KirPerformanceConfig, run_kir_performance};

    #[test]
    fn kir_performance_runs_small_scenario() {
        let output_dir = std::env::temp_dir().join(format!(
            "mercurio_kir_performance_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let report = run_kir_performance(KirPerformanceConfig {
            model_sizes: vec![10],
            edit_count: 3,
            output_dir: output_dir.clone(),
            keep_files: false,
            emf_command: None,
            max_runtime_size: Some(10),
            max_diff_size: Some(10),
        })
        .unwrap();

        assert_eq!(report.scenarios.len(), 1);
        assert_eq!(report.scenarios[0].model_size, 10);
        assert_eq!(report.scenarios[0].edit_count, 3);
        assert_eq!(report.scenarios[0].diff_summary.changed_attributes, 3);
        let _ = std::fs::remove_dir_all(output_dir);
    }
}
