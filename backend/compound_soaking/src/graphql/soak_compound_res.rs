use crate::entities::soak_compound;
use async_graphql::{Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use the_paginator::graphql::{CursorInput, ModelConnection};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct SoakCompoundQuery;

#[derive(Debug, Clone, Default)]
pub struct SoakCompoundMutation;

#[Object]
impl SoakCompoundQuery {
    async fn soaked_compounds(
        &self,
        ctx: &Context<'_>,
        cursor: CursorInput,
    ) -> async_graphql::Result<ModelConnection<soak_compound::Model>> {
        subject_authorization!("xchemlab.compound_soaking.read_soaked_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(cursor
            .try_into_query_cursor::<soak_compound::Entity>()?
            .all(db)
            .await?
            .try_into_connection()?)
    }

    async fn soaked_compound(
        &self,
        ctx: &Context<'_>,
        compound_plate_id: Uuid,
        compound_well_number: i16,
        crystal_plate_id: Uuid,
        crystal_well_number: i16,
    ) -> async_graphql::Result<Option<soak_compound::Model>> {
        subject_authorization!("xchemlab.compound_soaking.read_soaked_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(soak_compound::Entity::find()
            .filter(soak_compound::Column::CompoundPlateId.eq(compound_plate_id))
            .filter(soak_compound::Column::CompoundWellNumber.eq(compound_well_number))
            .filter(soak_compound::Column::CrystalPlateId.eq(crystal_plate_id))
            .filter(soak_compound::Column::CrystalWellNumber.eq(crystal_well_number))
            .one(db)
            .await?)
    }
}

#[Object]
impl SoakCompoundMutation {
    async fn add_soaked_compound(
        &self,
        ctx: &Context<'_>,
        compound_plate_id: Uuid,
        compound_well_number: i16,
        crystal_plate_id: Uuid,
        crystal_well_number: i16,
    ) -> async_graphql::Result<soak_compound::Model> {
        let operator_id =
            subject_authorization!("xchemlab.compound_soaking.write_soaked_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let crystal = soak_compound::ActiveModel {
            compound_plate_id: ActiveValue::Set(compound_plate_id),
            compound_well_number: ActiveValue::Set(compound_well_number),
            crystal_plate_id: ActiveValue::Set(crystal_plate_id),
            crystal_well_number: ActiveValue::Set(crystal_well_number),
            operator_id: ActiveValue::Set(operator_id),
            timestamp: ActiveValue::Set(Utc::now()),
        };
        Ok(soak_compound::Entity::insert(crystal)
            .exec_with_returning(db)
            .await?)
    }
}
