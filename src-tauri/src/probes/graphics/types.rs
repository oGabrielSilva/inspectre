use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct GraphicsInfo {
    pub adapters: Vec<GraphicsAdapter>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct GraphicsAdapter {
    pub name: String,
    pub vendor: GpuVendor,
    pub vendor_id: u32,
    pub device_id: u32,
    pub subsys_id: u32,
    pub revision: u32,
    pub kind: GpuKind,
    pub dedicated_memory_bytes: u64,
    pub shared_memory_bytes: u64,
    pub driver_version: Option<String>,
    pub driver_date: Option<String>,
    pub video_processor: Option<String>,
    pub current_resolution: Option<Resolution>,
    pub refresh_rate_hz: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, TS, PartialEq, Eq)]
#[ts(export)]
#[serde(rename_all = "lowercase")]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Microsoft,
    Qualcomm,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, TS, PartialEq, Eq)]
#[ts(export)]
#[serde(rename_all = "lowercase")]
pub enum GpuKind {
    Dedicated,
    Integrated,
    Software,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}
