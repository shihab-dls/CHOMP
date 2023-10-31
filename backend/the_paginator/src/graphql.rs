use crate::{CursorCreationError, CursorPage, QueryCursor};
use async_graphql::{
    connection::{
        Connection, CursorType, DefaultConnectionName, DefaultEdgeName, DisableNodesField, Edge,
        EmptyFields, OpaqueCursor,
    },
    InputObject, OutputType,
};
use sea_orm::{
    sea_query::{FromValueTuple, ValueTuple},
    EntityTrait, Iterable, ModelTrait, PrimaryKeyToColumn, PrimaryKeyTrait,
};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

/// An [`async_graphql`] input object for specifying page by cursor
#[derive(Debug, Clone, InputObject)]
pub struct CursorInput {
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
}

/// An error which occured when attempting to create a [`QueryCursor`] from a [`CursorInput`]
#[derive(Debug, thiserror::Error)]
pub enum CursorInputBuildError<Cursor>
where
    Cursor: Serialize + DeserializeOwned,
{
    /// An error which occured when creating the query cursor
    #[error("An error arose when creating the query cursor")]
    CursorCreationError(#[from] CursorCreationError),
    /// The value of first was negative
    #[error("First must be non-negative")]
    NegativeFirst(<i32 as TryInto<u64>>::Error),
    /// The value of last was negative
    #[error("Last must be non-negative")]
    NegativeLast(<i32 as TryInto<u64>>::Error),
    /// The value of after could not be decoded into the expected key type
    #[error("After could not be de-coded to the expected key type")]
    UndecodableAfter(<OpaqueCursor<Cursor> as CursorType>::Error),
    /// The value of last could not be decoded into the expected key type
    #[error("Before could not be de-coded to the expected key type")]
    UndecodableBefore(<OpaqueCursor<Cursor> as CursorType>::Error),
}

impl CursorInput {
    /// Attempts to create a [`QueryCursor`] from the [`CursorInput`]
    ///
    /// This function will return an error if the combination of values is invalid, the bound are negative, or the cursor value could not be decoded
    pub fn try_into_query_cursor<Entity>(
        self,
    ) -> Result<
        QueryCursor<Entity>,
        CursorInputBuildError<<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    >
    where
        Entity: EntityTrait,
        <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType: Clone + Serialize + DeserializeOwned,
    {
        let after = match self.after {
            Some(after) => Some(
                OpaqueCursor::<<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType>::decode_cursor(
                    &after,
                )
                .map_err(CursorInputBuildError::UndecodableAfter)?
                .0,
            ),
            None => None,
        };
        let before = match self.before {
            Some(before) => Some(
                OpaqueCursor::<<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType>::decode_cursor(
                    &before,
                )
                .map_err(CursorInputBuildError::UndecodableBefore)?
                .0,
            ),
            None => None,
        };
        let first = match self.first {
            Some(first) => Some(
                first
                    .try_into()
                    .map_err(CursorInputBuildError::NegativeFirst)?,
            ),
            None => None,
        };
        let last = match self.last {
            Some(last) => Some(
                last.try_into()
                    .map_err(CursorInputBuildError::NegativeLast)?,
            ),
            None => None,
        };

        Ok(QueryCursor::<Entity>::from_bounds(
            after, before, first, last,
        )?)
    }
}

/// A [`Connection`] which produces pages of a Model
#[allow(type_alias_bounds)]
pub type ModelConnection<Model: ModelTrait> = Connection<
    OpaqueCursor<<<Model::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    Model,
    EmptyFields,
    EmptyFields,
    DefaultConnectionName,
    DefaultEdgeName,
    DisableNodesField,
>;

/// An error produces when primary key extraction fails
#[derive(Debug, thiserror::Error)]
#[error("Failed to extract the primary key")]
pub struct PrimaryKeyExtractionError;

fn try_extract_primary_key<Model>(
    model: &Model,
) -> Result<
    <<Model::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
    PrimaryKeyExtractionError,
>
where
    Model: ModelTrait,
{
    let columns = <Model::Entity as EntityTrait>::PrimaryKey::iter()
        .map(|key| key.into_column())
        .collect::<Vec<_>>();
    let values = match columns.len() {
        1 => Ok(ValueTuple::One(model.get(columns[0]))),
        2 => Ok(ValueTuple::Two(
            model.get(columns[0]),
            model.get(columns[1]),
        )),
        3 => Ok(ValueTuple::Three(
            model.get(columns[0]),
            model.get(columns[1]),
            model.get(columns[2]),
        )),
        4 => Ok(ValueTuple::Four(
            model.get(columns[0]),
            model.get(columns[1]),
            model.get(columns[2]),
            model.get(columns[3]),
        )),
        5 => Ok(ValueTuple::Five(
            model.get(columns[0]),
            model.get(columns[1]),
            model.get(columns[2]),
            model.get(columns[3]),
            model.get(columns[4]),
        )),
        6 => Ok(ValueTuple::Six(
            model.get(columns[0]),
            model.get(columns[1]),
            model.get(columns[2]),
            model.get(columns[3]),
            model.get(columns[4]),
            model.get(columns[5]),
        )),
        _ => Err(PrimaryKeyExtractionError),
    }?;
    Ok(<<Model::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType::from_value_tuple(
            values,
        ))
}

impl<Model> CursorPage<Model>
where
    Model: ModelTrait + OutputType,
    <<Model::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType:
        Serialize + DeserializeOwned + Sync,
{
    /// Converts the [`CursorPage`] into an [`Connection`], with each item as an edge
    pub fn try_into_connection(self) -> Result<ModelConnection<Model>, PrimaryKeyExtractionError> {
        let edges = self
            .items
            .into_iter()
            .map(|item| {
                let primary_key = try_extract_primary_key(&item)?;
                let cursor = OpaqueCursor(primary_key);
                Ok(Edge::new(cursor, item))
            })
            .collect::<Result<Vec<_>, PrimaryKeyExtractionError>>()?;

        let mut connection = Connection::new(self.has_previous, self.has_next);
        connection.edges.extend(edges);
        Ok(connection)
    }
}
