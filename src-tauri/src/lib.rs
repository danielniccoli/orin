use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use rusqlite::Connection;
use rusqlite::DatabaseName;
use rusqlite_migration::Migrations;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::os::windows::prelude::*;
use std::path::PathBuf;
use tauri::ipc::Response;
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

#[tauri::command]
fn get_document(state: tauri::State<AppState>) -> Result<Response, String> {
    //fn get_document(state: tauri::State<AppState>) -> Result<Vec<Response>, String> {
    let db = open_db(state);
    let mut stmt = db.prepare("SELECT document FROM documents").unwrap();
    let document_iter = stmt
        .query_map([], |row| {
            let document: Vec<u8> = row.get(0).unwrap();
            Ok(document)
        })
        .unwrap();

    let fields: Vec<Vec<u8>> = document_iter.map(|s| s.unwrap()).collect();
    //Ok(Response::new(fields[0].clone()))
    Ok(Response::new(fields[0].clone()))
}

#[tauri::command]
fn store_file(file_path: String, state: tauri::State<AppState>) -> Result<(), String> {
    log::debug!("File path: {}", file_path);
    let mut file = {
        let result: Result<_, _>;
        // On Windows, do not allow other processes to modify this file while we have it open
        if cfg!(windows) {
            result = OpenOptions::new().read(true).share_mode(1).open(file_path);
        } else {
            result = File::open(file_path);
        }
        match result {
            Ok(file) => file,
            Err(msg) => {
                log::error!("{:#?}", msg);
                return Err(msg.to_string());
            }
        }
    };
    let file_size = file.metadata().expect("Unable to read file metadata").len();
    log::debug!("File size: {}", file_size);

    let mut db = open_db(state);
    let db = match db.transaction() {
        Ok(transaction) => transaction,
        Err(msg) => {
            log::error!("{:#?}", msg);
            return Err(msg.to_string());
        }
    };

    if let Err(msg) = db.execute(
        "INSERT INTO documents (document) VALUES (ZEROBLOB(?1))",
        [file_size],
    ) {
        log::error!("{:#?}", msg);
        return Err(msg.to_string());
    };

    let rowid = db.last_insert_rowid();

    let mut blob = match db.blob_open(DatabaseName::Main, "documents", "document", rowid, false) {
        Ok(blob) => blob,
        Err(msg) => {
            log::error!("{:#?}", msg);
            return Err(msg.to_string());
        }
    };

    let mut buffer = vec![0; 1 << 20]; // = 1 MiB
    loop {
        let bytes_read = file.read(&mut buffer).expect("buffer should be readable");
        if bytes_read == 0 {
            break;
        }
        blob.write_all(&buffer[..bytes_read])
            .expect("blob should be writable");
    }

    // TODO: Why does db.commit() throw a problem at me, when I remove blob.close()?
    blob.close().unwrap();
    db.commit().unwrap();

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
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            run_migrations,
            store_file,
            get_document
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
