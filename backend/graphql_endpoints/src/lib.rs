#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use async_graphql::{
    http::{GraphiQLSource, ALL_WEBSOCKET_PROTOCOLS},
    Data, Executor,
};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{
    body::{boxed, BoxBody, Bytes, HttpBody},
    extract::{FromRequestParts, WebSocketUpgrade},
    handler::Handler,
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    response::{Html, IntoResponse, Response},
    BoxError, RequestExt, TypedHeader,
};
use futures_core::future::BoxFuture;
use opa_client::AuthorizationToken;
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower_service::Service;

/// An [`axum`] [`Handler`] which provides the GraphiQL user interface, pre-configured for use with given GraphQL and websocket subscription endpoints.
///
/// # Example
/// ```
/// use axum::{routing::get, Router};
/// use graphql_endpoints::GraphiQLHandler;
///
/// fn add_graphiql_route(router: Router) -> Router {
///     router.route(
///         "/",
///         get(GraphiQLHandler::new(
///             "/graphql",
///             "/graphql/ws"
///         ))
///     )
/// }
/// ```
#[derive(Debug, Clone)]
pub struct GraphiQLHandler(Html<String>);

impl GraphiQLHandler {
    /// Constructs an instance of the handler with a given GraphQL endpoint and websocket subscription endpoint.
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

/// An [`axum`] [`Handler`] which provides a GraphQL endpoint.
///
/// This endpoint extracts the [`AuthorizationToken`] and injects it into the GraphQL execution.
///
/// # Examples
/// ```
/// use async_graphql::{ObjectType, Schema, SubscriptionType};
/// use axum::{routing::post, Router};
/// use graphql_endpoints::GraphQLHandler;
/// use opa_client::OPAClient;
/// use url::Url;
///
/// fn add_graphql_route<Query, Mutation, Subscription>(
///     router: Router
/// ) -> Router
/// where
///     Query: ObjectType + Clone + Default + 'static,
///     Mutation: ObjectType + Clone + Default + 'static,
///     Subscription: SubscriptionType + Clone + Default + 'static,
///  {
///     let schema = Schema::<Query, Mutation, Subscription>::default();
///     router.route(
///         "/graphql",
///         post(GraphQLHandler::new(schema))
///     )
/// }
/// ```
#[derive(Debug, Clone)]
pub struct GraphQLHandler<E: Executor> {
    executor: E,
}

impl<E: Executor> GraphQLHandler<E> {
    /// Constructs an instance of the handler with the provided schema.
    pub fn new(executor: E) -> Self {
        Self { executor }
    }
}

impl<S, B, E> Handler<((),), S, B> for GraphQLHandler<E>
where
    B: HttpBody + Unpin + Send + Sync + 'static,
    B::Data: Into<Bytes>,
    B::Error: Into<BoxError>,
    S: Send + Sync + 'static,
    E: Executor,
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
                        self.executor
                            .execute(request.into_inner().data(token))
                            .await,
                    )
                    .into_response()
                }
                Err(err) => (StatusCode::BAD_REQUEST, err.0.to_string()).into_response(),
            }
        })
    }
}

/// An [`axum`] [`Service`] which provides a GraphQL WebSocket Subscription endpoint.
///
/// This endpoint extracts the [`AuthorizationToken`] and injects it into the GraphQL execution.
///
/// # Examples
/// ```
/// use async_graphql::{ObjectType, Schema, SubscriptionType};
/// use axum::{routing::post, Router};
/// use graphql_endpoints::GraphQLSubscription;
/// use opa_client::OPAClient;
/// use url::Url;
///
/// fn add_graphql_route<Query, Mutation, Subscription>(
///     router: Router
/// ) -> Router
/// where
///     Query: ObjectType + Clone + Default + 'static,
///     Mutation: ObjectType + Clone + Default + 'static,
///     Subscription: SubscriptionType + Clone + Default + 'static,
///  {
///     let schema = Schema::<Query, Mutation, Subscription>::default();
///     router.route_service(
///         "/graphql",
///         GraphQLSubscription::new(schema)
///     )
/// }
/// ```
#[derive(Debug, Clone)]
pub struct GraphQLSubscription<E> {
    executor: E,
}

impl<E> GraphQLSubscription<E>
where
    E: Executor,
{
    /// Create a GraphQL subscription service.
    pub fn new(executor: E) -> Self {
        Self { executor }
    }
}

impl<B, E> Service<Request<B>> for GraphQLSubscription<E>
where
    B: HttpBody + Send + 'static,
    E: Executor,
{
    type Response = Response<BoxBody>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let executor = self.executor.clone();

        Box::pin(async move {
            let (mut parts, _body) = req.into_parts();

            let protocol = match GraphQLProtocol::from_request_parts(&mut parts, &()).await {
                Ok(protocol) => protocol,
                Err(err) => return Ok(err.into_response().map(boxed)),
            };
            let upgrade = match WebSocketUpgrade::from_request_parts(&mut parts, &()).await {
                Ok(protocol) => protocol,
                Err(err) => return Ok(err.into_response().map(boxed)),
            };

            let executor = executor.clone();

            let mut data = Data::default();
            data.insert(parts.headers.typed_get::<Authorization<Bearer>>());

            let resp = upgrade
                .protocols(ALL_WEBSOCKET_PROTOCOLS)
                .on_upgrade(move |stream| {
                    GraphQLWebSocket::new(stream, executor, protocol)
                        .with_data(data)
                        .serve()
                });
            Ok(resp.into_response().map(boxed))
        })
    }
}
