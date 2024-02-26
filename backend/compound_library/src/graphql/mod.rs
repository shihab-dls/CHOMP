/// The compound instances resolver module.
mod compound_instances_res;
/// The compound types resolver module.
mod compound_types_res;

use compound_instances_res::{CompoundInstanceMutation, CompoundInstanceQuery};
use compound_types_res::{CompoundMutation, CompoundQuery};

use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

/// Combines all query resolvers into a single GraphQL `Query` type.
#[derive(Debug, Clone, MergedObject, Default)]
pub struct Query(CompoundQuery, CompoundInstanceQuery);

/// Combines all mutation resolvers into a single GraphQL `Query` type.
#[derive(Debug, Clone, MergedObject, Default)]
pub struct Mutation(CompoundMutation, CompoundInstanceMutation);

/// Type alias for the complete GraphQL schema.
pub type RootSchema = Schema<Query, Mutation, EmptySubscription>;

/// This function initializes the schema with default instances of `Query`,
/// `Mutation`, and `EmptySubscription`.
pub fn root_schema_builder() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).enable_federation()
}
