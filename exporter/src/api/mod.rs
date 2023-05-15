pub mod import;

use self::import::ImportQuery;
use async_graphql::{
    EmptyMutation, EmptySubscription, MergedObject, MergedSubscription, Schema, SchemaBuilder,
};

pub type RootSchema = Schema<RootQuery, EmptyMutation, EmptySubscription>;

pub fn schema_builder() -> SchemaBuilder<RootQuery, EmptyMutation, EmptySubscription> {
    Schema::build(
        RootQuery::default(),
        EmptyMutation::default(),
        EmptySubscription::default(),
    )
}

#[derive(Debug, MergedObject, Default)]
pub struct RootQuery(ImportQuery);

#[derive(Debug, MergedObject, Default)]
pub struct RootMutation;

#[derive(Debug, MergedSubscription, Default)]
pub struct RootSubscription;
