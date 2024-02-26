use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};
use uuid::Uuid;

/// Represents a soaked compound within the database.
#[derive(Clone, Debug, DeriveEntityModel, PartialEq, SimpleObject)]
#[sea_orm(table_name = "soak_compound")]
#[graphql(name = "soak_compound")]
pub struct Model {
    /// ID of the plate on which the compound is located.
    #[sea_orm(primary_key, auto_increment = false)]
    pub compound_plate_id: Uuid,
    /// The well on the plate which the compound is located.
    #[sea_orm(primary_key, auto_increment = false)]
    pub compound_well_number: i16,
    /// ID of the plate on which the crystal is located.
    #[sea_orm(primary_key, auto_increment = false)]
    pub crystal_plate_id: Uuid,
    /// The well on the plate which the crystal is located.
    #[sea_orm(primary_key, auto_increment = false)]
    pub crystal_well_number: i16,
    /// The volume of compounds soaked.
    pub volume: f32,
    /// The identifier of the operator which added this entry.
    pub operator_id: String,
    /// The date and time when the compound instance was added.
    pub timestamp: DateTime<Utc>,
}

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
