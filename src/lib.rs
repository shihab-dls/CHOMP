pub(crate) mod tables;

use sea_orm::{Database, DatabaseConnection, DbErr};
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Supplied path is not valid unicode.")]
    InvalidPath,
    #[error("Database produced error during connection attempt.")]
    DatabaseError(#[from] DbErr),
}

pub async fn connect(path: impl AsRef<Path>) -> Result<DatabaseConnection, ConnectionError> {
    let database_url = format!(
        "sqlite://{}",
        path.as_ref().to_str().ok_or(ConnectionError::InvalidPath)?
    );
    Ok(Database::connect(database_url).await?)
}
