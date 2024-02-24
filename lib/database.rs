use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, Read, Write},
    path::Path,
};

use pbkdf2::password_hash::SaltString;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use serde_json::error::Category;
use thiserror::Error;

use crate::{
    encryption::{decrypt, encrypt, generate_nonce, FixedNonce},
    key::MasterKey,
};

#[derive(Serialize, Deserialize)]
pub struct EncryptedDatabase {
    pub(crate) encryption_params: RawDatabaseEncryptionParams,
    entries: Vec<u8>,
}

#[derive(Debug, Serialize, Error)] // serialize needed for frontend
pub enum DatabaseError {
    #[error("Could not encrypt the database")]
    EncryptError,
    #[error(
        "Could not decrypt the database, file may have been tampered with or the password is wrong"
    )]
    DecryptError,
    #[error("Could not serialize the database to JSON : {0}")]
    DatabaseSerializationError(String),
    #[error("Could not deserialize the database from JSON : {0}")]
    DatabaseDeserializationError(String),
    #[error("Could not write to the database file : {0}")]
    WriteError(String),
    #[error("Could not read the database file : {0}")]
    ReadError(String),
}

impl From<serde_json::Error> for DatabaseError {
    fn from(e: serde_json::Error) -> Self {
        match e.classify() {
            Category::Data => {
                return DatabaseError::DatabaseDeserializationError(format!(
                    "The encrypted database file contains invalid JSON data at {}:{}",
                    e.line(),
                    e.column()
                ))
            }
            Category::Syntax => {
                return DatabaseError::DatabaseDeserializationError(format!(
                    "The encrypted database file contains invalid JSON syntax at {}:{}",
                    e.line(),
                    e.column()
                ))
            }
            _ => return DatabaseError::DatabaseDeserializationError(e.to_string()),
        }
    }
}

#[derive(Serialize, Clone)] // serialize is needed for the frontend; maybe there is a better way to do this
pub struct Database {
    #[serde(skip)]
    pub pbkdf2_rounds: u32,
    pub(crate) entries: Vec<PasswordEntry>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RawDatabaseEncryptionParams {
    nonce: FixedNonce,
    pub(crate) salt: String,
    pub(crate) pbkdf2_rounds: u32,
}

impl Default for EncryptedDatabase {
    fn default() -> Self {
        EncryptedDatabase {
            encryption_params: RawDatabaseEncryptionParams {
                nonce: generate_nonce().into(),
                salt: SaltString::generate(&mut OsRng).to_string(),
                pbkdf2_rounds: 250_000,
            },
            entries: Vec::new(),
        }
    }
}

impl EncryptedDatabase {
    pub fn decrypt(&self, secret_key: String) -> Result<Database, DatabaseError> {
        if self.entries.is_empty() {
            return Ok(Database::default());
        }
        let decrypted_entries = decrypt(
            &self.entries,
            secret_key.as_bytes().to_vec(),
            &self.encryption_params.nonce.clone().into(),
        );

        match decrypted_entries {
            Ok(raw_entries) => {
                let entries: Vec<PasswordEntry> =
                    serde_json::from_slice(&raw_entries).map_err(|e| {
                        DatabaseError::DatabaseDeserializationError(
                            serde_json::Error::from(e).to_string(),
                        )
                    })?;
                Ok(Database {
                    entries,
                    pbkdf2_rounds: self.encryption_params.pbkdf2_rounds,
                })
            }
            Err(_) => Err(DatabaseError::DecryptError),
        }
    }
}

impl Database {
    fn encrypt(
        &self,
        params: &DatabaseEncryptionParams,
        new_nonce: [u8; 12],
    ) -> Result<EncryptedDatabase, DatabaseError> {
        let entries_as_bytes = serde_json::to_vec(&self.entries).map_err(|e| {
            DatabaseError::DatabaseSerializationError(serde_json::Error::from(e).to_string())
        })?;
        let result = encrypt(
            &entries_as_bytes,
            params.secret_key.as_bytes().to_vec(),
            &new_nonce.into(),
        );
        match result {
            Ok(raw_entries) => Ok(EncryptedDatabase {
                encryption_params: RawDatabaseEncryptionParams {
                    nonce: new_nonce.into(),
                    salt: params.pbkdf2_salt.to_string(),
                    pbkdf2_rounds: params.pbkdf2_rounds,
                },
                entries: raw_entries,
            }),
            Err(_) => Err(DatabaseError::EncryptError),
        }
    }

    pub(crate) fn write_to_disk(
        &self,
        path: &Path,
        encrypt_params: &DatabaseEncryptionParams,
    ) -> Result<(), DatabaseError> {
        match try_open_truncate_from_path(&path) {
            Ok(mut db_file) => {
                let encrypted_db: EncryptedDatabase = self
                    .encrypt(&encrypt_params, generate_nonce().into())
                    .map_err(|_e| DatabaseError::EncryptError)?;

                let encrypted_db_as_bytes = serde_json::to_vec_pretty(&encrypted_db).unwrap();
                db_file.write_all(&encrypted_db_as_bytes).map_err(|e| {
                    DatabaseError::WriteError(format!(
                        "Could not write to the database file: {}",
                        e
                    ))
                })?;
                Ok(())
            }
            Err(e) => Err(DatabaseError::ReadError(format!(
                "Could not read the database file: {}",
                e
            ))),
        }
    }

    pub fn add_password(&mut self, password: PasswordEntry) {
        self.entries.push(password)
    }

    pub fn get_entries(self) -> Vec<PasswordEntry> {
        self.entries
    }

    fn get_entry_index(&self, entry: &PasswordEntry) -> Option<usize> {
        self.entries.iter().position(|x| x == entry)
    }

    pub fn remove_entry(&mut self, to_remove: &PasswordEntry) -> Option<PasswordEntry> {
        self.get_entry_index(to_remove)
            .map(|index| self.entries.swap_remove(index))
    }

    pub fn replace_entry(&mut self, to_replace: PasswordEntry, new_value: PasswordEntry) -> () {
        self.remove_entry(&to_replace);
        self.add_password(new_value);
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            entries: Default::default(),
            pbkdf2_rounds: MasterKey::DEFAULT_ROUNDS,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PasswordEntry {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
}

impl PartialEq for PasswordEntry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.username == other.username
            && self.password == other.password
            && self.url == other.url
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

/// This struct holds the secret parameters used to encrypt or decrypt the database
pub(crate) struct DatabaseEncryptionParams {
    /// The actual key used to encrypt and decrypt the database
    pub(crate) secret_key: String,
    /// The salt that has been used for the secret_key derivation
    pub(crate) pbkdf2_salt: SaltString,
    pub(crate) pbkdf2_rounds: u32,
}

fn try_open_truncate_from_path(path: &Path) -> Result<File, io::Error> {
    OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(path)
}

fn try_open_from_path(path: &Path) -> Result<File, io::Error> {
    OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(path)
}

impl EncryptedDatabase {
    pub fn new(path: Option<&Path>) -> Result<Self, DatabaseError> {
        match path {
            None => Ok(EncryptedDatabase::default()),
            Some(path) => EncryptedDatabase::try_read(path),
        }
    }

    pub fn try_read(path: &Path) -> Result<Self, DatabaseError> {
        match try_open_from_path(&path) {
            Ok(db_file) => {
                // read raw bytes
                let mut reader = BufReader::new(db_file);
                let mut read_encrypted_content: Vec<u8> = Vec::new();

                reader
                    .read_to_end(&mut read_encrypted_content)
                    .map_err(|e| DatabaseError::ReadError(e.to_string()))?;

                if read_encrypted_content.len() == 0 {
                    return Err(DatabaseError::DatabaseDeserializationError(
                        "The file read is empty, cannot read the database contents".to_string(),
                    ));
                }

                return serde_json::from_slice(&read_encrypted_content).map_err(|e| e.into());
            }
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    // if the file does not exist, return an empty database for now
                    return Ok(EncryptedDatabase::default());
                }
                _ => panic!("Could not open the database file: {}", e.to_string()),
            },
        }
    }
}
