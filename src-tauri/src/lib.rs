use std::path::PathBuf;
use tauri::App;
use tauri::Manager;

struct AppState {
    db_path: PathBuf,
}

fn get_db_path(app: &App) -> PathBuf {
    let app_local_data_dir = app
        .path()
        .app_local_data_dir()
        .expect("Could not find local data directory!");
    log::debug!("Local data directory: {}", app_local_data_dir.display());
    let app_name = &app.package_info().name;
    let db_path = PathBuf::new()
        .join(app_local_data_dir)
        .join(app_name)
        .with_extension("db");
    log::debug!("Database path: {}", db_path.display());

    db_path
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            log::debug!("Logging is now enabled");

            app.manage(AppState {
                db_path: get_db_path(app),
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
