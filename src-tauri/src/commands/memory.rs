use std::sync::Mutex;
use std::time::Duration;

use tauri::{AppHandle, Emitter, State};
use tokio::task::JoinHandle;

use crate::error::InspectreError;
use crate::probes::memory::{self, MemoryInfo, MemoryLiveTick};

#[derive(Default)]
pub struct MemoryLiveState {
    handle: Mutex<Option<JoinHandle<()>>>,
}

#[tauri::command]
pub async fn memory_info() -> Result<MemoryInfo, InspectreError> {
    tokio::task::spawn_blocking(memory::read_snapshot)
        .await
        .map_err(|e| InspectreError::Internal(e.to_string()))?
}

#[tauri::command]
pub async fn start_memory_live_stream(
    app: AppHandle,
    state: State<'_, MemoryLiveState>,
    interval_ms: u64,
) -> Result<(), InspectreError> {
    if let Ok(mut guard) = state.handle.lock() {
        if let Some(prev) = guard.take() {
            prev.abort();
        }
    }

    let interval = Duration::from_millis(interval_ms.max(500));
    let app_handle = app.clone();

    let handle = tokio::spawn(async move {
        loop {
            let tick: Option<MemoryLiveTick> = tokio::task::spawn_blocking(memory::read_live_tick)
                .await
                .ok();

            if let Some(tick) = tick {
                if let Err(err) = app_handle.emit("memory:live", tick) {
                    tracing::warn!(?err, "falha ao emitir memory:live");
                }
            }

            tokio::time::sleep(interval).await;
        }
    });

    if let Ok(mut guard) = state.handle.lock() {
        *guard = Some(handle);
    }

    Ok(())
}

#[tauri::command]
pub async fn stop_memory_live_stream(
    state: State<'_, MemoryLiveState>,
) -> Result<(), InspectreError> {
    if let Ok(mut guard) = state.handle.lock() {
        if let Some(handle) = guard.take() {
            handle.abort();
        }
    }
    Ok(())
}
