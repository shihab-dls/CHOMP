use crate::resolvers::{export::ExportMutation, import::ImportQuery};
use async_graphql::{
    extensions::Tracing, http::GraphiQLSource, EmptySubscription, MergedObject, MergedSubscription,
    Schema, SchemaBuilder,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    response::{Html, IntoResponse},
    TypedHeader,
};
use opa_client::{AuthorizationToken, OPAClient};

pub fn build_schema() -> RootSchema {
    schema_builder().extension(Tracing).finish()
}

pub async fn graphiql_handler() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

pub async fn graphql_handler(
    State(schema): State<RootSchema>,
    State(authz_client): State<OPAClient>,
    authorization_header: Option<TypedHeader<Authorization<Bearer>>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let token =
        AuthorizationToken::from(authorization_header.map(|header| header.token().to_string()));
    schema
        .execute(req.into_inner().data(token).data(authz_client))
        .await
        .into()
}

pub type RootSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

pub fn schema_builder() -> SchemaBuilder<RootQuery, RootMutation, EmptySubscription> {
    Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        EmptySubscription::default(),
    )
}

#[derive(Debug, MergedObject, Default)]
pub struct RootQuery(ImportQuery);

#[derive(Debug, MergedObject, Default)]
pub struct RootMutation(ExportMutation);

#[derive(Debug, MergedSubscription, Default)]
pub struct RootSubscription;
