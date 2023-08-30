use crate::{tables::well, S3Bucket};
use async_graphql::{ComplexObject, Context, Object, SimpleObject};
use aws_sdk_s3::presigning::PresigningConfig;
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{
    prelude::Uuid, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryTrait,
};
use std::time::Duration;

#[ComplexObject]
impl well::Model {
    async fn download_url(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
        let s3_client = ctx.data::<aws_sdk_s3::Client>()?;
        let bucket = ctx.data::<S3Bucket>()?;
        Ok(s3_client
            .get_object()
            .bucket(bucket.clone())
            .key(self.image_object_key.to_string())
            .presigned(PresigningConfig::expires_in(Duration::from_secs(10 * 60))?)
            .await?
            .uri()
            .clone()
            .to_string())
    }
}

#[derive(Debug, Clone, Default)]
pub struct WellQuery;

#[Object]
impl WellQuery {
    async fn wells(
        &self,
        ctx: &Context<'_>,
        id: Option<Uuid>,
    ) -> async_graphql::Result<Vec<well::Model>> {
        subject_authorization!("xchemlab.targeting.read_well", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(well::Entity::find()
            .apply_if(id, |query, id| query.filter(well::Column::Id.eq(id)))
            .all(database)
            .await?)
    }
}

#[derive(Debug, SimpleObject)]
pub struct WellCreation {
    entity: well::Model,
    upload_url: String,
}

#[derive(Debug, Clone, Default)]
pub struct WellMutation;

#[Object]
impl WellMutation {
    async fn create_well(
        &self,
        ctx: &Context<'_>,
        plate_id: Uuid,
        plate_well: i16,
    ) -> async_graphql::Result<WellCreation> {
        let operator_id = subject_authorization!("xchemlab.targeting.write_well", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let s3_client = ctx.data::<aws_sdk_s3::Client>()?;
        let bucket = ctx.data::<S3Bucket>()?;
        let image_object_key = Uuid::new_v4();
        let s3_presigned_url = s3_client
            .put_object()
            .key(image_object_key.to_string())
            .bucket(bucket.clone())
            .presigned(PresigningConfig::expires_in(Duration::from_secs(10 * 60))?)
            .await?
            .uri()
            .clone();
        let well = well::ActiveModel {
            id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
            crystal_plate_id: sea_orm::ActiveValue::Set(plate_id),
            crystal_plate_well: sea_orm::ActiveValue::Set(plate_well),
            image_object_key: sea_orm::ActiveValue::Set(image_object_key),
            timestamp: sea_orm::ActiveValue::Set(Utc::now()),
            operator_id: sea_orm::ActiveValue::Set(operator_id),
        };
        let inserted = well::Entity::insert(well)
            .exec_with_returning(database)
            .await?;
        Ok(WellCreation {
            entity: inserted,
            upload_url: s3_presigned_url.to_string(),
        })
    }
}
