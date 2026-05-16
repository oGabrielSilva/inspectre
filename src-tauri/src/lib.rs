mod commands;
mod error;
mod log;
mod probes;
mod util;

use commands::cpu::CpuLiveState;
use commands::memory::MemoryLiveState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    log::init();

    tauri::Builder::default()
        .manage(CpuLiveState::default())
        .manage(MemoryLiveState::default())
        .invoke_handler(tauri::generate_handler![
            commands::bench::run_bench,
            commands::cpu::cpu_info,
            commands::cpu::start_cpu_live_stream,
            commands::cpu::stop_cpu_live_stream,
            commands::graphics::graphics_info,
            commands::mainboard::mainboard_info,
            commands::memory::memory_info,
            commands::memory::start_memory_live_stream,
            commands::memory::stop_memory_live_stream,
        ])
        .run(tauri::generate_context!())
        .expect("falha ao iniciar Tauri");
}
