use crate::models::{Metadata, MetadataReadback, Well};
use async_graphql::Object;
use itertools::Itertools;
use opa_client::graphql::OPAGuard;
use soakdb::SoakDB;

#[derive(Debug, Default)]
pub struct ExportMutation;

#[Object]
impl ExportMutation {
    #[graphql(guard = "OPAGuard::new(\"xchemlab.soakdb_interface.allow\")")]
    async fn update_metadata(
        &self,
        path: String,
        visit: Metadata,
    ) -> async_graphql::Result<MetadataReadback> {
        let mut database = SoakDB::connect(path).await?;
        let visit = database.write_metadata(visit.into()).await?;
        Ok(visit.into())
    }

    #[graphql(guard = "OPAGuard::new(\"xchemlab.soakdb_interface.allow\")")]
    async fn insert_wells(
        &self,
        path: String,
        wells: Vec<Well>,
    ) -> async_graphql::Result<Vec<i32>> {
        let mut database = SoakDB::connect(path).await?;
        let ids = database
            .insert_wells(wells.into_iter().map_into().collect())
            .await?
            .collect();
        Ok(ids)
    }
}
