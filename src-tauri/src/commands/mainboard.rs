use crate::error::InspectreError;
use crate::probes::mainboard::{self, MainboardInfo};

#[tauri::command]
pub async fn mainboard_info() -> Result<MainboardInfo, InspectreError> {
    tokio::task::spawn_blocking(mainboard::read_snapshot)
        .await
        .map_err(|e| InspectreError::Internal(e.to_string()))?
}
