#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub(crate) mod datatypes;
mod models;
pub(crate) mod tables;

pub use models::{
    Cryo, CryoReadback, Crystal, CrystalReadback, Fallible, ISPyBExport, Metadata,
    MetadataReadback, Mount, MountReadback, MountingResult, Position, Solvent, SolventReadback,
    Status, Visit, Well, WellReadback,
};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, EntityTrait, Schema};
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Supplied path is not valid unicode.")]
    InvalidPath,
    #[error("Database produced error during connection attempt.")]
    DatabaseError(#[from] DbErr),
}

#[derive(Debug, thiserror::Error)]
#[error("Database operation failed: {0}")]
pub struct DatabaseError(#[from] DbErr);

#[derive(Debug, thiserror::Error)]
pub enum CreationError {
    #[error("Connection failed")]
    ConnectionError(#[from] ConnectionError),
    #[error("Write failed")]
    WriteError(#[from] DbErr),
}

#[derive(Debug)]
pub struct SoakDB {
    connection: DatabaseConnection,
}

impl SoakDB {
    pub async fn connect(path: impl AsRef<Path>) -> Result<Self, ConnectionError> {
        let database_url = format!(
            "sqlite://{}",
            path.as_ref().to_str().ok_or(ConnectionError::InvalidPath)?
        );
        Ok(Self {
            connection: Database::connect(database_url).await?,
        })
    }

    pub async fn create(path: impl AsRef<Path>) -> Result<Self, CreationError> {
        let database = SoakDB::connect(format!(
            "{}?mode=rwc",
            path.as_ref().to_str().ok_or(ConnectionError::InvalidPath)?
        ))
        .await?;
        let builder = database.connection.get_database_backend();
        let schema = Schema::new(builder);
        database
            .connection
            .execute(builder.build(&schema.create_table_from_entity(tables::soak_db::Entity)))
            .await?;
        database
            .connection
            .execute(builder.build(&schema.create_table_from_entity(tables::main_table::Entity)))
            .await?;
        database
            .connection
            .execute(builder.build(&schema.create_table_from_entity(tables::pucks::Entity)))
            .await?;
        Ok(database)
    }

    pub async fn read_metadata(&self) -> Result<MetadataReadback, DatabaseError> {
        Ok(tables::soak_db::Entity::find()
            .one(&self.connection)
            .await?
            .ok_or(DbErr::Custom("No instances found".to_string()))?
            .into())
    }

    pub async fn read_wells(&self) -> Result<Vec<WellReadback>, DatabaseError> {
        Ok(tables::main_table::Entity::find()
            .all(&self.connection)
            .await?
            .into_iter()
            .map(WellReadback::from)
            .collect())
    }

    pub async fn write_metadata(
        &mut self,
        visit: Metadata,
    ) -> Result<MetadataReadback, DatabaseError> {
        Ok(
            tables::soak_db::Entity::update(tables::soak_db::ActiveModel::from(visit))
                .exec(&self.connection)
                .await?
                .into(),
        )
    }

    pub async fn insert_wells(
        &mut self,
        wells: Vec<Well>,
    ) -> Result<impl Iterator<Item = i32>, DatabaseError> {
        let num_inserts = wells.len();
        let insert = tables::main_table::Entity::insert_many(
            wells.into_iter().map(tables::main_table::ActiveModel::from),
        )
        .exec(&self.connection)
        .await?;
        Ok(insert.last_insert_id - num_inserts as i32..insert.last_insert_id)
    }
}

#[cfg(test)]
mod tests {
    use super::SoakDB;
    use std::{future::Future, path::PathBuf};

    pub fn connect_test_databases() -> impl Iterator<Item = impl Future<Output = (SoakDB, PathBuf)>>
    {
        let mut paths = std::fs::read_dir("../test_data")
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        paths.sort();
        paths.into_iter().map(|path| async {
            (
                SoakDB::connect(&path)
                    .await
                    .expect("Could not connect to test database at 'test_data/soakdb.sqlite'"),
                path,
            )
        })
    }
}
