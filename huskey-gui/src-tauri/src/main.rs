// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod errors;

use crate::errors::AppError;

use std::{
    borrow::BorrowMut,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
};

use huskey_lib::{
    create_or_read_db,
    database::{Database, PasswordEntry},
    decrypt_db, encrypt_and_save_db,
};
impl Default for OpenedDatabase {
    fn default() -> Self {
        return OpenedDatabase(Mutex::new(None));
    }
}
struct OpenedDatabase(Mutex<Option<Database>>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn open_database(
    path: &str,
    password: String,
    opened_database: tauri::State<OpenedDatabase>,
) -> Result<Database, AppError> {
    println!("Trying to open db at {}", path);
    let input_path: PathBuf = PathBuf::from(path);

    let encrypted_db = create_or_read_db(&input_path)?;

    let decrypted_db = decrypt_db(encrypted_db, password.clone());

    /*     if decrypted_db.is_ok() {
        println!("is ok");
        let mut new_db = decrypted_db.unwrap();
        new_db.add_password(PasswordEntry {
            name: "My google acc".to_string(),
            username: "myaccount-52".to_string(),
            password: "passSecret_*".to_string(),
            url: Some("https://www.google.com".to_string()),
        });
        huskey_lib::encrypt_and_save_db(&new_db, password.clone(), &input_path, None).unwrap();
        return Ok(new_db);
    } */
    //encrypt_and_save_db(&decrypted_db, password, &input_path, None);
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
fn save_database(
    path: &str,
    password: String,
    maybe_database: tauri::State<OpenedDatabase>,
) -> Option<AppError> {
    let mut mutx = maybe_database.0.lock().unwrap();
    match mutx.deref() {
        Some(db) => {
            let path = Path::new(path);
            encrypt_and_save_db(db, password, path, Some(db.pbdkf2_rounds)).unwrap();
            return None;
        }
        None => return Some(AppError::NoDatabaseOpened),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(OpenedDatabase::default())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![open_database])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
