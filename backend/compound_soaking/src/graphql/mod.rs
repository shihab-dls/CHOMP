pub mod soak_compound_res;

use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use soak_compound_res::{SoakCompoundMutation, SoakCompoundQuery};

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Query(SoakCompoundQuery);

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Mutation(SoakCompoundMutation);

pub type RootSchema = Schema<Query, Mutation, EmptySubscription>;
pub fn root_schema_builder() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).enable_federation()
}
