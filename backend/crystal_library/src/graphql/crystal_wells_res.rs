use crate::tables::{crystal_plates, crystal_wells};
use async_graphql::{ComplexObject, Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use the_paginator::graphql::{CursorInput, ModelConnection};
use uuid::Uuid;

/// CrystalQuery is a type that represents all the queries for the crystals.
#[derive(Debug, Clone, Default)]
pub struct CrystalWellsQuery;

/// CrystalMutation is a type that represents all the mutations for the crystals.
#[derive(Debug, Clone, Default)]
pub struct CrystalWellsMutation;

#[ComplexObject]
impl crystal_wells::Model {
    /// Fetches all crystal well on the crytal plate
    async fn plate(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Option<crystal_plates::Model>> {
        subject_authorization!("xchemlab.crystal_library.read_crystal_wells", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(crystal_plates::Entity).one(db).await?)
    }
}

#[Object]
impl CrystalWellsQuery {
    /// Fetches all crystals related from the database.
    async fn crystal_wells(
        &self,
        ctx: &Context<'_>,
        cursor: CursorInput,
    ) -> async_graphql::Result<ModelConnection<crystal_wells::Model>> {
        subject_authorization!("xchemlab.crystal_library.read_crystal_wells", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(cursor
            .try_into_query_cursor::<crystal_wells::Entity>()?
            .all(db)
            .await?
            .try_into_connection()?)
    }

    /// Fetches a single crystal well using the plate_id and well number.
    async fn crystal_well(
        &self,
        ctx: &Context<'_>,
        plate_id: Uuid,
        well_number: i16,
    ) -> async_graphql::Result<Option<crystal_wells::Model>> {
        subject_authorization!("xchemlab.crystal_library.read_crystal_wells", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(crystal_wells::Entity::find()
            .filter(crystal_wells::Column::PlateId.eq(plate_id))
            .filter(crystal_wells::Column::WellNumber.eq(well_number))
            .one(db)
            .await?)
    }
}

#[Object]
impl CrystalWellsMutation {
    /// Adds a crystal well to the database
    async fn add_crystal_well(
        &self,
        ctx: &Context<'_>,
        plate_id: Uuid,
        #[graphql(validator(minimum = 1, maximum = 288))] well_number: i16,
    ) -> async_graphql::Result<crystal_wells::Model> {
        let operator_id =
            subject_authorization!("xchemlab.crystal_library.write_crystal_wells", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let crystal = crystal_wells::ActiveModel {
            plate_id: ActiveValue::Set(plate_id),
            well_number: ActiveValue::Set(well_number),
            operator_id: ActiveValue::Set(operator_id),
            timestamp: ActiveValue::Set(Utc::now()),
        };
        Ok(crystal_wells::Entity::insert(crystal)
            .exec_with_returning(db)
            .await?)
    }
}
