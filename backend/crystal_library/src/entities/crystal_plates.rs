use super::crystal_wells;
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use uuid::Uuid;

/// Represents a plate on which crystals are located.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "crystal_plates")]
#[graphql(name = "crystal_plates", complex)]
pub struct Model {
    /// ID of the plate on which the crystal is located.
    #[sea_orm(primary_key, auto_increment = false)]
    pub plate_id: Uuid,
    /// Project proposal number
    pub proposal_number: i32,
    /// The identifier of the operator which added this entry.
    pub operator_id: String,
    /// The date and time when the compound instance was added.
    pub timestamp: DateTime<Utc>,
}

/// Defines the relationships between entities in the database schema
#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Defines the relations between the crystal wells and crystal plates
    #[sea_orm(has_many = "crystal_wells::Entity")]
    CrystalWells,
}

impl Related<crystal_wells::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CrystalWells.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
