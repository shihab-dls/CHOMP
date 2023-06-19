use crate::models::SoakDBReadback;
use async_graphql::{Context, Object};
use itertools::Itertools;
use opa_client::graphql::OPAGuard;
use soakdb_io::{models::MetadataReadback, SoakDB};

#[derive(Debug, Default)]
pub struct ImportQuery;

#[Object]
impl ImportQuery {
    #[graphql(guard = "OPAGuard::new(\"xchemlab.soakdb_interface.allow\")")]
    async fn read(&self, ctx: &Context<'_>, path: String) -> async_graphql::Result<SoakDBReadback> {
        let database = SoakDB::connect(path).await?;
        let metadata = if ctx.look_ahead().field("metadata").exists() {
            database.read_metadata().await?
        } else {
            MetadataReadback::default()
        };
        let wells = if ctx.look_ahead().field("wells").exists() {
            database.read_wells().await?
        } else {
            Vec::default()
        };
        Ok(SoakDBReadback {
            metadata: metadata.into(),
            wells: wells.into_iter().map_into().collect(),
        })
    }
}
