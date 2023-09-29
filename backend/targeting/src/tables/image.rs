use super::prediction;
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    prelude::Uuid, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
    EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "image")]
#[graphql(name = "Image", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub plate: Uuid,
    #[sea_orm(primary_key)]
    pub well: i16,
    pub timestamp: DateTime<Utc>,
    pub operator_id: String,
}

impl Model {
    pub fn object_key(&self) -> String {
        format!("{}/{}", self.plate, self.well)
    }
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "prediction::Entity")]
    Predictions,
}

impl Related<prediction::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Predictions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
