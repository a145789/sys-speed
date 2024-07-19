// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod plugins;
mod tray;

use plugins::system_info;
use plugins::window::{self, show_window, MAIN_WINDOW_LABEL};
use tauri::Manager;
use tauri::{async_runtime, generate_context, Builder, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;
use tray::{handler, menu};

fn main() {
    Builder::default()
        .setup(|app| {
            let window = app.get_window(MAIN_WINDOW_LABEL).unwrap();
            #[cfg(debug_assertions)]
            {
                window.open_devtools();
            }

            window_shadows::set_shadow(&window, true).unwrap();
            let _ = menu(app).build(app)?;

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
        .plugin(system_info::init())
        // // 系统托盘：https://tauri.app/v1/guides/features/system-tray
        // .system_tray(tray::menu())
        .on_system_tray_event(handler)
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
