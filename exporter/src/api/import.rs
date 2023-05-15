use crate::models::VisitReadback;
use async_graphql::{Context, Object};
use soakdb::read_visit;

#[derive(Debug, Default)]
pub struct ImportQuery;

#[Object]
impl ImportQuery {
    async fn read_visit(
        &self,
        _ctx: &Context<'_>,
        path: String,
    ) -> async_graphql::Result<VisitReadback> {
        Ok(read_visit(&path).await?.into())
    }
}
