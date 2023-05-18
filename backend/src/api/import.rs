use async_graphql::{Context, Object};
use soakdb::{
    models::{MetadataReadback, WellReadback},
    read_metadata, read_wells,
};

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

    async fn read_wells(
        &self,
        _ctx: &Context<'_>,
        path: String,
    ) -> async_graphql::Result<Vec<WellReadback>> {
        Ok(read_wells(&path).await?)
    }
}
