#![forbid(unsafe_code)]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![doc=include_str!("../README.md")]
mod entities;
mod graphql;
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

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    Serve(ServeArgs),
    Schema(SchemaArgs),
}

#[derive(Debug, Parser)]
struct ServeArgs {
    #[arg(short, long, default_value_t = 80)]
    port: u16,
    #[arg(long, env)]
    database_url: Url,
    #[arg(long, env, default_value = "compound_library")]
    database_path: String,
    #[arg(long, env)]
    opa_url: Url,
}

#[derive(Debug, Parser)]
struct SchemaArgs {
    #[arg(short, long)]
    path: Option<PathBuf>,
}

async fn setup_database(
    db_base: Url,
    db_path: String,
) -> Result<DatabaseConnection, TransactionError<DbErr>> {
    let db_url = format!("{}/{}", db_base, db_path);
    let db_options = ConnectOptions::new(db_url);
    let db = Database::connect(db_options).await?;
    migrator::Migrator::up(&db, None).await?;
    Ok(db)
}

fn setup_router(schema: RootSchema) -> Router {
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
            let db = setup_database(args.database_url, args.database_path)
                .await
                .unwrap();
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