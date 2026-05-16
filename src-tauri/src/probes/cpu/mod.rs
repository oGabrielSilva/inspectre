#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod codenames;
mod types;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use types::CacheLevel;
pub use types::{CacheInfo, CpuInfo, CpuLiveTick};

use std::time::Duration;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use raw_cpuid::{native_cpuid::CpuIdReaderNative, CacheType, CpuId};
#[cfg(windows)]
use serde::Deserialize;
use sysinfo::{System, MINIMUM_CPU_UPDATE_INTERVAL};
#[cfg(windows)]
use wmi::WMIConnection;

use crate::error::InspectreError;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type NativeCpuId = CpuId<CpuIdReaderNative>;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn read_snapshot() -> Result<CpuInfo, InspectreError> {
    let mut sys = System::new_all();
    sys.refresh_cpu_all();
    let threads = sys.cpus().len() as u16;
    let cores = System::physical_core_count()
        .map(|c| c as u16)
        .unwrap_or(threads);
    let name = sys
        .cpus()
        .first()
        .map(|c| c.brand().trim().to_string())
        .unwrap_or_else(|| "Desconhecido".to_string());
    let vendor = sys
        .cpus()
        .first()
        .map(|c| c.vendor_id().trim().to_string())
        .unwrap_or_else(|| "Desconhecido".to_string());

    Ok(CpuInfo {
        name,
        vendor,
        codename: None,
        socket: None,
        process_nm: None,
        revision: None,
        family: 0,
        model: 0,
        stepping: 0,
        ext_family: 0,
        ext_model: 0,
        cores,
        threads,
        hyperthreading: cores > 0 && cores < threads,
        virtualization_supported: false,
        base_clock_mhz: None,
        max_clock_mhz: None,
        bus_clock_mhz: None,
        current_clock_mhz: average_current_mhz(&sys),
        cache: CacheInfo {
            l1_data: None,
            l1_inst: None,
            l2: None,
            l3: None,
        },
        instruction_sets: Vec::new(),
        tdp_w: None,
        boot_time_sec: System::boot_time() as u32,
    })
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn read_snapshot() -> Result<CpuInfo, InspectreError> {
    let cpuid: NativeCpuId = CpuId::new();

    let name = cpuid
        .get_processor_brand_string()
        .map(|b| b.as_str().trim().to_string())
        .unwrap_or_else(|| "Desconhecido".to_string());

    let vendor = cpuid
        .get_vendor_info()
        .map(|v| v.as_str().to_string())
        .unwrap_or_else(|| "Desconhecido".to_string());

    let (family, model, stepping, ext_family, ext_model) = cpuid
        .get_feature_info()
        .map(|f| {
            let display_family = if f.family_id() == 0x0F {
                f.family_id() + f.extended_family_id()
            } else {
                f.family_id()
            };
            let display_model = if f.family_id() == 0x06 || f.family_id() == 0x0F {
                (f.extended_model_id() << 4) | f.model_id()
            } else {
                f.model_id()
            };
            (
                display_family,
                display_model,
                f.stepping_id(),
                f.extended_family_id(),
                f.extended_model_id(),
            )
        })
        .unwrap_or((0, 0, 0, 0, 0));

    let virtualization_supported = cpuid
        .get_feature_info()
        .map(|f| f.has_vmx())
        .unwrap_or(false)
        || cpuid
            .get_extended_processor_and_feature_identifiers()
            .map(|e| e.has_svm())
            .unwrap_or(false);

    let codename = codenames::lookup(&vendor, family, model).map(String::from);
    let revision = build_revision(family, model, stepping);

    let instruction_sets = collect_instruction_sets(&cpuid);
    let cache = read_cache_info(&cpuid);

    let mut sys = System::new_all();
    sys.refresh_cpu_all();
    let threads = sys.cpus().len() as u16;
    let cores = System::physical_core_count()
        .map(|c| c as u16)
        .unwrap_or(threads);
    let hyperthreading = cores > 0 && cores < threads;
    let boot_time_sec = System::boot_time() as u32;

    #[cfg(windows)]
    let (base_clock_mhz, max_clock_mhz, bus_clock_mhz, socket) = read_processor_via_wmi()
        .unwrap_or_else(|err| {
            tracing::warn!(?err, "Win32_Processor indisponível");
            None
        })
        .map(|p| {
            (
                p.current_clock_speed,
                p.max_clock_speed,
                p.ext_clock,
                p.socket_designation,
            )
        })
        .unwrap_or((None, None, None, None));
    #[cfg(not(windows))]
    let (base_clock_mhz, max_clock_mhz, bus_clock_mhz, socket): (
        Option<u32>,
        Option<u32>,
        Option<u32>,
        Option<String>,
    ) = (None, None, None, None);

    let current_clock_mhz = average_current_mhz(&sys);

    Ok(CpuInfo {
        name,
        vendor,
        codename,
        socket,
        process_nm: None,
        revision,
        family,
        model,
        stepping,
        ext_family,
        ext_model,
        cores,
        threads,
        hyperthreading,
        virtualization_supported,
        base_clock_mhz,
        max_clock_mhz,
        bus_clock_mhz,
        current_clock_mhz,
        cache,
        instruction_sets,
        tdp_w: None,
        boot_time_sec,
    })
}

pub fn read_live_tick() -> CpuLiveTick {
    let mut sys = System::new();
    sys.refresh_cpu_all();
    std::thread::sleep(Duration::from_millis(
        MINIMUM_CPU_UPDATE_INTERVAL.as_millis() as u64,
    ));
    sys.refresh_cpu_all();

    let cpus = sys.cpus();
    let mhz_per_core: Vec<u32> = cpus.iter().map(|c| c.frequency() as u32).collect();
    let usage_per_core: Vec<f32> = cpus.iter().map(|c| c.cpu_usage()).collect();

    let mhz_avg = if mhz_per_core.is_empty() {
        None
    } else {
        let total: u64 = mhz_per_core.iter().map(|&v| v as u64).sum();
        let avg = total / mhz_per_core.len() as u64;
        if avg == 0 { None } else { Some(avg as u32) }
    };

    let usage_avg = if usage_per_core.is_empty() {
        0.0
    } else {
        usage_per_core.iter().sum::<f32>() / usage_per_core.len() as f32
    };

    #[cfg(windows)]
    let temperature_c = match read_temperature_via_wmi() {
        Ok(t) => Some(t),
        Err(err) => {
            tracing::debug!(?err, "temperatura ACPI indisponível");
            None
        }
    };
    #[cfg(not(windows))]
    let temperature_c: Option<f32> = None;

    CpuLiveTick {
        mhz_avg,
        mhz_per_core,
        usage_avg,
        usage_per_core,
        temperature_c,
    }
}

fn average_current_mhz(sys: &System) -> Option<u32> {
    let cpus = sys.cpus();
    if cpus.is_empty() {
        return None;
    }
    let total: u64 = cpus.iter().map(|c| c.frequency()).sum();
    let avg = total / cpus.len() as u64;
    if avg == 0 { None } else { Some(avg as u32) }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn build_revision(family: u8, model: u8, stepping: u8) -> Option<String> {
    if family == 0 && model == 0 && stepping == 0 {
        return None;
    }
    Some(format!(
        "Family {family:X}h, Model {model:X}h, Stepping {stepping:X}h"
    ))
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn collect_instruction_sets(cpuid: &NativeCpuId) -> Vec<String> {
    let mut sets = Vec::new();

    if let Some(features) = cpuid.get_feature_info() {
        if features.has_mmx() {
            sets.push("MMX".into());
        }
        if features.has_sse() {
            sets.push("SSE".into());
        }
        if features.has_sse2() {
            sets.push("SSE2".into());
        }
        if features.has_sse3() {
            sets.push("SSE3".into());
        }
        if features.has_ssse3() {
            sets.push("SSSE3".into());
        }
        if features.has_sse41() {
            sets.push("SSE4.1".into());
        }
        if features.has_sse42() {
            sets.push("SSE4.2".into());
        }
        if features.has_avx() {
            sets.push("AVX".into());
        }
        if features.has_aesni() {
            sets.push("AES-NI".into());
        }
        if features.has_fma() {
            sets.push("FMA3".into());
        }
    }

    if let Some(features) = cpuid.get_extended_feature_info() {
        if features.has_avx2() {
            sets.push("AVX2".into());
        }
        if features.has_avx512f() {
            sets.push("AVX-512F".into());
        }
        if features.has_bmi1() {
            sets.push("BMI1".into());
        }
        if features.has_bmi2() {
            sets.push("BMI2".into());
        }
        if features.has_sha() {
            sets.push("SHA".into());
        }
    }

    if let Some(features) = cpuid.get_extended_processor_and_feature_identifiers() {
        if features.has_64bit_mode() {
            sets.push("x86-64".into());
        }
    }

    sets
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn read_cache_info(cpuid: &NativeCpuId) -> CacheInfo {
    let mut info = CacheInfo {
        l1_data: None,
        l1_inst: None,
        l2: None,
        l3: None,
    };

    if let Some(params) = cpuid.get_cache_parameters() {
        for param in params {
            let ways = param.associativity() as u32;
            let line_size = param.coherency_line_size() as u32;
            let sets = param.sets() as u32;
            let partitions = param.physical_line_partitions() as u32;
            let size_bytes = (ways as u64) * (partitions as u64) * (line_size as u64) * (sets as u64);
            let level = CacheLevel {
                size_kb: (size_bytes / 1024) as u32,
                ways,
                line_size,
                sets,
            };

            match (param.level(), param.cache_type()) {
                (1, CacheType::Data) => info.l1_data = Some(level),
                (1, CacheType::Instruction) => info.l1_inst = Some(level),
                (2, _) => info.l2 = Some(level),
                (3, _) => info.l3 = Some(level),
                _ => {}
            }
        }
    }

    info
}

#[cfg(all(windows, any(target_arch = "x86", target_arch = "x86_64")))]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Win32Processor {
    current_clock_speed: Option<u32>,
    max_clock_speed: Option<u32>,
    ext_clock: Option<u32>,
    socket_designation: Option<String>,
}

#[cfg(all(windows, any(target_arch = "x86", target_arch = "x86_64")))]
fn read_processor_via_wmi() -> Result<Option<Win32Processor>, InspectreError> {
    let con = WMIConnection::new()?;
    let rows: Vec<Win32Processor> = con.raw_query(
        "SELECT CurrentClockSpeed, MaxClockSpeed, ExtClock, SocketDesignation FROM Win32_Processor",
    )?;
    Ok(rows.into_iter().next())
}

#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ThermalZoneTemperature {
    current_temperature: u32,
}

#[cfg(windows)]
fn read_temperature_via_wmi() -> Result<f32, InspectreError> {
    let con = WMIConnection::with_namespace_path("root\\WMI")?;
    let rows: Vec<ThermalZoneTemperature> =
        con.raw_query("SELECT CurrentTemperature FROM MSAcpi_ThermalZoneTemperature")?;
    rows.into_iter()
        .next()
        .map(|r| (r.current_temperature as f32 - 2731.5) / 10.0)
        .ok_or_else(|| InspectreError::SensorUnavailable {
            sensor: "MSAcpi_ThermalZoneTemperature".into(),
        })
}
