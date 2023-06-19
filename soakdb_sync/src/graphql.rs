use crate::resolvers::{export::ExportMutation, import::ImportQuery};
use async_graphql::{
    extensions::Tracing, http::GraphiQLSource, EmptySubscription, MergedObject, MergedSubscription,
    Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    body::{Bytes, HttpBody},
    handler::Handler,
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    response::{Html, IntoResponse, Response},
    BoxError, RequestExt, TypedHeader,
};
use derive_more::Constructor;
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

    fn call(self, _req: Request<B>, _state: S) -> Self::Future {
        Box::pin(async { self.0.into_response() })
    }
}

#[derive(Clone, Constructor)]
pub struct GraphQLHandler {
    schema: RootSchema,
    opa_client: OPAClient,
}

impl<S, B> Handler<((),), S, B> for GraphQLHandler
where
    B: HttpBody + Unpin + Send + Sync + 'static,
    B::Data: Into<Bytes>,
    B::Error: Into<BoxError>,
    S: Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send + 'static>>;

    fn call(self, mut req: Request<B>, _state: S) -> Self::Future {
        Box::pin(async move {
            let token = req
                .extract_parts::<TypedHeader<Authorization<Bearer>>>()
                .await
                .ok();
            let request = req.extract::<GraphQLRequest, _>().await;
            match request {
                Ok(request) => {
                    let token =
                        AuthorizationToken::new(token.map(|token| token.token().to_string()));
                    GraphQLResponse::from(
                        self.schema
                            .execute(request.into_inner().data(token).data(self.opa_client))
                            .await,
                    )
                    .into_response()
                }
                Err(err) => (StatusCode::BAD_REQUEST, err.0.to_string()).into_response(),
            }
        })
    }
}

pub type RootSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

#[derive(Debug, MergedObject, Default)]
pub struct RootQuery(ImportQuery);

#[derive(Debug, MergedObject, Default)]
pub struct RootMutation(ExportMutation);

#[derive(Debug, MergedSubscription, Default)]
pub struct RootSubscription;
