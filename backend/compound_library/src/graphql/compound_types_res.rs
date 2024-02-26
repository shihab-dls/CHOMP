use crate::tables::{compound_instances, compound_types};
use async_graphql::{ComplexObject, Context, CustomValidator, InputValueError, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use purr::{
    graph::Builder,
    read::{read, Trace},
};
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use the_paginator::graphql::{CursorInput, ModelConnection};

/// CompundQuery is a type that represents all the queries for the compound types.
#[derive(Debug, Clone, Default)]
pub struct CompoundQuery;

/// CompundMutation is a type that represents all the mutations for the compound types.
#[derive(Debug, Clone, Default)]
pub struct CompoundMutation;

#[ComplexObject]
impl compound_types::Model {
    /// This function fetches all compound instances related to this compound type.
    async fn instances(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<compound_instances::Model>> {
        subject_authorization!("xchemlab.compound_library.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(self
            .find_related(compound_instances::Entity)
            .all(db)
            .await?)
    }
}

#[Object]
impl CompoundQuery {
    /// This function fetches all compound types from the database.
    async fn compounds(
        &self,
        ctx: &Context<'_>,
        cursor: CursorInput,
    ) -> async_graphql::Result<ModelConnection<compound_types::Model>> {
        subject_authorization!("xchemlab.compound_library.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(cursor
            .try_into_query_cursor::<compound_types::Entity>()?
            .all(db)
            .await?
            .try_into_connection()?)
    }

    /// This function fetches a single compound type using the compound name.
    async fn compound(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> async_graphql::Result<Option<compound_types::Model>> {
        subject_authorization!("xchemlab.compound_library.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(compound_types::Entity::find()
            .filter(compound_types::Column::Name.eq(name.to_ascii_lowercase()))
            .one(db)
            .await?)
    }
}

/// SmilesValidator is a type for custom graphql validation.
struct SmilesValidator;

impl SmilesValidator {
    /// This function checks if the given `smiles` string is in correct format.
    fn valid_smiles(smiles: &str) -> bool {
        let mut builder = Builder::new();
        let mut trace = Trace::new();
        read(smiles, &mut builder, Some(&mut trace)).is_ok()
    }
}

impl CustomValidator<String> for SmilesValidator {
    fn check(&self, smiles: &String) -> Result<(), InputValueError<String>> {
        SmilesValidator::valid_smiles(smiles)
            .then_some(())
            .ok_or(InputValueError::custom("Invalid SMILES format"))
    }
}

#[Object]
impl CompoundMutation {
    /// This function adds a compound type to the database.
    async fn add_compound(
        &self,
        ctx: &Context<'_>,
        name: String,
        #[graphql(validator(custom = "SmilesValidator"))] smiles: String,
    ) -> async_graphql::Result<compound_types::Model> {
        let operator_id =
            subject_authorization!("xchemlab.compound_library.write_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let compound = compound_types::ActiveModel {
            name: ActiveValue::set(name.to_ascii_lowercase()),
            smiles: ActiveValue::Set(smiles.to_ascii_uppercase()),
            operator_id: ActiveValue::Set(operator_id),
            timestamp: ActiveValue::Set(Utc::now()),
        };
        Ok(compound_types::Entity::insert(compound)
            .exec_with_returning(db)
            .await?)
    }
}
