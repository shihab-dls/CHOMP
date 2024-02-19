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
#[allow(clippy::large_enum_variant)]
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
    #[arg(long, env)]
    opa_url: Url,
}

#[derive(Debug, Parser)]
struct SchemaArgs {
    #[arg(short, long)]
    path: Option<PathBuf>,
}

async fn setup_database() -> Result<DatabaseConnection, TransactionError<DbErr>> {
    let db_url =
        ConnectOptions::new("postgres://postgres:password@postgres/crystal_library".to_string());
    let db = Database::connect(db_url).await?;
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

async fn serve(router: Router) {
    let socket_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 82));
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
            let db = setup_database().await.unwrap();
            let opa_client = OPAClient::new(args.opa_url);
            let schema = root_schema_builder()
                .data(db)
                .data(opa_client)
                .extension(Tracing)
                .finish();
            let router = setup_router(schema);
            serve(router).await;
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
