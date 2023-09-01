use super::{image, prediction_drop};
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    prelude::Uuid, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
    EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "prediction")]
#[graphql(name = "Prediction", complex)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[graphql(skip)]
    pub plate_id: Uuid,
    #[graphql(skip)]
    pub well_number: i16,
    #[graphql(skip)]
    pub well_centroid_x: i32,
    #[graphql(skip)]
    pub well_centroid_y: i32,
    pub well_radius: i32,
    pub timestamp: DateTime<Utc>,
    pub operator_id: String,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "image::Entity",
        from = "(Column::PlateId, Column::WellNumber)",
        to = "(image::Column::PlateId, image::Column::WellNumber)"
    )]
    Well,
    #[sea_orm(has_many = "prediction_drop::Entity")]
    Drops,
}

impl Related<image::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Well.def()
    }
}

impl Related<prediction_drop::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Drops.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
