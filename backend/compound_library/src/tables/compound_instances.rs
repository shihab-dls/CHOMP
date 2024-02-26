use super::compound_types;
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use uuid::Uuid;

/// Represents a compound instance within the database.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "compound_instances")]
#[graphql(name = "compound_instances", complex, shareable)]
pub struct Model {
    /// ID of the plate on which the compound instance is located.
    #[sea_orm(primary_key, auto_increment = false)]
    pub plate_id: Uuid,
    /// The well on the plate which the compound instance is located.
    #[sea_orm(primary_key, auto_increment = false)]
    pub well_number: i16,
    /// The name of the type of the compound in the well.
    pub compound_type: String,
    /// The identifier of the operator which added this entry.
    pub operator_id: String,
    /// The date and time when the compound instance was added.
    pub timestamp: DateTime<Utc>,
}

/// Defines the relationships between entities in the database schema.
#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Defines the realtions between compound instances and compound types.
    #[sea_orm(
        belongs_to = "compound_types::Entity",
        from = "Column::CompoundType",
        to = "compound_types::Column::Name"
    )]
    CompoundTypes,
}

impl Related<compound_types::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CompoundTypes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
