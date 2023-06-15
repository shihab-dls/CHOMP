use crate::resolvers::{export::ExportMutation, import::ImportQuery};
use async_graphql::{
    extensions::Tracing, http::GraphiQLSource, EmptySubscription, MergedObject, MergedSubscription,
    Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    handler::Handler,
    headers::{authorization::Bearer, Authorization},
    response::{Html, IntoResponse, Response},
    TypedHeader,
};
use opa_client::{AuthorizationToken, OPAClient};
use std::{future::Future, pin::Pin};

pub fn build_schema() -> RootSchema {
    Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        EmptySubscription::default(),
    )
    .extension(Tracing)
    .finish()
}

#[derive(Debug, Clone)]
pub struct GraphiQLHandler(Html<String>);

impl GraphiQLHandler {
    pub fn new(graphql_endpoint: impl AsRef<str>, subscription_endpoint: impl AsRef<str>) -> Self {
        Self(Html(
            GraphiQLSource::build()
                .endpoint(graphql_endpoint.as_ref())
                .subscription_endpoint(subscription_endpoint.as_ref())
                .finish(),
        ))
    }
}

impl<S, B> Handler<((),), S, B> for GraphiQLHandler {
    type Future = Pin<Box<dyn Future<Output = Response> + Send + 'static>>;

    fn call(self, _req: axum::http::Request<B>, _state: S) -> Self::Future {
        Box::pin(async { self.0.into_response() })
    }
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

#[derive(Debug, MergedObject, Default)]
pub struct RootQuery(ImportQuery);

#[derive(Debug, MergedObject, Default)]
pub struct RootMutation(ExportMutation);

#[derive(Debug, MergedSubscription, Default)]
pub struct RootSubscription;
