use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub max_supported_bytes: Option<u64>,
    pub slots_total: u16,
    pub slots_populated: u16,
    pub ecc_supported: bool,
    pub channels: ChannelInfo,
    pub modules: Vec<MemoryModule>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct ChannelInfo {
    pub configuration: ChannelConfig,
    pub detected: u8,
}

#[derive(Debug, Clone, Copy, Serialize, TS, PartialEq, Eq)]
#[ts(export)]
#[serde(rename_all = "lowercase")]
pub enum ChannelConfig {
    Single,
    Dual,
    Triple,
    Quad,
    Unknown,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct MemoryModule {
    pub slot: String,
    pub bank: Option<String>,
    pub size_bytes: u64,
    pub memory_type: Option<String>,
    pub form_factor: Option<String>,
    pub speed_mhz: Option<u32>,
    pub configured_speed_mhz: Option<u32>,
    pub manufacturer: Option<String>,
    pub part_number: Option<String>,
    pub serial: Option<String>,
    pub data_width: Option<u16>,
    pub total_width: Option<u16>,
    pub voltage_v: Option<f32>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct MemoryLiveTick {
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
}
