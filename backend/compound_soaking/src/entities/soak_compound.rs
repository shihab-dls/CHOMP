use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};
use uuid::Uuid;

#[derive(Clone, Debug, DeriveEntityModel, Eq, PartialEq, SimpleObject)]
#[sea_orm(table_name = "soak_compound")]
#[graphql(name = "soak_compound")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub compound_plate_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub compound_well_number: i16,
    #[sea_orm(primary_key, auto_increment = false)]
    pub crystal_plate_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub crystal_well_number: i16,
    pub operator_id: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
