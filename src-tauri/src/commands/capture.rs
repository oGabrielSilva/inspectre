use std::path::PathBuf;

use chrono::Local;
use tauri::{AppHandle, Manager};
use tracing::info;
use xcap::Monitor;

use crate::error::InspectreError;

#[tauri::command]
pub async fn capture_window(app: AppHandle, section: String) -> Result<String, InspectreError> {
    let webview = app
        .get_webview_window("main")
        .ok_or_else(|| InspectreError::Capture {
            message: "janela principal não encontrada".to_string(),
        })?;
    let position = webview.inner_position().map_err(|err| InspectreError::Capture {
        message: format!("posição da janela indisponível: {err}"),
    })?;
    let size = webview.inner_size().map_err(|err| InspectreError::Capture {
        message: format!("tamanho da janela indisponível: {err}"),
    })?;

    let cache_dir = app
        .path()
        .app_cache_dir()
        .map_err(|err| InspectreError::Capture {
            message: format!("cache dir indisponível: {err}"),
        })?;
    let captures_dir = cache_dir.join("captures");

    let win_x = position.x;
    let win_y = position.y;
    let win_w = size.width as i32;
    let win_h = size.height as i32;

    if win_w <= 0 || win_h <= 0 {
        return Err(InspectreError::Capture {
            message: format!("dimensões inválidas: {win_w}x{win_h}"),
        });
    }

    tokio::task::spawn_blocking(move || -> Result<String, InspectreError> {
        let center_x = win_x + win_w / 2;
        let center_y = win_y + win_h / 2;

        let monitor =
            Monitor::from_point(center_x, center_y).map_err(|err| InspectreError::Capture {
                message: format!("falha ao localizar monitor: {err}"),
            })?;

        let mon_x = monitor.x().map_err(|err| InspectreError::Capture {
            message: format!("monitor.x falhou: {err}"),
        })?;
        let mon_y = monitor.y().map_err(|err| InspectreError::Capture {
            message: format!("monitor.y falhou: {err}"),
        })?;

        let rel_x = (win_x - mon_x).max(0) as u32;
        let rel_y = (win_y - mon_y).max(0) as u32;

        let image = monitor
            .capture_region(rel_x, rel_y, win_w as u32, win_h as u32)
            .map_err(|err| InspectreError::Capture {
                message: format!("captura da região falhou: {err}"),
            })?;

        std::fs::create_dir_all(&captures_dir)?;
        let timestamp = Local::now().format("%Y-%m-%d-%H%M%S").to_string();
        let filename = format!("inspectre-{section}-{timestamp}.png");
        let path: PathBuf = captures_dir.join(filename);

        image.save(&path).map_err(|err| InspectreError::Capture {
            message: format!("falha ao salvar PNG: {err}"),
        })?;

        info!("captura temporária em {}", path.display());
        Ok(path.to_string_lossy().to_string())
    })
    .await
    .map_err(|err| InspectreError::Internal(format!("task: {err}")))?
}

#[tauri::command]
pub async fn save_capture(from: String, to: String) -> Result<String, InspectreError> {
    tokio::task::spawn_blocking(move || -> Result<String, InspectreError> {
        let from = PathBuf::from(from);
        let to = PathBuf::from(to);
        if let Some(parent) = to.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(&from, &to).map_err(|err| InspectreError::Capture {
            message: format!("falha ao copiar imagem: {err}"),
        })?;
        info!("captura salva em {}", to.display());
        Ok(to.to_string_lossy().to_string())
    })
    .await
    .map_err(|err| InspectreError::Internal(format!("task: {err}")))?
}
