use super::compound_types;
use async_graphql::SimpleObject;
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "compound_instances")]
#[graphql(name = "compund_instances", complex)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub plate_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub well_number: i16,
    pub compound_type: String,
    pub username: String, 
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
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
