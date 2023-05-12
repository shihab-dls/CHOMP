pub(crate) mod datatypes;
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

#[cfg(test)]
mod tests {
    use super::connect;
    use sea_orm::DatabaseConnection;
    use std::{future::Future, path::PathBuf};

    pub fn connect_test_databases(
    ) -> impl Iterator<Item = impl Future<Output = (DatabaseConnection, PathBuf)>> {
        let mut paths = std::fs::read_dir("test_data")
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        paths.sort();
        paths.into_iter().map(|path| async {
            (
                connect(&path)
                    .await
                    .expect("Could not connect to test database at 'test_data/soakdb.sqlite'"),
                path,
            )
        })
    }
}
