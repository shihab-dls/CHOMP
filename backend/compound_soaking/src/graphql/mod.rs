/// A collection of resolvers relating to soaked compounds
pub mod soak_compound_res;

use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use soak_compound_res::{SoakCompoundMutation, SoakCompoundQuery};

/// Combines all query resolvers into a single GraphQL `Query` type.
#[derive(Debug, Clone, MergedObject, Default)]
pub struct Query(SoakCompoundQuery);

/// Combines all mutations resolvers into a single GraphQL `Mutation` type.
#[derive(Debug, Clone, MergedObject, Default)]
pub struct Mutation(SoakCompoundMutation);

/// Type alias for the complete GraphQL schema.
pub type RootSchema = Schema<Query, Mutation, EmptySubscription>;

/// Initializes the schema with default instances of `Query`,
/// `Mutation`, and `EmptySubscription`.
pub fn root_schema_builder() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).enable_federation()
}
