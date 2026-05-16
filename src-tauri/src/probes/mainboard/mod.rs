mod types;

pub use types::{BiosInfo, FirmwareType, MainboardInfo, SecurityInfo};

use serde::Deserialize;
use windows::Win32::System::SystemInformation;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;
use wmi::WMIConnection;

use crate::error::InspectreError;
use crate::util::format_cim_date;

pub fn read_snapshot() -> Result<MainboardInfo, InspectreError> {
    let baseboard = read_baseboard().unwrap_or_else(|err| {
        tracing::warn!(?err, "Win32_BaseBoard indisponível");
        None
    });

    let chassis = read_chassis().unwrap_or_else(|err| {
        tracing::warn!(?err, "Win32_ComputerSystem indisponível");
        None
    });

    let bios_row = read_bios().unwrap_or_else(|err| {
        tracing::warn!(?err, "Win32_BIOS indisponível");
        None
    });

    let bios = BiosInfo {
        vendor: bios_row.as_ref().and_then(|b| b.manufacturer.clone()),
        version: bios_row.as_ref().and_then(|b| b.smbios_bios_version.clone()),
        smbios_version: bios_row.as_ref().and_then(smbios_version_string),
        release_date: bios_row
            .as_ref()
            .and_then(|b| b.release_date.as_deref())
            .and_then(format_cim_date),
        firmware_type: read_firmware_type(),
    };

    let security = read_security();

    Ok(MainboardInfo {
        manufacturer: baseboard.as_ref().and_then(|b| b.manufacturer.clone()),
        product: baseboard.as_ref().and_then(|b| b.product.clone()),
        version: baseboard.as_ref().and_then(|b| b.version.clone()),
        serial: baseboard.as_ref().and_then(|b| b.serial_number.clone()),
        chassis_manufacturer: chassis.as_ref().and_then(|c| c.manufacturer.clone()),
        chassis_model: chassis.as_ref().and_then(|c| c.model.clone()),
        bios,
        security,
    })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BaseboardRow {
    manufacturer: Option<String>,
    product: Option<String>,
    version: Option<String>,
    serial_number: Option<String>,
}

fn read_baseboard() -> Result<Option<BaseboardRow>, InspectreError> {
    let con = WMIConnection::new()?;
    let rows: Vec<BaseboardRow> = con.raw_query(
        "SELECT Manufacturer, Product, Version, SerialNumber FROM Win32_BaseBoard",
    )?;
    Ok(rows.into_iter().next())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ComputerSystemRow {
    manufacturer: Option<String>,
    model: Option<String>,
}

fn read_chassis() -> Result<Option<ComputerSystemRow>, InspectreError> {
    let con = WMIConnection::new()?;
    let rows: Vec<ComputerSystemRow> =
        con.raw_query("SELECT Manufacturer, Model FROM Win32_ComputerSystem")?;
    Ok(rows.into_iter().next())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BiosRow {
    manufacturer: Option<String>,
    smbios_bios_version: Option<String>,
    smbios_major_version: Option<u16>,
    smbios_minor_version: Option<u16>,
    release_date: Option<String>,
}

fn read_bios() -> Result<Option<BiosRow>, InspectreError> {
    let con = WMIConnection::new()?;
    let rows: Vec<BiosRow> = con.raw_query(
        "SELECT Manufacturer, SMBIOSBIOSVersion, SMBIOSMajorVersion, SMBIOSMinorVersion, ReleaseDate FROM Win32_BIOS",
    )?;
    Ok(rows.into_iter().next())
}

fn smbios_version_string(b: &BiosRow) -> Option<String> {
    match (b.smbios_major_version, b.smbios_minor_version) {
        (Some(major), Some(minor)) => Some(format!("{major}.{minor}")),
        _ => None,
    }
}

fn read_firmware_type() -> FirmwareType {
    let mut value = SystemInformation::FIRMWARE_TYPE(0);
    let result = unsafe { SystemInformation::GetFirmwareType(&mut value) };
    if result.is_err() {
        return FirmwareType::Unknown;
    }
    match value.0 {
        1 => FirmwareType::Bios,
        2 => FirmwareType::Uefi,
        _ => FirmwareType::Unknown,
    }
}

fn read_security() -> SecurityInfo {
    let secure_boot = read_secure_boot().unwrap_or_else(|err| {
        tracing::debug!(?err, "Secure Boot indisponível");
        None
    });

    let tpm = read_tpm().unwrap_or_else(|err| {
        tracing::debug!(?err, "TPM indisponível");
        None
    });

    SecurityInfo {
        secure_boot,
        tpm_present: tpm.is_some(),
        tpm_enabled: tpm.as_ref().and_then(|t| t.is_enabled_initial_value),
        tpm_spec_version: tpm.as_ref().and_then(|t| t.spec_version.clone()),
    }
}

fn read_secure_boot() -> Result<Option<bool>, InspectreError> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm
        .open_subkey(r"SYSTEM\CurrentControlSet\Control\SecureBoot\State")
        .map_err(|e| InspectreError::Internal(e.to_string()))?;
    let value: u32 = key
        .get_value("UEFISecureBootEnabled")
        .map_err(|e| InspectreError::Internal(e.to_string()))?;
    Ok(Some(value == 1))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TpmRow {
    is_enabled_initial_value: Option<bool>,
    spec_version: Option<String>,
}

fn read_tpm() -> Result<Option<TpmRow>, InspectreError> {
    let con = WMIConnection::with_namespace_path("ROOT\\CIMV2\\Security\\MicrosoftTpm")?;
    let rows: Vec<TpmRow> =
        con.raw_query("SELECT IsEnabled_InitialValue, SpecVersion FROM Win32_Tpm")?;
    Ok(rows.into_iter().next())
}
