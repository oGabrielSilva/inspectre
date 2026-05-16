use std::path::PathBuf;

use crate::error::InspectreError;

#[tauri::command]
pub async fn save_icon_source(bytes: Vec<u8>) -> Result<String, InspectreError> {
    let path = tokio::task::spawn_blocking(move || -> Result<PathBuf, InspectreError> {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let icons_dir = manifest_dir.join("icons");
        std::fs::create_dir_all(&icons_dir)?;
        let target = icons_dir.join("icon-source.png");
        std::fs::write(&target, &bytes)?;
        Ok(target)
    })
    .await
    .map_err(|e| InspectreError::Internal(e.to_string()))??;

    Ok(path.to_string_lossy().to_string())
}
