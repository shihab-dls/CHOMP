use async_graphql::Object;
use soakdb::{
    models::{Visit, VisitReadback},
    write_visit,
};
use tracing::debug;

#[derive(Debug, Default)]
pub struct ExportMutation;

#[Object]
impl ExportMutation {
    async fn update_visit(
        &self,
        path: String,
        visit: Visit,
    ) -> async_graphql::Result<VisitReadback> {
        debug!("Writing metadata to '{}'", path);
        let visit = write_visit(&path, visit).await?;
        debug!("Wrote metadata to '{}'", path);
        Ok(visit)
    }
}
