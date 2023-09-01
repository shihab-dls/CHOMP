use crate::{
    tables::{image, prediction},
    S3Bucket,
};
use async_graphql::{ComplexObject, Context, Object, SimpleObject};
use aws_sdk_s3::presigning::PresigningConfig;
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{
    prelude::Uuid, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
    QueryTrait,
};
use std::time::Duration;

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

#[derive(Debug, SimpleObject)]
pub struct ImageCreation {
    entity: image::Model,
    upload_url: String,
}

#[derive(Debug, Clone, Default)]
pub struct ImageMutation;

#[Object]
impl ImageMutation {
    async fn create_image(
        &self,
        ctx: &Context<'_>,
        plate_id: Uuid,
        well_number: i16,
    ) -> async_graphql::Result<ImageCreation> {
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
        let inserted = image::Entity::insert(well)
            .exec_with_returning(database)
            .await?;
        let s3_presigned_url = s3_client
            .put_object()
            .key(inserted.object_key())
            .bucket(bucket.clone())
            .presigned(PresigningConfig::expires_in(Duration::from_secs(10 * 60))?)
            .await?
            .uri()
            .clone();
        Ok(ImageCreation {
            entity: inserted,
            upload_url: s3_presigned_url.to_string(),
        })
    }
}
