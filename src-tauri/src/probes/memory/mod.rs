mod types;

pub use types::{ChannelConfig, ChannelInfo, MemoryInfo, MemoryLiveTick, MemoryModule};

use std::collections::HashSet;

use serde::Deserialize;
use sysinfo::System;
use wmi::WMIConnection;

use crate::error::InspectreError;

pub fn read_snapshot() -> Result<MemoryInfo, InspectreError> {
    let modules = read_modules().unwrap_or_else(|err| {
        tracing::warn!(?err, "Win32_PhysicalMemory indisponível");
        Vec::new()
    });

    let array = read_memory_array().unwrap_or_else(|err| {
        tracing::warn!(?err, "Win32_PhysicalMemoryArray indisponível");
        None
    });

    let mut sys = System::new();
    sys.refresh_memory();
    let total_bytes = sys.total_memory();

    let slots_populated = modules.len() as u16;
    let slots_total = array
        .as_ref()
        .and_then(|a| a.memory_devices)
        .unwrap_or(slots_populated);

    let max_supported_bytes = array
        .as_ref()
        .and_then(|a| a.max_capacity_ex.or_else(|| a.max_capacity.map(|kb| kb as u64)))
        .map(|kb| kb * 1024);

    let ecc_supported = array
        .as_ref()
        .and_then(|a| a.memory_error_correction)
        .map(|code| matches!(code, 4..=6))
        .unwrap_or(false);

    let channels = detect_channels(&modules);

    Ok(MemoryInfo {
        total_bytes,
        max_supported_bytes,
        slots_total,
        slots_populated,
        ecc_supported,
        channels,
        modules,
    })
}

pub fn read_live_tick() -> MemoryLiveTick {
    let mut sys = System::new_all();
    sys.refresh_memory();
    let tick = MemoryLiveTick {
        used_bytes: sys.used_memory(),
        available_bytes: sys.available_memory(),
        total_bytes: sys.total_memory(),
        swap_used_bytes: sys.used_swap(),
        swap_total_bytes: sys.total_swap(),
    };
    tracing::trace!(?tick, "memory tick");
    tick
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PhysicalMemoryRow {
    capacity: Option<u64>,
    memory_type: Option<u16>,
    smbiosmemory_type: Option<u32>,
    form_factor: Option<u16>,
    speed: Option<u32>,
    configured_clock_speed: Option<u32>,
    manufacturer: Option<String>,
    part_number: Option<String>,
    serial_number: Option<String>,
    bank_label: Option<String>,
    device_locator: Option<String>,
    data_width: Option<u16>,
    total_width: Option<u16>,
    configured_voltage: Option<u32>,
}

fn read_modules() -> Result<Vec<MemoryModule>, InspectreError> {
    let con = WMIConnection::new()?;
    let rows: Vec<PhysicalMemoryRow> = con.raw_query(
        "SELECT Capacity, MemoryType, SMBIOSMemoryType, FormFactor, Speed, ConfiguredClockSpeed, \
         Manufacturer, PartNumber, SerialNumber, BankLabel, DeviceLocator, DataWidth, TotalWidth, \
         ConfiguredVoltage FROM Win32_PhysicalMemory",
    )?;

    Ok(rows.into_iter().map(row_to_module).collect())
}

fn row_to_module(row: PhysicalMemoryRow) -> MemoryModule {
    let memory_type = row
        .smbiosmemory_type
        .and_then(map_smbios_memory_type)
        .or_else(|| row.memory_type.and_then(map_cim_memory_type))
        .map(String::from);

    let form_factor = row.form_factor.and_then(map_form_factor).map(String::from);

    let voltage_v = row
        .configured_voltage
        .filter(|v| *v > 0)
        .map(|mv| mv as f32 / 1000.0);

    let slot = row
        .device_locator
        .clone()
        .unwrap_or_else(|| "Desconhecido".to_string());

    MemoryModule {
        slot,
        bank: row.bank_label,
        size_bytes: row.capacity.unwrap_or(0),
        memory_type,
        form_factor,
        speed_mhz: row.speed,
        configured_speed_mhz: row.configured_clock_speed,
        manufacturer: row.manufacturer.map(|m| m.trim().to_string()).filter(|s| !s.is_empty()),
        part_number: row.part_number.map(|m| m.trim().to_string()).filter(|s| !s.is_empty()),
        serial: row.serial_number.map(|m| m.trim().to_string()).filter(|s| !s.is_empty()),
        data_width: row.data_width,
        total_width: row.total_width,
        voltage_v,
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MemoryArrayRow {
    memory_devices: Option<u16>,
    max_capacity: Option<u32>,
    max_capacity_ex: Option<u64>,
    memory_error_correction: Option<u16>,
}

fn read_memory_array() -> Result<Option<MemoryArrayRow>, InspectreError> {
    let con = WMIConnection::new()?;
    let rows: Vec<MemoryArrayRow> = con.raw_query(
        "SELECT MemoryDevices, MaxCapacity, MaxCapacityEx, MemoryErrorCorrection \
         FROM Win32_PhysicalMemoryArray",
    )?;
    Ok(rows.into_iter().next())
}

fn detect_channels(modules: &[MemoryModule]) -> ChannelInfo {
    let mut detected_letters: HashSet<char> = HashSet::new();

    for m in modules {
        let upper = m.slot.to_uppercase();
        for letter in ['A', 'B', 'C', 'D'] {
            let needles = [
                format!("CHANNEL{letter}"),
                format!("CHANNEL {letter}"),
                format!("CHANNEL-{letter}"),
                format!("DIMM_{letter}"),
                format!("DIMM-{letter}"),
                format!("DIMM {letter}"),
                format!("CHA{letter}"),
            ];
            if needles.iter().any(|n| upper.contains(n)) {
                detected_letters.insert(letter);
                break;
            }
        }
    }

    let detected = detected_letters.len() as u8;
    let configuration = match detected {
        0 => match modules.len() {
            1 => ChannelConfig::Single,
            2 => ChannelConfig::Dual,
            3 => ChannelConfig::Triple,
            4 => ChannelConfig::Quad,
            _ => ChannelConfig::Unknown,
        },
        1 => ChannelConfig::Single,
        2 => ChannelConfig::Dual,
        3 => ChannelConfig::Triple,
        4 => ChannelConfig::Quad,
        _ => ChannelConfig::Unknown,
    };

    ChannelInfo {
        configuration,
        detected: detected.max(1),
    }
}

fn map_smbios_memory_type(code: u32) -> Option<&'static str> {
    match code {
        0x14 => Some("DDR"),
        0x15 => Some("DDR2"),
        0x16 => Some("DDR2 FB-DIMM"),
        0x18 => Some("DDR3"),
        0x19 => Some("FBD2"),
        0x1A => Some("DDR4"),
        0x1B => Some("LPDDR"),
        0x1C => Some("LPDDR2"),
        0x1D => Some("LPDDR3"),
        0x1E => Some("LPDDR4"),
        0x1F => Some("Logical NVDIMM"),
        0x20 => Some("HBM"),
        0x21 => Some("HBM2"),
        0x22 => Some("DDR5"),
        0x23 => Some("LPDDR5"),
        0x24 => Some("HBM3"),
        _ => None,
    }
}

fn map_cim_memory_type(code: u16) -> Option<&'static str> {
    match code {
        20 => Some("DDR"),
        21 => Some("DDR2"),
        22 => Some("DDR2 FB-DIMM"),
        24 => Some("DDR3"),
        25 => Some("FBD2"),
        26 => Some("DDR4"),
        _ => None,
    }
}

fn map_form_factor(code: u16) -> Option<&'static str> {
    match code {
        8 => Some("DIMM"),
        12 => Some("SODIMM"),
        11 => Some("RIMM"),
        13 => Some("SRIMM"),
        15 => Some("FB-DIMM"),
        _ => None,
    }
}
