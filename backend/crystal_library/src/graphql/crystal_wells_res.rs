use crate::entities::crystal_wells;
use async_graphql::{Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use the_paginator::graphql::{CursorInput, ModelConnection};
use uuid::Uuid;

/// CrystalQuery is a type that represents all the queries for the crystals.
#[derive(Debug, Clone, Default)]
pub struct CrystalQuery;

/// CrystalMutation is a type that represents all the mutations for the crystals.
#[derive(Debug, Clone, Default)]
pub struct CrystalMutation;

#[Object]
impl CrystalQuery {
    /// This function fetches all crystals related from the database.
    async fn crystals(
        &self,
        ctx: &Context<'_>,
        cursor: CursorInput,
    ) -> async_graphql::Result<ModelConnection<crystal_wells::Model>> {
        subject_authorization!("xchemlab.crystal_library.read_crystal", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(cursor
            .try_into_query_cursor::<crystal_wells::Entity>()?
            .all(db)
            .await?
            .try_into_connection()?)
    }

    /// This function fetches a single crystal using the plate_id and well number.
    async fn crystal(
        &self,
        ctx: &Context<'_>,
        plate_id: Uuid,
        well_number: i16,
    ) -> async_graphql::Result<Option<crystal_wells::Model>> {
        subject_authorization!("xchemlab.crystal_library.read_crystal", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(crystal_wells::Entity::find()
            .filter(crystal_wells::Column::PlateId.eq(plate_id))
            .filter(crystal_wells::Column::WellNumber.eq(well_number))
            .one(db)
            .await?)
    }
}

#[Object]
impl CrystalMutation {
    /// This function adds a crystal to the database
    async fn add_crystal(
        &self,
        ctx: &Context<'_>,
        plate_id: Uuid,
        well_number: i16,
        proposal_number: i32,
    ) -> async_graphql::Result<crystal_wells::Model> {
        let operator_id =
            subject_authorization!("xchemlab.crystal_library.write_crystal", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let crystal = crystal_wells::ActiveModel {
            plate_id: ActiveValue::Set(plate_id),
            well_number: ActiveValue::Set(well_number),
            proposal_number: ActiveValue::Set(proposal_number),
            operator_id: ActiveValue::Set(operator_id),
            timestamp: ActiveValue::Set(Utc::now()),
        };
        Ok(crystal_wells::Entity::insert(crystal)
            .exec_with_returning(db)
            .await?)
    }
}
