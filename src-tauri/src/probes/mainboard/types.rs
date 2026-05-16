use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct MainboardInfo {
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub version: Option<String>,
    pub serial: Option<String>,
    pub chassis_manufacturer: Option<String>,
    pub chassis_model: Option<String>,
    pub bios: BiosInfo,
    pub security: SecurityInfo,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BiosInfo {
    pub vendor: Option<String>,
    pub version: Option<String>,
    pub smbios_version: Option<String>,
    pub release_date: Option<String>,
    pub firmware_type: FirmwareType,
}

#[derive(Debug, Clone, Copy, Serialize, TS, PartialEq, Eq)]
#[ts(export)]
#[serde(rename_all = "lowercase")]
pub enum FirmwareType {
    Bios,
    Uefi,
    Unknown,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct SecurityInfo {
    pub secure_boot: Option<bool>,
    pub tpm_present: bool,
    pub tpm_enabled: Option<bool>,
    pub tpm_spec_version: Option<String>,
}
