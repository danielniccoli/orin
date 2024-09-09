#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            log::debug!("Logging is now enabled");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
