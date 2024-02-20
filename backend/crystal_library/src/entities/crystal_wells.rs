use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};
use uuid::Uuid;

/// Represents a crystal within the database.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "crystal_wells")]
#[graphql(name = "crystal_wells")]
pub struct Model {
    /// ID of the plate on which the crystal is located.
    #[sea_orm(primary_key, auto_increment = false)]
    pub plate_id: Uuid,
    /// The well on the plate which the crystal is located.
    #[sea_orm(primary_key, auto_increment = false)]
    pub well_number: i16,
    /// The identifier of the operator which added this entry.
    pub operator_id: String,
    /// The date and time when the compound instance was added.
    pub timestamp: DateTime<Utc>,
    /// Project proposal number
    pub proposal_number: i32,
}

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
