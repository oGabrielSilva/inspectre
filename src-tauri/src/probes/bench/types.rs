use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BenchReport {
    pub sha256: BenchResult,
    pub aes256: BenchResult,
    pub prime: BenchResult,
    pub total_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BenchResult {
    pub name: String,
    pub unit: String,
    pub single_thread: BenchScore,
    pub multi_thread: BenchScore,
    pub scaling: f64,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BenchScore {
    pub throughput: f64,
    pub iterations: u64,
    pub duration_ms: u64,
    pub thread_count: u16,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BenchProgress {
    pub phase: String,
    pub phase_index: u8,
    pub total_phases: u8,
}
