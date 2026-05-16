mod types;
mod workloads;

pub use types::{BenchProgress, BenchReport, BenchResult, BenchScore};

use std::time::{Duration, Instant};

use tauri::{AppHandle, Emitter};

use crate::error::InspectreError;

const TOTAL_PHASES: u8 = 6;

pub fn run_full_report(app: AppHandle, duration_secs: u64) -> Result<BenchReport, InspectreError> {
    let duration = Duration::from_secs(duration_secs.clamp(1, 30));
    let total_start = Instant::now();

    let sha = run_named(&app, "sha256", 0, duration, workloads::sha256_single, workloads::sha256_multi, "B/s");
    let aes = run_named(&app, "aes256", 2, duration, workloads::aes256_single, workloads::aes256_multi, "B/s");
    let prime = run_named(
        &app,
        "prime",
        4,
        duration,
        workloads::prime_single,
        workloads::prime_multi,
        "numbers/s",
    );

    emit_progress(&app, "done", TOTAL_PHASES);

    Ok(BenchReport {
        sha256: sha,
        aes256: aes,
        prime,
        total_duration_ms: total_start.elapsed().as_millis() as u64,
    })
}

fn run_named<S, M>(
    app: &AppHandle,
    name: &str,
    phase_offset: u8,
    duration: Duration,
    single: S,
    multi: M,
    unit: &str,
) -> BenchResult
where
    S: FnOnce(Duration) -> BenchScore,
    M: FnOnce(Duration) -> BenchScore,
{
    emit_progress(app, &format!("{name}-single"), phase_offset);
    let single_score = single(duration);

    emit_progress(app, &format!("{name}-multi"), phase_offset + 1);
    let multi_score = multi(duration);

    let scaling = if single_score.throughput > 0.0 {
        multi_score.throughput / single_score.throughput
    } else {
        0.0
    };

    BenchResult {
        name: name.to_string(),
        unit: unit.to_string(),
        single_thread: single_score,
        multi_thread: multi_score,
        scaling,
    }
}

fn emit_progress(app: &AppHandle, phase: &str, phase_index: u8) {
    let progress = BenchProgress {
        phase: phase.to_string(),
        phase_index,
        total_phases: TOTAL_PHASES,
    };
    if let Err(err) = app.emit("bench:progress", progress) {
        tracing::warn!(?err, "falha ao emitir bench:progress");
    }
}
