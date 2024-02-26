use crate::tables::{compound_instances, compound_types};
use axum::async_trait;
use sea_orm::{DbErr, DeriveMigrationName, Schema};
use sea_orm_migration::{MigrationTrait, MigratorTrait, SchemaManager};

/// Migrator for managing and applying database migrations.
pub struct Migrator;

/// This struct is used to define the very first migration that sets up
/// the initial database schema.
#[derive(DeriveMigrationName)]
struct Initial;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(Initial)]
    }
}

#[async_trait]
impl MigrationTrait for Initial {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();
        let schema = Schema::new(backend);

        manager
            .create_table(schema.create_table_from_entity(compound_types::Entity))
            .await?;

        manager
            .create_table(schema.create_table_from_entity(compound_instances::Entity))
            .await?;

        Ok(())
    }
}
