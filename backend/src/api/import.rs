use async_graphql::{Context, Object};
use soakdb::{models::MetadataReadback, read_entries, read_metadata};

#[derive(Debug, Default)]
pub struct ImportQuery;

#[Object]
impl ImportQuery {
    async fn read_metadata(
        &self,
        _ctx: &Context<'_>,
        path: String,
    ) -> async_graphql::Result<MetadataReadback> {
        Ok(read_metadata(&path).await?)
    }
}
