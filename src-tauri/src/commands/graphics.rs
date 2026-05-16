use crate::error::InspectreError;
use crate::probes::graphics::{self, GraphicsInfo};

#[tauri::command]
pub async fn graphics_info() -> Result<GraphicsInfo, InspectreError> {
    tokio::task::spawn_blocking(graphics::read_snapshot)
        .await
        .map_err(|e| InspectreError::Internal(e.to_string()))?
}
