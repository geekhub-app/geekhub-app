#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod tray;

use tauri::Manager;
use window_shadows::set_shadow;

fn main() {
    tauri::Builder::default()
        // .system_tray(tauri::SystemTray::default())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        // .menu(tauri::Menu::os_default(&context.package_info().name))
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .on_window_event(|event| {
          if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
              event.window().minimize().unwrap();
              api.prevent_close();
          }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
