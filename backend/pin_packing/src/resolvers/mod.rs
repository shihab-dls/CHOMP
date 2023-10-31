pub mod cane_library;
pub mod cane_mount;
pub mod crystal;
pub mod pin_library;
pub mod pin_mount;
pub mod puck_library;
pub mod puck_mount;

use anyhow::Context;
use async_graphql::{connection::CursorType, InputObject};
use sea_orm::{EntityTrait, PrimaryKeyTrait};
use std::{error::Error, fmt::Debug};
use the_paginator::QueryCursor;

#[derive(Debug, Clone, InputObject)]
struct CursorInput {
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
}

impl CursorInput {
    fn into_query_cursor<Entity>(self) -> Result<QueryCursor<Entity>, anyhow::Error>
    where
        Entity: EntityTrait,
        <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType: CursorType + Clone,
        <<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType as CursorType>::Error:
            Error + Send + Sync,
    {
        let after = match self.after {
            Some(after) => {
                Some(<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType::decode_cursor(&after)?)
            }
            None => None,
        };
        let before = match self.before {
            Some(before) => {
                Some(<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType::decode_cursor(&before)?)
            }
            None => None,
        };
        let first = match self.first {
            Some(first) => Some(first.try_into().context("First must be non-negative")?),
            None => None,
        };
        let last = match self.last {
            Some(last) => Some(last.try_into().context("Last must be non-negative")?),
            None => None,
        };

        Ok(QueryCursor::<Entity>::from_bounds(
            after, before, first, last,
        )?)
    }
}
