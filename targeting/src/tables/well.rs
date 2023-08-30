use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    prelude::Uuid, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
    EntityTrait, EnumIter, PrimaryKeyTrait,
};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "well")]
#[graphql(name = "Well", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub plate_id: Uuid,
    #[sea_orm(primary_key)]
    pub plate_well: i16,
    pub timestamp: DateTime<Utc>,
    pub operator_id: String,
}

impl Model {
    pub fn image_object_key(&self) -> String {
        format!("{}/{}", self.plate_id, self.plate_well)
    }
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
