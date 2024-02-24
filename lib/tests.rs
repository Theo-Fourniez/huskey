#[cfg(test)]
mod tests {
    use std::{
        fs::{self},
        panic,
        path::PathBuf,
    };

    use pbkdf2::password_hash::SaltString;
    use rand::Rng;

    use crate::{
        create_db,
        database::{EncryptedDatabase, PasswordEntry},
        decrypt_db, encrypt_and_save_db,
        key::MasterKey,
        read_db,
    };
    /// This function is used to run a test that needs a database file.
    /// It will create a random database file, run the test and then delete the file.
    /// This function is used to prevent the database file to be left on the disk if the test fails.
    fn run_db_test<T>(test: T) -> ()
    where
        T: FnOnce(PathBuf) -> () + panic::UnwindSafe,
    {
        let db_path = setup();
        let result = panic::catch_unwind(|| test(db_path.clone()));
        teardown(db_path);
        assert!(result.is_ok())
    }

    fn setup() -> PathBuf {
        generate_random_db_path()
    }

    fn teardown(path: PathBuf) {
        try_cleanup_db_file(path);
    }

    fn generate_random_db_path() -> PathBuf {
        let mut rng = rand::thread_rng();
        let random_number: u16 = rng.gen();
        PathBuf::from(format!("./db_test_{}.json", random_number))
    }
    fn try_cleanup_db_file(path: PathBuf) -> () {
        if path.try_exists().unwrap() {
            let _ = fs::remove_file(path);
        };
    }

    #[test]
    fn test_create_new_db_and_add_a_password_private_api() {
        run_db_test(|path| {
            let encrypted_db = EncryptedDatabase::new(None).unwrap();
            let read_salt = SaltString::from_b64(&encrypted_db.encryption_params.salt).unwrap();
            let key: MasterKey = MasterKey::new(String::from("password"));

            let encryption_params = key.to_decrypt_params(Some(read_salt), Some(1000)).unwrap();

            let mut db = encrypted_db
                .decrypt(encryption_params.secret_key.clone())
                .unwrap();

            db.add_password(PasswordEntry {
                name: "Test entry".to_string(),
                username: "Username".to_string(),
                password: "SuperSecretPassword".to_string(),
                url: None,
            });

            let _ = db.write_to_disk(&path, &encryption_params);
            assert_eq!(db.entries.len(), 1);
        });
    }

    /// A test used to check if we can create a new database, add a password and then save it to disk
    #[test]
    fn test_create_new_db_and_add_a_password() {
        run_db_test(|path| {
            let db = create_db().unwrap();
            let password = String::from("password");

            let mut decrypted_db = decrypt_db(db, password.clone()).unwrap();
            decrypted_db.add_password(PasswordEntry {
                name: "Entry 1".to_string(),
                username: "Username".to_string(),
                password: "Password".to_string(),
                url: None,
            });
            assert_eq!(decrypted_db.entries.len(), 1);

            let _ = encrypt_and_save_db(&decrypted_db, password.clone(), &path, None);

            decrypted_db.add_password(PasswordEntry {
                name: "Entry 1".to_string(),
                username: "Username".to_string(),
                password: "Password".to_string(),
                url: None,
            });
            assert_eq!(decrypted_db.entries.len(), 2);

            let _ = encrypt_and_save_db(&decrypted_db, password, &path, None);
        });
    }

    #[test]
    /// Create a new empty database with the lib api, write it to disk and then try to open it again
    fn test_create_new_db_write_and_open() {
        run_db_test(|path| {
            let db = create_db().unwrap();
            let password = String::from("password");

            let mut decrypted_db = decrypt_db(db, password.clone()).unwrap();
            let _ = encrypt_and_save_db(&decrypted_db, password.clone(), &path, None);

            let db = read_db(&path).unwrap();
            decrypted_db = decrypt_db(db, password.clone()).unwrap();

            assert_eq!(decrypted_db.entries.len(), 0);
        });
    }

    #[test]
    /// Create a new empty database with the lib api, write it to disk and then try to open it again in a loop
    /// This test is used to check if the database is correctly written to disk and can be opened again
    fn test_create_new_db_write_and_open_in_loop() {
        run_db_test(|path| {
            let password = String::from("password");

            let db = create_db().unwrap();
            let decrypted_db = decrypt_db(db, password.clone()).unwrap();
            let _ = encrypt_and_save_db(&decrypted_db, password.clone(), &path, None);

            for _ in 0..10 {
                let decrypted_db = read_db(&path).unwrap();
                let decrypted_db = decrypt_db(decrypted_db, password.clone()).unwrap();
                let _ = encrypt_and_save_db(&decrypted_db, password.clone(), &path, None);
            }
        });
    }

    #[test]
    fn test_changing_pbdkf2_rounds_number() {
        run_db_test(|path| {
            let password = String::from("password");

            let db = create_db().unwrap();
            let decrypted_db = decrypt_db(db, password.clone()).unwrap();
            let _ = encrypt_and_save_db(&decrypted_db, password.clone(), &path, Some(1000));

            // Change the number of rounds
            let db = read_db(&path).unwrap();
            let decrypted_db = decrypt_db(db, password.clone()).unwrap();
            assert_eq!(decrypted_db.pbdkf2_rounds, 1000);
            let _ = encrypt_and_save_db(&decrypted_db, password.clone(), &path, Some(1234));

            // Verify that the number of rounds has been changed
            let db = read_db(&path).unwrap();
            assert_eq!(db.encryption_params.pbdkf2_rounds, 1234);

            // Verify that we can still open the database
            let decrypted_db = decrypt_db(db, password.clone()).unwrap();
            assert_eq!(decrypted_db.pbdkf2_rounds, 1234);
        });
    }

    #[test]
    fn test_opening_a_non_existing_db() {
        run_db_test(|path| {
            let db = read_db(&path);
            assert!(
                db.is_err(),
                "The file does not exist, should not be able to open it"
            );
        });
    }

    #[test]
    fn test_opening_a_empty_file() {
        run_db_test(|path| {
            let _ = fs::write(&path, "");
            let db = read_db(&path);
            assert!(
                db.is_err(),
                "The file is empty, should not be able to open it"
            );
        });
    }

    #[test]
    fn test_opening_a_file_with_random_content() {
        run_db_test(|path| {
            let _ = fs::write(&path, "random content");
            let db = read_db(&path);
            assert!(
                db.is_err(),
                "The file is not a valid database, should not be able to open it"
            );
        });
    }

    #[test]
    fn test_opening_a_file_with_random_json_content() {
        run_db_test(|path| {
            let _ = fs::write(&path, r#"{"random": "content"}"#);
            let db = read_db(&path);
            assert!(
                db.is_err(),
                "The file is not a valid database, should not be able to open it"
            );
        });
    }
}
