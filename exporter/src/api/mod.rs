use async_graphql::{MergedObject, MergedSubscription, Schema};

pub type RootSchema = Schema<RootQuery, RootMutation, RootSubscription>;

#[derive(Debug, MergedObject, Default)]
pub struct RootQuery;

#[derive(Debug, MergedObject, Default)]
pub struct RootMutation;

#[derive(Debug, MergedSubscription, Default)]
pub struct RootSubscription;
