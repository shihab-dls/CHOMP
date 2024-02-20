#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc=include_str!("../README.md")]

/// This module defines the structure and schema of the database tables
/// through various entity structs.
mod entities;
/// This module sets up the GraphQL schema, including queries, mutations,
/// and subscriptions. It defines how data is queried and mutated through the API.
mod graphql;
/// This module is responsible for defining and applying database migrations.
mod migrator;

use async_graphql::extensions::Tracing;
use axum::{routing::get, Router, Server};
use clap::Parser;
use graphql::{root_schema_builder, RootSchema};
use graphql_endpoints::{GraphQLHandler, GraphQLSubscription, GraphiQLHandler};
use opa_client::OPAClient;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr, TransactionError};
use sea_orm_migration::MigratorTrait;
use std::{
    fs::File,
    io::Write,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
};
use url::Url;

/// A service for tracking crystals available in the XChem lab
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    /// Starts a webserver serving the GraphQL API
    Serve(ServeArgs),
    /// Prints the GraphQL API to stdout
    Schema(SchemaArgs),
}

#[derive(Debug, Parser)]
#[allow(clippy::missing_docs_in_private_items)]
struct ServeArgs {
    /// The port number to serve on
    #[arg(short, long, default_value_t = 80)]
    port: u16,
    /// URL for the database
    #[arg(long, env)]
    database_url: Url,
    /// URL for the OPA server
    #[arg(long, env)]
    opa_url: Url,
}

/// Arguments for the `schema` command
#[derive(Debug, Parser)]
struct SchemaArgs {
    /// Specifies an optional path to the file to save the schema
    #[arg(short, long)]
    path: Option<PathBuf>,
}

/// Sets up the database connection and performs the migrations
/// The database name is set of compound_library if not provided
///
/// Returns a `Result` with a `DatabaseConnection` on success,
/// or a `TransactionError<DbErr>` if connecting to the database or running
/// migrations fails
async fn setup_database(
    mut database_url: Url,
) -> Result<DatabaseConnection, TransactionError<DbErr>> {
    if database_url.path().is_empty() {
        database_url.set_path("crystal_library");
    }
    let connection_options = ConnectOptions::new(database_url.to_string());
    let connection = Database::connect(connection_options).await?;
    migrator::Migrator::up(&connection, None).await?;
    Ok(connection)
}

/// Sets up the router for handling GraphQL queries and subscriptions
/// Returns a `Router` configured with routes
fn setup_router(schema: RootSchema) -> Router {
    /// The endpoint for handling GraphQL queries and mutations
    const GRAPHQL_ENDPOINT: &str = "/";
    /// The endpoint for establishing WebSocket connections for GraphQL subscriptions
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

/// Starts a web server to handle HTTP requests as defined in the provided `router`
async fn serve(router: Router, port: u16) {
    let socket_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port));
    println!("GraphiQL IDE: {}", socket_addr);
    Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = Cli::parse();
    let tracing_subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(tracing_subscriber).unwrap();

    match args {
        Cli::Serve(args) => {
            let db = setup_database(args.database_url).await.unwrap();
            let opa_client = OPAClient::new(args.opa_url);
            let schema = root_schema_builder()
                .data(db)
                .data(opa_client)
                .extension(Tracing)
                .finish();
            let router = setup_router(schema);
            serve(router, args.port).await;
        }
        Cli::Schema(args) => {
            let schema = root_schema_builder().finish();
            let schema_string = schema.sdl();
            if let Some(path) = args.path {
                let mut file = File::create(path).unwrap();
                file.write_all(schema_string.as_bytes()).unwrap();
            } else {
                println!("{}", schema_string);
            }
        }
    }
}
