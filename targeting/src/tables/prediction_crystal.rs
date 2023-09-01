use super::prediction_drop;
use async_graphql::SimpleObject;
use sea_orm::{
    prelude::Uuid, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
    EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "crystal_prediction")]
#[graphql(name = "Crystal", complex)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub drop_id: Uuid,
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
        belongs_to = "prediction_drop::Entity",
        from = "Column::DropId",
        to = "prediction_drop::Column::Id"
    )]
    Drop,
}

impl Related<prediction_drop::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Drop.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
