// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod errors;

use crate::errors::AppError;

use std::{
    ops::Deref,
    path::{Path, PathBuf},
    sync::Mutex,
};

use huskey_lib::{create_or_read_db, database::Database, decrypt_db, encrypt_and_save_db};
impl Default for OpenedDatabase {
    fn default() -> Self {
        return OpenedDatabase(Mutex::new(None));
    }
}
struct OpenedDatabase(Mutex<Option<Database>>);

#[tauri::command]
async fn open_database(
    path: &str,
    password: String,
    opened_database: tauri::State<'_, OpenedDatabase>,
) -> Result<Database, AppError> {
    println!("Trying to open db at {}", path);
    let input_path: PathBuf = PathBuf::from(path);

    let encrypted_db = create_or_read_db(&input_path)?;
    let decrypted_db = decrypt_db(encrypted_db, password.clone());

    match decrypted_db {
        Ok(db) => {
            let mut d = opened_database.0.lock().unwrap();
            *d = Some(db.clone());
            return Ok(db);
        }
        Err(e) => {
            println!("Error: {}", e);
            return Err(e.into());
        }
    };
}

#[tauri::command]
async fn save_database(
    path: &str,
    password: String,
    maybe_database: tauri::State<'_, OpenedDatabase>,
) -> Result<(), AppError> {
    let mutx = maybe_database.0.lock().unwrap();
    match mutx.deref() {
        Some(db) => {
            let path = Path::new(path);
            encrypt_and_save_db(db, password, path, Some(db.pbdkf2_rounds))?;
            return Ok(());
        }
        None => return Err(AppError::NoDatabaseOpened),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(OpenedDatabase::default())
        .invoke_handler(tauri::generate_handler![open_database, save_database])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
