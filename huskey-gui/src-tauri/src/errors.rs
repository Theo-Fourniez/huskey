use huskey_lib::database::DatabaseError;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) enum AppError {
    PasswordDatabaseError(String),
    NoDatabaseOpened,
    EditedEntryNotFound,
}

impl From<DatabaseError> for AppError {
    fn from(e: DatabaseError) -> Self {
        AppError::PasswordDatabaseError(e.to_string())
    }
}
