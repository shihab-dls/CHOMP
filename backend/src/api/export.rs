use async_graphql::Object;
use soakdb::{
    models::{Metadata, MetadataReadback},
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
}
