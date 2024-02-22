/// A collection of resolvers relating to crystal plates
pub mod crystal_plates_res;
/// A collection of resolvers relating to crystal wells
pub mod crystal_wells_res;

use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use crystal_plates_res::{CrystalPlatesMutation, CrystalPlatesQuery};
use crystal_wells_res::{CrystalWellsMutation, CrystalWellsQuery};

/// Combines all query resolvers into a single GraphQL `Query` type.
#[derive(Debug, Clone, MergedObject, Default)]
pub struct Query(CrystalWellsQuery, CrystalPlatesQuery);

/// Combines all mutation resolvers into a single GraphQL `Query` type.
#[derive(Debug, Clone, MergedObject, Default)]
pub struct Mutation(CrystalWellsMutation, CrystalPlatesMutation);

/// Type alias for the complete GraphQL schema.
pub type RootSchema = Schema<Query, Mutation, EmptySubscription>;

/// This function initializes the schema with default instances of `Query`,
/// `Mutation`, and `EmptySubscription`.
pub fn root_schema_builder() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).enable_federation()
}
