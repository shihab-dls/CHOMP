use async_graphql::Object;
use derive_more::{Deref, DerefMut, From};

#[derive(Debug, Clone, Deref, DerefMut, From)]
pub struct Visit(soakdb::models::Visit);

#[Object]
impl Visit {
    async fn name(&self) -> &String {
        &self.name
    }

    async fn protein(&self) -> &String {
        &self.protein
    }
}

#[derive(Debug, Clone, Deref, DerefMut, From)]
pub struct VisitReadback(soakdb::models::VisitReadback);

#[Object]
impl VisitReadback {
    async fn name(&self) -> &Option<String> {
        &self.name
    }

    async fn protein(&self) -> &Option<String> {
        &self.protein
    }
}
