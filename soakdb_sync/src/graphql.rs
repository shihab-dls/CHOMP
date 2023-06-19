use crate::resolvers::{export::ExportMutation, import::ImportQuery};
use async_graphql::{
    extensions::Tracing, EmptySubscription, MergedObject, MergedSubscription, Schema,
};

pub fn build_schema() -> RootSchema {
    Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        EmptySubscription::default(),
    )
    .extension(Tracing)
    .finish()
}

pub type RootSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

#[derive(Debug, Clone, MergedObject, Default)]
pub struct RootQuery(ImportQuery);

#[derive(Debug, Clone, MergedObject, Default)]
pub struct RootMutation(ExportMutation);

#[derive(Debug, Clone, MergedSubscription, Default)]
pub struct RootSubscription;
