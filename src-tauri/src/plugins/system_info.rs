use std::sync::Mutex;
use sysinfo::{Networks, RefreshKind, System};

use tauri::{
    command, generate_handler,
    plugin::{Builder, TauriPlugin},
    Manager, Wry,
};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    memory_usage: f32,
    cpu_usage: f32,
    network_speed_up: u64,
    network_speed_down: u64,
}

pub struct SysInfoState {
    pub sysinfo: Mutex<SysInfo>,
}
/// A Mute Wrapper for sysinfo crate's System struct
pub struct SysInfo {
    pub sys: System,
}

impl Default for SysInfoState {
    fn default() -> SysInfoState {
        SysInfoState {
            sysinfo: Mutex::new(SysInfo {
                sys: System::new_with_specifics(RefreshKind::everything().without_processes()),
            }),
        }
    }
}

fn get_wlan_data(networks: &Networks) -> (u64, u64) {
    for (interface_name, data) in networks {
        if interface_name == "WLAN" {
            return (data.received(), data.transmitted());
        }
    }
    println!("WLAN interface not found");
    // If WLAN interface is not found, return 0 for both received and transmitted data.
    (0, 0)
}

#[command]
pub fn get_sys_info(state: tauri::State<'_, SysInfoState>) -> SystemInfo {
    // 锁定 sysinfo 并将其绑定到一个变量上
    let mut sysinfo_guard = state.sysinfo.lock().unwrap();

    sysinfo_guard.sys.refresh_memory();
    let total_memory = sysinfo_guard.sys.total_memory();
    let used_memory = sysinfo_guard.sys.used_memory();
    let memory_usage = used_memory as f32 / total_memory as f32 * 100.0;

    // // cpu 占用
    sysinfo_guard.sys.refresh_cpu();

    let cpu_usage = sysinfo_guard
        .sys
        .cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .sum::<f32>()
        / sysinfo_guard.sys.cpus().len() as f32;

    let mut networks = Networks::new_with_refreshed_list();
    networks.refresh_list();

    let (network_speed_down, network_speed_up) = get_wlan_data(&networks);

    SystemInfo {
        memory_usage,
        cpu_usage,
        network_speed_up,
        network_speed_down,
    }
}

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("system_info")
        .setup(move |app| {
            app.manage(SysInfoState::default());

            Ok(())
        })
        .invoke_handler(generate_handler![get_sys_info,])
        .build()
}
