pub mod export;
pub mod import;

use self::{export::ExportMutation, import::ImportQuery};
use async_graphql::{EmptySubscription, MergedObject, MergedSubscription, Schema, SchemaBuilder};

pub type RootSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

pub fn schema_builder() -> SchemaBuilder<RootQuery, RootMutation, EmptySubscription> {
    Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        EmptySubscription::default(),
    )
}

#[derive(Debug, MergedObject, Default)]
pub struct RootQuery(ImportQuery);

#[derive(Debug, MergedObject, Default)]
pub struct RootMutation(ExportMutation);

#[derive(Debug, MergedSubscription, Default)]
pub struct RootSubscription;
