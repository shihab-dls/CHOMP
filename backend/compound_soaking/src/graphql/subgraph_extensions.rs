use async_graphql::SimpleObject;
use uuid::Uuid;

/// CrystalWell is an extension from the crystal library subgraph
#[derive(SimpleObject)]
#[graphql(name = "crystal_wells", complex)]
pub struct CrystalWells {
    /// ID of the plate on which the crystal is located
    pub plate_id: Uuid,
    /// Well on the plate in which crystal is located
    pub well_number: i16,
}

/// CompoundInstances is an extension of compound library subgraph
#[derive(SimpleObject)]
#[graphql(name = "compound_instances", complex)]
pub struct CompoundInstances {
    /// ID of the plate on which the compound is located
    pub plate_id: Uuid,
    /// Well on the plate in which compound is located
    pub well_number: i16,
}
