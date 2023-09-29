use crate::tables::{prediction, prediction_crystal, prediction_drop};
use async_graphql::{ComplexObject, Context, InputObject, Object, SimpleObject};
use chrono::Utc;
use graphql_types::Well;
use opa_client::subject_authorization;
use sea_orm::{
    prelude::Uuid, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait,
    QueryFilter, QueryTrait, TransactionTrait,
};

#[derive(Debug, Clone, SimpleObject, InputObject)]
#[graphql(input_name = "PointInput")]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, SimpleObject, InputObject)]
#[graphql(input_name = "BoundingBoxInput")]
struct BoundingBox {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

#[ComplexObject]
impl prediction_crystal::Model {
    async fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            left: self.left,
            right: self.right,
            top: self.top,
            bottom: self.bottom,
        }
    }
}

#[derive(Debug, Clone, InputObject)]
struct CrystalInput {
    bounding_box: BoundingBox,
}

impl prediction_crystal::ActiveModel {
    fn from_crystal_input_and_drop_id(crystal_input: &CrystalInput, drop_id: Uuid) -> Self {
        Self {
            id: ActiveValue::Set(Uuid::new_v4()),
            drop_id: ActiveValue::Set(drop_id),
            left: ActiveValue::Set(crystal_input.bounding_box.left),
            right: ActiveValue::Set(crystal_input.bounding_box.right),
            top: ActiveValue::Set(crystal_input.bounding_box.top),
            bottom: ActiveValue::Set(crystal_input.bounding_box.bottom),
        }
    }
}

#[ComplexObject]
impl prediction_drop::Model {
    async fn insertion_point(&self) -> Point {
        Point {
            x: self.insertion_point_x,
            y: self.insertion_point_y,
        }
    }

    async fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            left: self.left,
            right: self.right,
            top: self.top,
            bottom: self.bottom,
        }
    }

    async fn crystals(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<prediction_crystal::Model>> {
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self
            .find_related(prediction_crystal::Entity)
            .all(database)
            .await?)
    }
}

#[derive(Debug, Clone, InputObject)]
struct DropInput {
    insertion_point: Point,
    bounding_box: BoundingBox,
    crystals: Vec<CrystalInput>,
}

impl prediction_drop::ActiveModel {
    fn from_drop_input_and_prediction_id(drop_input: &DropInput, prediction_id: Uuid) -> Self {
        Self {
            id: ActiveValue::Set(Uuid::new_v4()),
            prediction_id: ActiveValue::Set(prediction_id),
            insertion_point_x: ActiveValue::Set(drop_input.insertion_point.x),
            insertion_point_y: ActiveValue::Set(drop_input.insertion_point.y),
            left: ActiveValue::Set(drop_input.bounding_box.left),
            right: ActiveValue::Set(drop_input.bounding_box.right),
            top: ActiveValue::Set(drop_input.bounding_box.top),
            bottom: ActiveValue::Set(drop_input.bounding_box.bottom),
        }
    }
}

#[ComplexObject]
impl prediction::Model {
    async fn image(&self) -> Well {
        Well {
            plate: self.plate,
            well: self.well,
        }
    }

    async fn well_centroid(&self) -> Point {
        Point {
            x: self.well_centroid_x,
            y: self.well_centroid_y,
        }
    }

    async fn drops(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<prediction_drop::Model>> {
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self
            .find_related(prediction_drop::Entity)
            .all(database)
            .await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PredictionQuery;

#[Object]
impl PredictionQuery {
    async fn predictions(
        &self,
        ctx: &Context<'_>,
        id: Option<Uuid>,
        plate: Option<Uuid>,
        well: Option<i16>,
        operator_id: Option<String>,
    ) -> async_graphql::Result<Vec<prediction::Model>> {
        subject_authorization!("xchemlab.targeting.read_prediction", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(prediction::Entity::find()
            .apply_if(id, |query, id| query.filter(prediction::Column::Id.eq(id)))
            .apply_if(plate, |query, plate| {
                query.filter(prediction::Column::Plate.eq(plate))
            })
            .apply_if(well, |query, well| {
                query.filter(prediction::Column::Well.eq(well))
            })
            .apply_if(operator_id, |query, operator_id| {
                query.filter(prediction::Column::OperatorId.eq(operator_id))
            })
            .all(database)
            .await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PredicitonMutation;

#[Object]
impl PredicitonMutation {
    async fn create_prediction(
        &self,
        ctx: &Context<'_>,
        plate: Well,
        well_centroid: Point,
        well_radius: i32,
        drops: Vec<DropInput>,
    ) -> async_graphql::Result<prediction::Model> {
        let operator_id =
            subject_authorization!("xchemlab.targeting.write_prediction", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;

        let prediction = database
            .transaction::<_, _, DbErr>(|transaction| {
                Box::pin(async move {
                    let prediction = prediction::Entity::insert(prediction::ActiveModel {
                        id: ActiveValue::Set(Uuid::new_v4()),
                        plate: ActiveValue::Set(plate.plate),
                        well: ActiveValue::Set(plate.well),
                        well_centroid_x: ActiveValue::Set(well_centroid.x),
                        well_centroid_y: ActiveValue::Set(well_centroid.y),
                        well_radius: ActiveValue::Set(well_radius),
                        timestamp: ActiveValue::Set(Utc::now()),
                        operator_id: ActiveValue::Set(operator_id),
                    })
                    .exec_with_returning(transaction)
                    .await?;

                    for drop_input in drops {
                        let drop = prediction_drop::Entity::insert(
                            prediction_drop::ActiveModel::from_drop_input_and_prediction_id(
                                &drop_input,
                                prediction.id,
                            ),
                        )
                        .exec_with_returning(transaction)
                        .await?;

                        prediction_crystal::Entity::insert_many(
                            drop_input.crystals.into_iter().map(|crystal_input| {
                                prediction_crystal::ActiveModel::from_crystal_input_and_drop_id(
                                    &crystal_input,
                                    drop.id,
                                )
                            }),
                        )
                        .exec(transaction)
                        .await?;
                    }

                    Ok(prediction)
                })
            })
            .await?;

        Ok(prediction)
    }
}
