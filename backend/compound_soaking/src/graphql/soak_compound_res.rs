use super::subgraph_extensions::{CompoundInstances, CrystalWells};
use crate::tables::soak_compound;
use async_graphql::{ComplexObject, Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use the_paginator::graphql::{CursorInput, ModelConnection};
use uuid::Uuid;

/// SoakCompoundQuery is a type that represents all the queries for the compound soaking.
#[derive(Debug, Clone, Default)]
pub struct SoakCompoundQuery;

/// SoakCompoundMutation is a type that represents all the mutations for the compound soaking.
#[derive(Debug, Clone, Default)]
pub struct SoakCompoundMutation;

#[ComplexObject]
impl CrystalWells {
    /// Fetches all the compounds soaked in a crystal well
    async fn compound_soaked(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<soak_compound::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(soak_compound::Entity::find()
            .filter(soak_compound::Column::CrystalPlateId.eq(self.plate_id))
            .filter(soak_compound::Column::CrystalWellNumber.eq(self.well_number))
            .all(db)
            .await?)
    }
}

#[ComplexObject]
impl CompoundInstances {
    /// Fetches all the crystals soaked with the compounds
    async fn crystal_soaked(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<soak_compound::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(soak_compound::Entity::find()
            .filter(soak_compound::Column::CompoundPlateId.eq(self.plate_id))
            .filter(soak_compound::Column::CompoundWellNumber.eq(self.well_number))
            .all(db)
            .await?)
    }
}

#[ComplexObject]
impl soak_compound::Model {
    /// Fetches the information of the crystals soaked from
    /// crystal library subgraph
    async fn crystals(&self) -> async_graphql::Result<CrystalWells> {
        Ok(CrystalWells {
            plate_id: self.crystal_plate_id,
            well_number: self.crystal_well_number,
        })
    }

    /// Fetches the information of the compounds soaked from
    /// compound library subgraph
    async fn compounds(&self) -> async_graphql::Result<CompoundInstances> {
        Ok(CompoundInstances {
            plate_id: self.compound_plate_id,
            well_number: self.compound_well_number,
        })
    }
}

#[Object]
impl SoakCompoundQuery {
    /// Fetches all soaked compounds from the database
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

    /// Fetches a single soaked compound from the database
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

    /// Reference resolver for crystal wells
    #[graphql(entity)]
    async fn get_crystal_well_by_plate_id(&self, plate_id: Uuid, well_number: i16) -> CrystalWells {
        CrystalWells {
            plate_id,
            well_number,
        }
    }

    /// Reference resolver for compound wells
    #[graphql(entity)]
    async fn get_compound_instances_by_plate_id(
        &self,
        plate_id: Uuid,
        well_number: i16,
    ) -> CompoundInstances {
        CompoundInstances {
            plate_id,
            well_number,
        }
    }
}

#[Object]
impl SoakCompoundMutation {
    /// Adds a soaked compound to the database
    async fn add_soaked_compound(
        &self,
        ctx: &Context<'_>,
        compound_plate_id: Uuid,
        #[graphql(validator(minimum = 1, maximum = 288))] compound_well_number: i16,
        crystal_plate_id: Uuid,
        #[graphql(validator(minimum = 1, maximum = 288))] crystal_well_number: i16,
        volume: f32,
    ) -> async_graphql::Result<soak_compound::Model> {
        let operator_id =
            subject_authorization!("xchemlab.compound_soaking.write_soaked_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let crystal = soak_compound::ActiveModel {
            compound_plate_id: ActiveValue::Set(compound_plate_id),
            compound_well_number: ActiveValue::Set(compound_well_number),
            crystal_plate_id: ActiveValue::Set(crystal_plate_id),
            crystal_well_number: ActiveValue::Set(crystal_well_number),
            volume: ActiveValue::set(volume),
            operator_id: ActiveValue::Set(operator_id),
            timestamp: ActiveValue::Set(Utc::now()),
        };
        Ok(soak_compound::Entity::insert(crystal)
            .exec_with_returning(db)
            .await?)
    }
}
