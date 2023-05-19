use async_graphql::Object;
use soakdb::{
    insert_wells,
    models::{Metadata, MetadataReadback, Well},
    write_metadata,
};
use tracing::debug;

#[derive(Debug, Default)]
pub struct ExportMutation;

#[Object]
impl ExportMutation {
    async fn update_metadata(
        &self,
        path: String,
        visit: Metadata,
    ) -> async_graphql::Result<MetadataReadback> {
        debug!("Writing metadata to '{}'", path);
        let visit = write_metadata(&path, visit).await?;
        debug!("Wrote metadata to '{}'", path);
        Ok(visit)
    }

    async fn insert_wells(
        &self,
        path: String,
        wells: Vec<Well>,
    ) -> async_graphql::Result<Vec<i32>> {
        let ids = insert_wells(path, wells).await?.collect();
        Ok(ids)
    }
}
