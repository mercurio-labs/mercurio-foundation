use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

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

pub fn run_core_scalability_metric(
    config: CoreScalabilityMetricConfig,
) -> Result<CoreScalabilityReport, Box<dyn Error>> {
    let generated_at_unix_seconds = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    Ok(CoreScalabilityReport {
        generated_at_unix_seconds,
        target_file: config.target_file,
        package_name: config.package_name,
        creation_strategy: config.creation_strategy,
        edit_count_requested: config.edit_count,
        scenarios: Vec::new(),
    })
}
