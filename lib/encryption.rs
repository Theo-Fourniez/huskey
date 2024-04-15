/*
This file is part of Huskey.

Huskey is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Huskey is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Huskey. If not, see <https://www.gnu.org/licenses/>.
*/

use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use typenum::U12;

#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("The key length must be 32 bytes, got a key of length {0}")]
    InvalidKeyLength(u8),
    #[error("Error encrypting data with AES-GCM")]
    EncryptError,
    #[error("Error decrypting data with AES-GCM : make sure you are using the right decryption parameters and the data has not been tampered with")]
    DecryptError,
}

pub(crate) fn encrypt(
    value: &Vec<u8>,
    secret: Vec<u8>,
    nonce: &Nonce<U12>,
) -> Result<Vec<u8>, EncryptionError> {
    let cipher = Aes256Gcm::new_from_slice(&secret)
        .map_err(|_e| EncryptionError::InvalidKeyLength(secret.len() as u8))?;

    cipher
        .encrypt(nonce, value.as_ref())
        .map_err(|_e| EncryptionError::EncryptError)
}

pub(crate) fn decrypt(
    value: &Vec<u8>,
    secret: Vec<u8>,
    nonce: &Nonce<U12>,
) -> Result<Vec<u8>, EncryptionError> {
    let cipher = Aes256Gcm::new_from_slice(&secret)
        .map_err(|_e| EncryptionError::InvalidKeyLength(secret.len() as u8))?;

    cipher
        .decrypt(nonce, value.as_ref())
        .map_err(|_e| EncryptionError::DecryptError)
}

pub(crate) fn generate_nonce() -> Nonce<U12> {
    *Nonce::from_slice(
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12) // 12 * 8 (u8 size) = 96
            .map(char::from)
            .collect::<String>()
            .as_bytes(),
    )
}
#[derive(Serialize, Deserialize, Debug)]
/// Custom fixed 12 bytes nonces size to expand [u8; 12] type (newtype pattern)
pub struct FixedNonce([u8; 12]);

impl From<Nonce<U12>> for FixedNonce {
    fn from(value: Nonce<U12>) -> Self {
        FixedNonce(
            value
                .as_slice()
                .try_into()
                .expect("Length of the nonce must be 12 bytes"),
        )
    }
}

use std::ops::{Deref, DerefMut};

impl Deref for FixedNonce {
    type Target = [u8; 12];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FixedNonce {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[u8; 12]> for FixedNonce {
    fn from(value: [u8; 12]) -> Self {
        FixedNonce(value)
    }
}
