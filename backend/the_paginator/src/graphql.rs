use crate::{CursorCreationError, CursorPage, QueryCursor};
use async_graphql::{
    connection::{
        Connection, CursorType, DefaultConnectionName, DefaultEdgeName, DisableNodesField, Edge,
        EmptyFields,
    },
    InputObject, OutputType,
};
use sea_orm::{EntityTrait, ModelTrait, PrimaryKeyTrait};
use std::{error::Error, fmt::Debug};

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
    Cursor: CursorType,
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
    UndecodableAfter(Cursor::Error),
    /// The value of last could not be decoded into the expected key type
    #[error("Before could not be de-coded to the expected key type")]
    UndecodableBefore(Cursor::Error),
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
        <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType: CursorType + Clone,
        <<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType as CursorType>::Error:
            Error + Send + Sync,
    {
        let after = match self.after {
            Some(after) => Some(
                <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType::decode_cursor(&after)
                    .map_err(CursorInputBuildError::UndecodableAfter)?,
            ),
            None => None,
        };
        let before = match self.before {
            Some(before) => Some(
                <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType::decode_cursor(&before)
                    .map_err(CursorInputBuildError::UndecodableBefore)?,
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
    <<Model::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
    Model,
    EmptyFields,
    EmptyFields,
    DefaultConnectionName,
    DefaultEdgeName,
    DisableNodesField,
>;

impl<Model> CursorPage<Model>
where
    Model: ModelTrait + OutputType,
    <<Model as ModelTrait>::Entity as EntityTrait>::Model: OutputType,
    <<Model::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: CursorType + Sync,
{
    /// Converts the [`CursorPage`] into an [`Connection`], with each item as an edge
    pub fn into_connection(
        self,
        extract_cursor: impl Fn(&Model) -> <<Model::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> ModelConnection<Model> {
        let mut connection = Connection::new(self.has_previous, self.has_next);
        connection.edges.extend(
            self.items
                .into_iter()
                .map(|item| Edge::new(extract_cursor(&item), item)),
        );
        connection
    }
}
