use crate::{
    plugins::window::{hide_window, show_window},
    window::MAIN_WINDOW_LABEL,
};
use tauri::{
    async_runtime, App, AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};

pub fn menu(app: &App) -> SystemTray {
    let version = app.package_info().version.to_string();
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("version".to_string(), version))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("hidden".to_string(), "隐藏"))
        .add_item(CustomMenuItem::new("screen-center".to_string(), "居中显示"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("exit".to_string(), "退出"));

    SystemTray::new()
        .with_menu(tray_menu)
        .with_tooltip("Sys Speed")
}
pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    async_runtime::block_on(async {
        let window = app.get_window(MAIN_WINDOW_LABEL).unwrap();

        match event {
            SystemTrayEvent::LeftClick { .. } => window.emit("tray-click", true).unwrap(),
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "vsersion" => {}
                "hidden" => {
                    window.emit("hidden", true).unwrap();
                    hide_window(window).await
                }
                "screen-center" => {
                    window.emit("screen-center", true).unwrap();
                    show_window(window).await
                }
                "exit" => app.exit(0),
                _ => {}
            },
            _ => {}
        }
    })
}
