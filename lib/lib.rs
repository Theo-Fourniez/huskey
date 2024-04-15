/*
This file is part of Huskey.

Huskey is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Huskey is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Huskey. If not, see <https://www.gnu.org/licenses/>.
*/

//! It is a library that can be used to create, read, update and delete passwords from a database.
//! The database is encrypted and saved to disk.
//! Using AES-256-GCM for encryption and PBKDF2 (SHA512) for key derivation.

use std::path::Path;

use database::{Database, DatabaseError, EncryptedDatabase};
use pbkdf2::password_hash::SaltString;

use crate::key::MasterKey;

pub mod database;
mod encryption;
mod key;
mod tests;

pub fn create_db() -> Result<EncryptedDatabase, DatabaseError> {
    EncryptedDatabase::new(None)
}

pub fn read_db(path: &Path) -> Result<EncryptedDatabase, DatabaseError> {
    EncryptedDatabase::new(Some(path))
}

/// Decrypt a database with the given password.
pub fn decrypt_db(db: EncryptedDatabase, password: String) -> Result<Database, DatabaseError> {
    let encryption_key = MasterKey::new(password);

    let salt = SaltString::from_b64(&db.encryption_params.salt)
        .expect("Could not decode the salt from the database");

    let encryption_params = encryption_key
        .to_decrypt_params(Some(salt), Some(db.encryption_params.pbkdf2_rounds))
        .expect("Could not derive the users password with PBKDF2 while decrypting");

    db.decrypt(encryption_params.secret_key)
}

/// Encrypt and save a database to disk.
/// The number of PBKDF2 rounds can be optionally provided.
/// If not provided, the number of rounds used to decrypt the database will be used.
/// If the database is new, the default number of rounds will be used.
pub fn encrypt_and_save_db(
    db: &Database,
    password: String,
    path: &Path,
    pbkdf2_rounds: Option<u32>,
) -> Result<(), DatabaseError> {
    let encryption_key = MasterKey::new(password);

    let params = encryption_key
        .to_decrypt_params(None, pbkdf2_rounds.or(Some(db.pbkdf2_rounds)))
        .expect("Could not derive the users password with PBKDF2 while encrypting");

    db.write_to_disk(path, &params)?;
    Ok(())
}
