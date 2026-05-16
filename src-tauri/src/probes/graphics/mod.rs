mod types;

#[allow(unused_imports)]
pub use types::{GpuKind, GpuVendor, GraphicsAdapter, GraphicsInfo, Resolution};

#[cfg(windows)]
use serde::Deserialize;
#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory1, IDXGIFactory1, DXGI_ADAPTER_FLAG_SOFTWARE,
};
#[cfg(windows)]
use wmi::WMIConnection;

use crate::error::InspectreError;
#[cfg(windows)]
use crate::util::format_cim_date;

#[cfg(not(windows))]
pub fn read_snapshot() -> Result<GraphicsInfo, InspectreError> {
    Ok(GraphicsInfo {
        adapters: Vec::new(),
    })
}

#[cfg(windows)]
pub fn read_snapshot() -> Result<GraphicsInfo, InspectreError> {
    let dxgi_adapters = enumerate_dxgi_adapters().unwrap_or_else(|err| {
        tracing::warn!(?err, "falha ao enumerar DXGI adapters");
        Vec::new()
    });

    let wmi_controllers = read_video_controllers().unwrap_or_else(|err| {
        tracing::warn!(?err, "Win32_VideoController indisponível");
        Vec::new()
    });

    let adapters = dxgi_adapters
        .into_iter()
        .map(|dxgi| merge_adapter(dxgi, &wmi_controllers))
        .collect();

    Ok(GraphicsInfo { adapters })
}

#[cfg(windows)]
#[derive(Debug)]
struct DxgiAdapter {
    name: String,
    vendor_id: u32,
    device_id: u32,
    subsys_id: u32,
    revision: u32,
    dedicated_memory_bytes: u64,
    shared_memory_bytes: u64,
    is_software: bool,
}

#[cfg(windows)]
fn enumerate_dxgi_adapters() -> Result<Vec<DxgiAdapter>, InspectreError> {
    let mut adapters = Vec::new();
    unsafe {
        let factory: IDXGIFactory1 =
            CreateDXGIFactory1().map_err(|e| InspectreError::Internal(e.to_string()))?;
        let mut index: u32 = 0;
        while let Ok(adapter) = factory.EnumAdapters1(index) {
            if let Ok(desc) = adapter.GetDesc1() {
                let name = decode_wide(&desc.Description);
                let is_software = (desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32) != 0;

                adapters.push(DxgiAdapter {
                    name,
                    vendor_id: desc.VendorId,
                    device_id: desc.DeviceId,
                    subsys_id: desc.SubSysId,
                    revision: desc.Revision,
                    dedicated_memory_bytes: desc.DedicatedVideoMemory as u64,
                    shared_memory_bytes: desc.SharedSystemMemory as u64,
                    is_software,
                });
            }
            index += 1;
        }
    }
    Ok(adapters)
}

#[cfg(windows)]
fn decode_wide(buf: &[u16]) -> String {
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..end]).trim().to_string()
}

#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct VideoControllerRow {
    name: Option<String>,
    driver_version: Option<String>,
    driver_date: Option<String>,
    current_horizontal_resolution: Option<u32>,
    current_vertical_resolution: Option<u32>,
    current_refresh_rate: Option<u32>,
    video_processor: Option<String>,
}

#[cfg(windows)]
fn read_video_controllers() -> Result<Vec<VideoControllerRow>, InspectreError> {
    let con = WMIConnection::new()?;
    let rows: Vec<VideoControllerRow> = con.raw_query(
        "SELECT Name, DriverVersion, DriverDate, CurrentHorizontalResolution, \
         CurrentVerticalResolution, CurrentRefreshRate, VideoProcessor FROM Win32_VideoController",
    )?;
    Ok(rows)
}

#[cfg(windows)]
fn merge_adapter(dxgi: DxgiAdapter, controllers: &[VideoControllerRow]) -> GraphicsAdapter {
    let key = dxgi.name.to_lowercase();
    let matched = controllers
        .iter()
        .find(|c| c.name.as_deref().map(|n| n.to_lowercase()) == Some(key.clone()));

    let current_resolution = matched.and_then(|c| {
        match (c.current_horizontal_resolution, c.current_vertical_resolution) {
            (Some(w), Some(h)) if w > 0 && h > 0 => Some(Resolution {
                width: w,
                height: h,
            }),
            _ => None,
        }
    });

    let refresh_rate_hz = matched.and_then(|c| c.current_refresh_rate).filter(|&v| v > 0);
    let driver_version = matched.and_then(|c| c.driver_version.clone());
    let driver_date = matched
        .and_then(|c| c.driver_date.as_deref())
        .and_then(format_cim_date);
    let video_processor = matched.and_then(|c| c.video_processor.clone());

    let vendor = classify_vendor(dxgi.vendor_id);
    let kind = classify_kind(&dxgi, vendor);

    GraphicsAdapter {
        name: dxgi.name,
        vendor,
        vendor_id: dxgi.vendor_id,
        device_id: dxgi.device_id,
        subsys_id: dxgi.subsys_id,
        revision: dxgi.revision,
        kind,
        dedicated_memory_bytes: dxgi.dedicated_memory_bytes,
        shared_memory_bytes: dxgi.shared_memory_bytes,
        driver_version,
        driver_date,
        video_processor,
        current_resolution,
        refresh_rate_hz,
    }
}

#[cfg(windows)]
fn classify_vendor(vendor_id: u32) -> GpuVendor {
    match vendor_id {
        0x10DE => GpuVendor::Nvidia,
        0x1002 | 0x1022 => GpuVendor::Amd,
        0x8086 => GpuVendor::Intel,
        0x1414 => GpuVendor::Microsoft,
        0x5143 | 0x4D4F4351 => GpuVendor::Qualcomm,
        _ => GpuVendor::Other,
    }
}

#[cfg(windows)]
fn classify_kind(dxgi: &DxgiAdapter, vendor: GpuVendor) -> GpuKind {
    if dxgi.is_software || vendor == GpuVendor::Microsoft {
        return GpuKind::Software;
    }
    if dxgi.dedicated_memory_bytes < 512 * 1024 * 1024 && vendor == GpuVendor::Intel {
        return GpuKind::Integrated;
    }
    if dxgi.dedicated_memory_bytes >= 1024 * 1024 * 1024 {
        return GpuKind::Dedicated;
    }
    GpuKind::Unknown
}
