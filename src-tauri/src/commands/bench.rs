use tauri::AppHandle;

use crate::error::InspectreError;
use crate::probes::bench::{self, BenchReport};

#[tauri::command]
pub async fn run_bench(
    app: AppHandle,
    duration_secs: u64,
) -> Result<BenchReport, InspectreError> {
    tokio::task::spawn_blocking(move || bench::run_full_report(app, duration_secs))
        .await
        .map_err(|e| InspectreError::Internal(e.to_string()))?
}
