use super::compound_instances;
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};

/// Represents a compound type within the database.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "compound_types")]
#[graphql(name = "compound_types", complex)]
pub struct Model {
    /// The name of the type of the compound in the well.
    #[sea_orm(primary_key, auto_increment = false)]
    pub name: String,
    #[sea_orm(
        desc = "Simplified Molecular-Input Line-Entry System (SMILES) notation for the compound",
        unique
    )]
    /// Describes the SMIELS notations for the compound.
    pub smiles: String,
    /// The identifier of the operator which added this entry.
    pub operator_id: String,
    /// The date and time when the compound instance was added.
    pub timestamp: DateTime<Utc>,
}

/// Defines the relationships between entities in the database schema.
#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Defines the realtions between compound types and compound instances.
    #[sea_orm(has_many = "compound_instances::Entity")]
    CompoundInstances,
}

impl Related<compound_instances::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CompoundInstances.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
