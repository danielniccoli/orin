use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use rusqlite::Connection;
use rusqlite_migration::Migrations;
use std::path::PathBuf;
use tauri::App;
use tauri::Manager;

struct AppState {
    db_path: PathBuf,
}

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");
lazy_static! {
    static ref MIGRATIONS: Migrations<'static> =
        Migrations::from_directory(&MIGRATIONS_DIR).unwrap();
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

fn open_db(state: tauri::State<AppState>) -> Connection {
    let db_path = &state.db_path;
    let err_msg = std::format!(
        "Could not open or create the database at {}",
        db_path.display()
    );

    Connection::open(db_path).expect(&err_msg)
}

/// Runs database migrations. Invoked from the frontend.
#[tauri::command]
fn run_migrations(state: tauri::State<AppState>) -> Result<(), String> {
    // TODO: Return the error with the original error type and handle it in the frontend
    if let Err(msg) = MIGRATIONS.to_latest(&mut open_db(state)) {
        log::error!("{:#?}", msg);
        return Err(msg.to_string());
    }

    Ok(())
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
        .invoke_handler(tauri::generate_handler![run_migrations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
