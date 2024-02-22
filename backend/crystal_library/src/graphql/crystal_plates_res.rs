use crate::entities::{crystal_plates, crystal_wells};
use async_graphql::{ComplexObject, Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};
use the_paginator::graphql::{CursorInput, ModelConnection};
use uuid::Uuid;

/// CrystalQuery is a type that represents all the queries for the crystals.
#[derive(Debug, Clone, Default)]
pub struct CrystalPlatesQuery;

/// CrystalMutation is a type that represents all the mutations for the crystals.
#[derive(Debug, Clone, Default)]
pub struct CrystalPlatesMutation;

#[ComplexObject]
impl crystal_plates::Model {
    /// This function fetches all crystal well on the crytal plate
    async fn wells(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<crystal_wells::Model>> {
        subject_authorization!("xchemlab.crystal_library.read_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(crystal_wells::Entity).all(db).await?)
    }
}

#[Object]
impl CrystalPlatesQuery {
    /// Fetches all crystal plates from the database.
    async fn crystal_plates(
        &self,
        ctx: &Context<'_>,
        cursor: CursorInput,
    ) -> async_graphql::Result<ModelConnection<crystal_plates::Model>> {
        subject_authorization!("xchemlab.crystal_library.read_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(cursor
            .try_into_query_cursor::<crystal_plates::Entity>()?
            .all(db)
            .await?
            .try_into_connection()?)
    }

    /// Fetches a single crystal plate using the plate_id.
    async fn crystal_plate(
        &self,
        ctx: &Context<'_>,
        plate_id: Uuid,
    ) -> async_graphql::Result<Option<crystal_plates::Model>> {
        subject_authorization!("xchemlab.crystal_library.read_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(crystal_plates::Entity::find_by_id(plate_id).one(db).await?)
    }
}

#[Object]
impl CrystalPlatesMutation {
    /// Adds a crystal plates to the database
    async fn add_crystal_plate(
        &self,
        ctx: &Context<'_>,
        plate_id: Uuid,
        proposal_number: i32,
    ) -> async_graphql::Result<crystal_plates::Model> {
        let operator_id =
            subject_authorization!("xchemlab.crystal_library.write_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let crystal = crystal_plates::ActiveModel {
            plate_id: ActiveValue::Set(plate_id),
            proposal_number: ActiveValue::Set(proposal_number),
            operator_id: ActiveValue::Set(operator_id),
            timestamp: ActiveValue::Set(Utc::now()),
        };
        Ok(crystal_plates::Entity::insert(crystal)
            .exec_with_returning(db)
            .await?)
    }
}
