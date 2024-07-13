// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod plugins;
mod tray;

use plugins::window::{self, show_window, MAIN_WINDOW_LABEL};
use sysinfo::{Networks, RefreshKind, System};
use tauri::Manager;
use tauri::{async_runtime, command, generate_context, generate_handler, Builder, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct SystemInfo {
    memory_usage: f32,
    cpu_usage: f32,
    network_speed_up: u64,
    network_speed_down: u64,
}

#[command]
fn get_sys_info() -> SystemInfo {
    let mut sys = System::new_with_specifics(RefreshKind::everything().without_processes());
    sys.refresh_all();

    // Raw
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_usage = used_memory as f32 / total_memory as f32 * 100.0;

    // cpu 占用
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu();
    let cpu_usage =
        sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

    let mut networks = Networks::new_with_refreshed_list();
    networks.refresh_list();
    let network_speed_up = networks
        .iter()
        .map(|(_, data)| data.transmitted())
        .sum::<u64>()
        / networks.len() as u64;
    let network_speed_down = networks
        .iter()
        .map(|(_, data)| data.received())
        .sum::<u64>()
        / networks.len() as u64;

    SystemInfo {
        memory_usage,
        cpu_usage,
        network_speed_up,
        network_speed_down,
    }
}

fn main() {
    Builder::default()
        .setup(|app| {
            let window = app.get_window(MAIN_WINDOW_LABEL).unwrap();
            #[cfg(debug_assertions)]
            {
                window.open_devtools();
            }

            window_shadows::set_shadow(&window, true).unwrap();
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            let window = app.get_window(MAIN_WINDOW_LABEL).unwrap();

            async_runtime::block_on(async move {
                show_window(window).await;
            });
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        // 自定义的窗口管理插件
        .plugin(window::init())
        // 系统托盘：https://tauri.app/v1/guides/features/system-tray
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(generate_handler![get_sys_info])
        // 让 app 保持在后台运行：https://tauri.app/v1/guides/features/system-tray/#preventing-the-app-from-closing
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}
