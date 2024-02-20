/// The cyrstal wells resolver module
pub mod crystal_wells_res;

use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use crystal_wells_res::{CrystalMutation, CrystalQuery};

/// Combines all query resolvers into a single GraphQL `Query` type.
#[derive(Debug, Clone, MergedObject, Default)]
pub struct Query(CrystalQuery);

/// Combines all mutation resolvers into a single GraphQL `Query` type.
#[derive(Debug, Clone, MergedObject, Default)]
pub struct Mutation(CrystalMutation);

/// Type alias for the complete GraphQL schema.
pub type RootSchema = Schema<Query, Mutation, EmptySubscription>;

/// This function initializes the schema with default instances of `Query`,
/// `Mutation`, and `EmptySubscription`.
pub fn root_schema_builder() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).enable_federation()
}
