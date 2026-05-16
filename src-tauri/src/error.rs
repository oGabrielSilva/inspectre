use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[allow(dead_code)]
pub enum InspectreError {
    #[error("sensor indisponível: {sensor}")]
    SensorUnavailable { sensor: String },

    #[error("falha WMI: {message}")]
    Wmi { message: String },

    #[error("falha no probe {domain}: {message}")]
    Probe { domain: String, message: String },

    #[error("falha ao capturar a tela: {message}")]
    Capture { message: String },

    #[error("erro interno: {0}")]
    Internal(String),
}

#[cfg(windows)]
impl From<wmi::WMIError> for InspectreError {
    fn from(err: wmi::WMIError) -> Self {
        Self::Wmi {
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for InspectreError {
    fn from(err: std::io::Error) -> Self {
        Self::Internal(err.to_string())
    }
}
