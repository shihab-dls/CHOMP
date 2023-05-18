#[doc = include_str!("../../README.md")]
#[forbid(unsafe_code)]
#[warn(missing_docs)]
pub(crate) mod datatypes;
pub mod models;
pub(crate) mod tables;

use models::{Metadata, MetadataReadback, WellReadback};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, EntityTrait, Schema};
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Supplied path is not valid unicode.")]
    InvalidPath,
    #[error("Database produced error during connection attempt.")]
    DatabaseError(#[from] DbErr),
}

async fn connect(path: impl AsRef<Path>) -> Result<DatabaseConnection, ConnectionError> {
    let database_url = format!(
        "sqlite://{}",
        path.as_ref().to_str().ok_or(ConnectionError::InvalidPath)?
    );
    Ok(Database::connect(database_url).await?)
}

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("Connection failed")]
    ConnectionError(#[from] ConnectionError),
    #[error("Read failed")]
    ReadError(#[from] DbErr),
}

pub async fn read_metadata(path: impl AsRef<Path>) -> Result<MetadataReadback, ReadError> {
    let database = connect(path).await?;
    Ok(tables::soak_db::Entity::find()
        .one(&database)
        .await?
        .ok_or(DbErr::Custom("No instances found".to_string()))?
        .into())
}

pub async fn read_wells(path: impl AsRef<Path>) -> Result<Vec<WellReadback>, ReadError> {
    let database = connect(path).await?;
    Ok(tables::main_table::Entity::find()
        .all(&database)
        .await?
        .into_iter()
        .map(WellReadback::from)
        .collect())
}

#[derive(Debug, thiserror::Error)]
pub enum WriteError {
    #[error("Connection failed")]
    ConnectionError(#[from] ConnectionError),
    #[error("Write failed")]
    WriteError(#[from] DbErr),
}

pub async fn create_database(path: impl AsRef<Path>) -> Result<(), WriteError> {
    let database = connect(format!(
        "{}?mode=rwc",
        path.as_ref().to_str().ok_or(ConnectionError::InvalidPath)?
    ))
    .await?;
    let builder = database.get_database_backend();
    let schema = Schema::new(builder);
    database
        .execute(builder.build(&schema.create_table_from_entity(tables::soak_db::Entity)))
        .await?;
    database
        .execute(builder.build(&schema.create_table_from_entity(tables::main_table::Entity)))
        .await?;
    database
        .execute(builder.build(&schema.create_table_from_entity(tables::pucks::Entity)))
        .await?;
    Ok(())
}

pub async fn write_metadata(
    path: impl AsRef<Path>,
    visit: Metadata,
) -> Result<MetadataReadback, WriteError> {
    let database = connect(path).await?;
    Ok(
        tables::soak_db::Entity::update(tables::soak_db::ActiveModel::from(visit))
            .exec(&database)
            .await?
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::connect;
    use sea_orm::DatabaseConnection;
    use std::{future::Future, path::PathBuf};

    pub fn connect_test_databases(
    ) -> impl Iterator<Item = impl Future<Output = (DatabaseConnection, PathBuf)>> {
        let mut paths = std::fs::read_dir("../test_data")
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
