use crate::{
    tables::{image, prediction},
    S3Bucket,
};
use async_graphql::{ComplexObject, Context, Object, Subscription, Upload};
use aws_sdk_s3::presigning::PresigningConfig;
use chrono::Utc;
use graphql_event_broker::EventBroker;
use opa_client::subject_authorization;
use sea_orm::{
    prelude::Uuid, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
    QueryTrait,
};
use std::time::Duration;
use tokio_stream::Stream;

#[ComplexObject]
impl image::Model {
    async fn download_url(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
        let s3_client = ctx.data::<aws_sdk_s3::Client>()?;
        let bucket = ctx.data::<S3Bucket>()?;
        Ok(s3_client
            .get_object()
            .bucket(bucket.clone())
            .key(self.object_key())
            .presigned(PresigningConfig::expires_in(Duration::from_secs(10 * 60))?)
            .await?
            .uri()
            .clone()
            .to_string())
    }

    async fn predictions(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<prediction::Model>> {
        subject_authorization!("xchemlab.targeting.read_prediction", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(prediction::Entity).all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ImageQuery;

#[Object]
impl ImageQuery {
    async fn images(
        &self,
        ctx: &Context<'_>,
        plate_id: Option<Uuid>,
        well_number: Option<i16>,
    ) -> async_graphql::Result<Vec<image::Model>> {
        subject_authorization!("xchemlab.targeting.read_image", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(image::Entity::find()
            .apply_if(plate_id, |query, plate_id| {
                query.filter(image::Column::PlateId.eq(plate_id))
            })
            .apply_if(well_number, |query, well_number| {
                query.filter(image::Column::WellNumber.eq(well_number))
            })
            .all(database)
            .await?)
    }
}

static IMAGE_CREATION_BROKER: EventBroker<image::Model> = EventBroker::new();

#[derive(Debug, Clone, Default)]
pub struct ImageMutation;

#[Object]
impl ImageMutation {
    async fn create_image(
        &self,
        ctx: &Context<'_>,
        plate_id: Uuid,
        well_number: i16,
        image: Upload,
    ) -> async_graphql::Result<image::Model> {
        let operator_id = subject_authorization!("xchemlab.targeting.write_image", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let s3_client = ctx.data::<aws_sdk_s3::Client>()?;
        let bucket = ctx.data::<S3Bucket>()?;
        let well = image::ActiveModel {
            plate_id: sea_orm::ActiveValue::Set(plate_id),
            well_number: sea_orm::ActiveValue::Set(well_number),
            timestamp: sea_orm::ActiveValue::Set(Utc::now()),
            operator_id: sea_orm::ActiveValue::Set(operator_id),
        };
        s3_client
            .put_object()
            .key(format!("{plate_id}/{well_number}"))
            .bucket(bucket.clone())
            .body(image.value(ctx)?.content.into())
            .send()
            .await?;
        let inserted = image::Entity::insert(well)
            .exec_with_returning(database)
            .await?;
        IMAGE_CREATION_BROKER.publish(inserted.clone());
        Ok(inserted)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ImageSubscription;

#[Subscription]
impl ImageSubscription {
    async fn image_created(&self) -> impl Stream<Item = image::Model> {
        IMAGE_CREATION_BROKER.subscribe()
    }
}
