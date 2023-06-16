use crate::models::{MetadataReadback, WellReadback};
use async_graphql::{Context, Object};
use itertools::Itertools;
use opa_client::graphql::OPAGuard;
use soakdb::SoakDB;

#[derive(Debug, Default)]
pub struct ImportQuery;

#[Object]
impl ImportQuery {
    #[graphql(guard = "OPAGuard::new(\"xchemlab.soakdb_interface.allow\")")]
    async fn read_metadata(
        &self,
        _ctx: &Context<'_>,
        path: String,
    ) -> async_graphql::Result<MetadataReadback> {
        let database = SoakDB::connect(path).await?;
        Ok(database.read_metadata().await?.into())
    }

    #[graphql(guard = "OPAGuard::new(\"xchemlab.soakdb_interface.allow\")")]
    async fn read_wells(
        &self,
        _ctx: &Context<'_>,
        path: String,
    ) -> async_graphql::Result<Vec<WellReadback>> {
        let database = SoakDB::connect(path).await?;
        Ok(database
            .read_wells()
            .await?
            .into_iter()
            .map_into()
            .collect())
    }
}
