use tauri::{SystemTray, SystemTrayEvent, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri::AppHandle;
use tauri::Manager;

pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("hide".to_string(), "Hide Window"))
        .add_item(CustomMenuItem::new("show".to_string(), "Show Window"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit Playgram"));

    SystemTray::new().with_menu(tray_menu)
}

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    let window = app.get_window("main").unwrap();
    // let parent_window = Some(&window);
    
    match event {
        
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
        }
        
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }

        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "show" => {
                window.show().unwrap();
            }
            "hide" => {
                window.hide().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}
