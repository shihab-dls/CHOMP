use async_graphql::{MergedObject, MergedSubscription, Schema, SchemaBuilder};

pub type RootSchema = Schema<RootQuery, RootMutation, RootSubscription>;

pub fn schema_builder() -> SchemaBuilder<RootQuery, RootMutation, RootSubscription> {
    Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        RootSubscription::default(),
    )
}

#[derive(Debug, MergedObject, Default)]
pub struct RootQuery;

#[derive(Debug, MergedObject, Default)]
pub struct RootMutation;

#[derive(Debug, MergedSubscription, Default)]
pub struct RootSubscription;
