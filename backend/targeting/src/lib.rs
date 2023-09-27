mod graphql;
mod migrations;
mod resolvers;
mod tables;

use anyhow::Context;
use aws_sdk_s3::Client;
use axum::{routing::get, Router, Server};
use derive_more::{Deref, FromStr, Into};
pub use graphql::root_schema_builder;
use graphql::RootSchema;
use graphql_endpoints::{GraphQLHandler, GraphQLSubscription, GraphiQLHandler};
use migrations::Migrator;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr, TransactionError};
use sea_orm_migration::MigratorTrait;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use url::Url;

pub fn setup_router(schema: RootSchema) -> Router {
    const GRAPHQL_ENDPOINT: &str = "/";
    const SUBSCRIPTION_ENDPOINT: &str = "/ws";

    Router::new()
        .route(
            GRAPHQL_ENDPOINT,
            get(GraphiQLHandler::new(
                GRAPHQL_ENDPOINT,
                SUBSCRIPTION_ENDPOINT,
            ))
            .post(GraphQLHandler::new(schema.clone())),
        )
        .route_service(SUBSCRIPTION_ENDPOINT, GraphQLSubscription::new(schema))
}

pub async fn setup_database(
    database_url: Url,
) -> Result<DatabaseConnection, TransactionError<DbErr>> {
    let connection_options = ConnectOptions::new(database_url.to_string());
    let connection = Database::connect(connection_options).await?;
    Migrator::up(&connection, None).await?;
    Ok(connection)
}

pub async fn setup_bucket(s3_client: &Client, bucket: S3Bucket) -> Result<(), anyhow::Error> {
    match s3_client.create_bucket().bucket(bucket).send().await {
        Ok(_) => Ok(()),
        Err(err) => {
            let err = err.into_service_error();
            if err.is_bucket_already_owned_by_you() {
                Ok(())
            } else {
                Err(err).context("Failed to create bucket")
            }
        }
    }
}

#[derive(Debug, Clone, Deref, FromStr, Into)]
pub struct S3Bucket(String);

pub async fn serve(router: Router, port: u16) {
    let socket_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port));
    println!("GraphiQL IDE: {}", socket_addr);
    Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
