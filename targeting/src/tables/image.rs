use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    prelude::Uuid, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
    EntityTrait, EnumIter, PrimaryKeyTrait,
};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "image")]
#[graphql(name = "Image", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub plate_id: Uuid,
    #[sea_orm(primary_key)]
    pub well_number: i16,
    pub timestamp: DateTime<Utc>,
    pub operator_id: String,
}

impl Model {
    pub fn object_key(&self) -> String {
        format!("{}/{}", self.plate_id, self.well_number)
    }
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
