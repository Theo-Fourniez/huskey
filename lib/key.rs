/*
This file is part of Huskey.

Huskey is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Huskey is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Huskey. If not, see <https://www.gnu.org/licenses/>.
*/

use pbkdf2::Params;
use rand_core::OsRng;
use sha2::Digest;
use sha2::Sha512;

/// This structs holds the actual user provided values used to encrypt or decrypt the data
/// For now there is only a password for authentication, but more will be implemented in the future
pub(crate) struct MasterKey {
    password: String,
}

impl MasterKey {
    pub fn new(password: String) -> Self {
        MasterKey { password }
    }
}

trait Hashable {
    fn sha512(self) -> String;
}

impl Hashable for String {
    fn sha512(self) -> String {
        let mut hasher: Sha512 = Sha512::new();
        // write input message
        hasher.update(self);
        // create digest and transform to upper hex string
        format!("{:X}", hasher.finalize())
    }
}
use pbkdf2::{
    password_hash::{PasswordHasher, SaltString},
    Pbkdf2,
};

use crate::database::DatabaseEncryptionParams;
impl MasterKey {
    /// A output of length 24 is needed to be used as the secret in AES256GCM.
    const NEEDED_OUTPUT_LENGTH: usize = 24;

    pub const OWASP_RECOMMENDED_PBKDF2_ROUNDS: u32 = 210_000;
    #[cfg(debug_assertions)]
    pub const DEFAULT_ROUNDS: u32 = 1000; // for testing purposes
    #[cfg(not(debug_assertions))]
    pub const DEFAULT_ROUNDS: u32 = MasterKey::OWASP_RECOMMENDED_PBKDF2_ROUNDS; // for production

    /// Use the PBKDF2 (SHA512) to derive the MasterKey using a salt.
    /// If the salt is not provided, a random one is generated.
    /// A random salt must be generated each time the database is saved to prevent rainbow tables attacks.
    /// The most probable error would be a password or a salt considered invalid.
    pub(crate) fn to_decrypt_params(
        &self,
        optional_salt: Option<SaltString>,
        pbkdf2_rounds: Option<u32>,
    ) -> Result<DatabaseEncryptionParams, pbkdf2::password_hash::Error> {
        let salt = optional_salt.unwrap_or(SaltString::generate(&mut OsRng));
        let rounds = pbkdf2_rounds.unwrap_or(Self::DEFAULT_ROUNDS);
        let full_master_key = self.password.clone().sha512();

        if rounds < MasterKey::OWASP_RECOMMENDED_PBKDF2_ROUNDS {
            println!(
                "Warning: The number of PBKDF2 rounds ({}) is lower than the OWASP recommended value of {}",
                rounds,
                MasterKey::OWASP_RECOMMENDED_PBKDF2_ROUNDS
            );
        }

        let master_key_hash = Pbkdf2.hash_password_customized(
            full_master_key.as_bytes(),
            Some(pbkdf2::Algorithm::Pbkdf2Sha512.ident()),
            None,
            Params {
                rounds,
                output_length: Self::NEEDED_OUTPUT_LENGTH,
            },
            &salt,
        )?;

        Ok(DatabaseEncryptionParams {
            secret_key: master_key_hash.hash.unwrap().to_string(),
            pbkdf2_salt: salt,
            pbkdf2_rounds: rounds,
        })
    }
}
