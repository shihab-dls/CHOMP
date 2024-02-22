use super::crystal_plates;
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use uuid::Uuid;

/// Represents a crystal within the database.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "crystal_wells")]
#[graphql(name = "crystal_wells", complex)]
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
}

/// Defines the relationships between entities in the database schema
#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crystal_plates::Entity",
        from = "Column::PlateId",
        to = "crystal_plates::Column::PlateId"
    )]
    /// Defines the relations between the crystal plates and crystal wells
    CrystalPlates,
}

impl Related<crystal_plates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CrystalPlates.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
