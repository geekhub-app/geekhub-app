#[tauri::command]
fn open_new_window(url: String) -> Result<(), String> {
    tauri::async_runtime::spawn(async move {
        let window = tauri::WindowBuilder::new()
            .title("New Window")
            .build(tauri::generate_context!())
            .unwrap();
        window.load_url(&url).unwrap();
    });
    Ok(())
}
