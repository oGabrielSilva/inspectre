mod commands;
mod error;
mod log;
mod probes;

use commands::cpu::CpuLiveState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    log::init();

    tauri::Builder::default()
        .manage(CpuLiveState::default())
        .invoke_handler(tauri::generate_handler![
            commands::cpu::cpu_info,
            commands::cpu::start_cpu_live_stream,
            commands::cpu::stop_cpu_live_stream,
        ])
        .run(tauri::generate_context!())
        .expect("falha ao iniciar Tauri");
}
