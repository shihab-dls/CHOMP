use super::{prediction, prediction_crystal};
use async_graphql::SimpleObject;
use sea_orm::{
    prelude::Uuid, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
    EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "drop_prediction")]
#[graphql(name = "Drop", complex)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub prediction_id: Uuid,
    #[graphql(skip)]
    pub insertion_point_x: i32,
    #[graphql(skip)]
    pub insertion_point_y: i32,
    #[graphql(skip)]
    pub left: i32,
    #[graphql(skip)]
    pub right: i32,
    #[graphql(skip)]
    pub top: i32,
    #[graphql(skip)]
    pub bottom: i32,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "prediction::Entity",
        from = "Column::PredictionId",
        to = "prediction::Column::Id"
    )]
    Prediction,
    #[sea_orm(has_many = "prediction_crystal::Entity")]
    Crystals,
}

impl Related<prediction::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Prediction.def()
    }
}

impl Related<prediction_crystal::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Crystals.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
