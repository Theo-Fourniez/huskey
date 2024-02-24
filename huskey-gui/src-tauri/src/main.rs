// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod errors;

use crate::errors::AppError;

use std::{
    ops::Deref,
    path::{Path, PathBuf},
    sync::Mutex,
};

use huskey_lib::{
    create_db,
    database::{Database, PasswordEntry},
    decrypt_db, encrypt_and_save_db, read_db,
};
impl Default for OpenedDatabase {
    fn default() -> Self {
        return OpenedDatabase {
            database: Mutex::new(None),
        };
    }
}

struct OpenedDatabase {
    database: Mutex<Option<Database>>,
}

#[tauri::command]
async fn open_database(
    path: &str,
    password: String,
    opened_database: tauri::State<'_, OpenedDatabase>,
) -> Result<Database, AppError> {
    println!("Trying to open db at {}", path);
    let input_path: PathBuf = PathBuf::from(path);

    let encrypted_db = read_db(&input_path)?;
    let decrypted_db = decrypt_db(encrypted_db, password.clone());

    match decrypted_db {
        Ok(db) => {
            let mut d = opened_database.database.lock().unwrap();
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
    let mutx = maybe_database.database.lock().unwrap();
    match mutx.deref() {
        Some(db) => {
            let path = Path::new(path);
            encrypt_and_save_db(db, password, path, Some(db.pbkdf2_rounds))?;
            return Ok(());
        }
        None => return Err(AppError::NoDatabaseOpened),
    }
}

#[tauri::command]
async fn close_database(opened_database: tauri::State<'_, OpenedDatabase>) -> Result<(), AppError> {
    let mut d = opened_database.database.lock().unwrap();
    if (*d).is_none() {
        return Err(AppError::NoDatabaseOpened);
    }
    *d = None;
    Ok(())
}

#[tauri::command]
async fn add_password_entry(
    entry: PasswordEntry,
    opened_database: tauri::State<'_, OpenedDatabase>,
) -> Result<Database, AppError> {
    let mut d = opened_database.database.lock().unwrap();
    match d.deref().clone() {
        Some(mut db) => {
            db.add_password(entry);
            *d = Some(db.clone());
            return Ok(db);
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
