// src/graphql.rs

use crate::resolvers::crystal_wells_res::{CrystalMutation, CrystalQuery};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Query(CrystalQuery);

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Mutation(CrystalMutation);

pub type RootSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn root_schema_builder() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).enable_federation()
}
