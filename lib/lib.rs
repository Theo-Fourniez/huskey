use std::path::Path;

use database::{Database, DatabaseError, EncryptedDatabase};
use pbkdf2::password_hash::SaltString;

use crate::key::MasterKey;

pub mod database;
mod encryption;
mod key;
mod tests;

/// Create a new database or read an existing one from disk.
pub fn create_or_read_db(path: &Path) -> Result<EncryptedDatabase, DatabaseError> {
    EncryptedDatabase::new(path)
}

/// Decrypt a database with the given password.
pub fn decrypt_db(db: EncryptedDatabase, password: String) -> Result<Database, DatabaseError> {
    let encryption_key = MasterKey::new(password);

    let salt = SaltString::from_b64(&db.encryption_params.salt)
        .expect("Could not decode the salt from the database");

    let encryption_params = encryption_key
        .to_decrypt_params(Some(salt), Some(db.encryption_params.pbdkf2_rounds))
        .expect("Could not derive the users password with PBDKF2 while decrypting");

    db.decrypt(encryption_params.secret_key)
}

/// Encrypt and save a database to disk.
/// The number of PBDKF2 rounds can be optionally provided.
/// If not provided, the number of rounds used to decrypt the database will be used.
/// If the database is new, the default number of rounds will be used.
pub fn encrypt_and_save_db(
    db: &Database,
    password: String,
    path: &Path,
    pbdkf2_rounds: Option<u32>,
) -> Result<(), DatabaseError> {
    let encryption_key = MasterKey::new(password);
    let params = encryption_key
        .to_decrypt_params(None, pbdkf2_rounds.or(Some(db.pbdkf2_rounds)))
        .expect("Could not derive the users password with PBDKF2 while encrypting");
    db.write_to_disk(path, &params)?;
    Ok(())
}
