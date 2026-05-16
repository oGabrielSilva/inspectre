use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct CpuInfo {
    pub name: String,
    pub vendor: String,
    pub codename: Option<String>,
    pub socket: Option<String>,
    pub process_nm: Option<u16>,
    pub revision: Option<String>,
    pub family: u8,
    pub model: u8,
    pub stepping: u8,
    pub ext_family: u8,
    pub ext_model: u8,
    pub cores: u16,
    pub threads: u16,
    pub hyperthreading: bool,
    pub virtualization_supported: bool,
    pub base_clock_mhz: Option<u32>,
    pub max_clock_mhz: Option<u32>,
    pub bus_clock_mhz: Option<u32>,
    pub current_clock_mhz: Option<u32>,
    pub cache: CacheInfo,
    pub instruction_sets: Vec<String>,
    pub tdp_w: Option<u16>,
    pub boot_time_sec: u32,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct CacheLevel {
    pub size_kb: u32,
    pub ways: u32,
    pub line_size: u32,
    pub sets: u32,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct CacheInfo {
    pub l1_data: Option<CacheLevel>,
    pub l1_inst: Option<CacheLevel>,
    pub l2: Option<CacheLevel>,
    pub l3: Option<CacheLevel>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct CpuLiveTick {
    pub mhz_avg: Option<u32>,
    pub mhz_per_core: Vec<u32>,
    pub usage_avg: f32,
    pub usage_per_core: Vec<f32>,
    pub temperature_c: Option<f32>,
}
