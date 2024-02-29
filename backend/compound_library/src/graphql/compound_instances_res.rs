use crate::tables::{compound_instances, compound_types};
use async_graphql::{ComplexObject, Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use the_paginator::graphql::{CursorInput, ModelConnection};
use uuid::Uuid;

/// CompoundInstanceQuery is a type that represents all the queries for the compound instances.
#[derive(Debug, Clone, Default)]
pub struct CompoundInstanceQuery;

/// CompoundInstanceMutation is a type that represents all the mutations for the compound instances.
#[derive(Debug, Clone, Default)]
pub struct CompoundInstanceMutation;

#[ComplexObject]
impl compound_instances::Model {
    /// This function fetches all compound types related to this compound instance.
    async fn types(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Option<compound_types::Model>> {
        subject_authorization!("xchemlab.compound_library.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(compound_types::Entity).one(db).await?)
    }
}

#[Object]
impl CompoundInstanceQuery {
    /// This function fetches all the compound instances from the database.
    async fn compound_instances(
        &self,
        ctx: &Context<'_>,
        cursor: CursorInput,
    ) -> async_graphql::Result<ModelConnection<compound_instances::Model>> {
        subject_authorization!("xchemlab.compound_library.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(cursor
            .try_into_query_cursor::<compound_instances::Entity>()?
            .all(db)
            .await?
            .try_into_connection()?)
    }

    /// This function fetches a single compound instance from the database using the compound name.
    async fn compound_instance(
        &self,
        ctx: &Context<'_>,
        compound_name: String,
    ) -> async_graphql::Result<Option<compound_instances::Model>> {
        subject_authorization!("xchemlab.compound_library.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(compound_instances::Entity::find()
            .filter(compound_instances::Column::CompoundType.eq(compound_name.to_ascii_lowercase()))
            .one(db)
            .await?)
    }

    /// Reference resolver for compound instance in compound library subgraph
    #[graphql(entity)]
    async fn route_compound_instance(
        &self,
        ctx: &Context<'_>,
        plate_id: Uuid,
        well_number: i16,
    ) -> async_graphql::Result<Option<compound_instances::Model>> {
        subject_authorization!("xchemlab.compound_library.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(compound_instances::Entity::find()
            .filter(compound_instances::Column::PlateId.eq(plate_id))
            .filter(compound_instances::Column::WellNumber.eq(well_number))
            .one(db)
            .await?)
    }
}

#[Object]
impl CompoundInstanceMutation {
    /// This function adds a compound instance to the database.
    async fn add_compound_instance(
        &self,
        ctx: &Context<'_>,
        plate_id: Uuid,
        #[graphql(validator(minimum = 1, maximum = 288))] well_number: i16,
        compound_type: String,
    ) -> async_graphql::Result<compound_instances::Model> {
        let operator_id =
            subject_authorization!("xchemlab.compound_library.write_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let compound_instance = compound_instances::ActiveModel {
            plate_id: ActiveValue::Set(plate_id),
            well_number: ActiveValue::Set(well_number),
            compound_type: ActiveValue::Set(compound_type.to_ascii_lowercase()),
            operator_id: ActiveValue::Set(operator_id),
            timestamp: ActiveValue::Set(Utc::now()),
        };
        Ok(compound_instances::Entity::insert(compound_instance)
            .exec_with_returning(db)
            .await?)
    }
}
