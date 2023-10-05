use async_graphql::extensions::Tracing;
use aws_credential_types::{provider::SharedCredentialsProvider, Credentials};
use aws_sdk_s3::{config::Region, Client};
use clap::{ArgAction::SetTrue, Parser};
use opa_client::OPAClient;
use std::{fs::File, io::Write, path::PathBuf};
use targeting::{root_schema_builder, serve, setup_bucket, setup_database, setup_router, S3Bucket};
use url::Url;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[allow(clippy::large_enum_variant)]
enum Cli {
    /// Starts a webserver serving the GraphQL API
    Serve(ServeArgs),
    /// Prints the GraphQL API to stdout
    Schema(SchemaArgs),
}

#[derive(Debug, Parser)]
struct ServeArgs {
    /// The port number to serve on.
    #[arg(short, long, default_value_t = 80)]
    port: u16,
    /// The URL of a postgres database which will be used to persist service data.
    #[arg(long, env)]
    database_url: Url,
    /// The S3 bucket which images are to be stored in.
    #[arg(long, env)]
    s3_bucket: S3Bucket,
    /// Skip creation of the S3 bucket.
    #[arg(long, env, action = SetTrue)]
    s3_create_bucket: bool,
    /// Configuration argument of the S3 client.
    #[command(flatten)]
    s3_client: S3ClientArgs,
    /// The URL of an Open Policy Agent instance serving the required policy endpoints.
    #[arg(long, env)]
    opa_url: Url,
}

/// Arguments for configuring the S3 Client.
#[derive(Debug, Parser)]
pub struct S3ClientArgs {
    /// The URL of the S3 endpoint to retrieve images from.
    #[arg(long, env)]
    s3_endpoint_url: Option<Url>,
    /// The ID of the access key used for S3 authorization.
    #[arg(long, env)]
    s3_access_key_id: Option<String>,
    /// The secret access key used for S3 authorization.
    #[arg(long, env)]
    s3_secret_access_key: Option<String>,
    /// Forces path style endpoint URIs for S3 queries.
    #[arg(long, env, action = SetTrue)]
    s3_force_path_style: bool,
    /// The AWS region of the S3 bucket.
    #[arg(long, env)]
    s3_region: Option<String>,
}

pub trait FromS3ClientArgs {
    /// Creates a S3 [`Client`] with the supplied credentials using the supplied endpoint configuration.
    fn from_s3_client_args(args: S3ClientArgs) -> Self;
}

impl FromS3ClientArgs for Client {
    fn from_s3_client_args(args: S3ClientArgs) -> Self {
        let credentials = Credentials::new(
            args.s3_access_key_id.unwrap_or_default(),
            args.s3_secret_access_key.unwrap_or_default(),
            None,
            None,
            "chimp-chomp-cli",
        );
        let credentials_provider = SharedCredentialsProvider::new(credentials);
        let mut config_builder = aws_sdk_s3::config::Builder::new();
        config_builder.set_credentials_provider(Some(credentials_provider));
        config_builder.set_endpoint_url(args.s3_endpoint_url.map(String::from));
        config_builder.set_force_path_style(Some(args.s3_force_path_style));
        config_builder.set_region(Some(Region::new(
            args.s3_region.unwrap_or(String::from("undefined")),
        )));
        let config = config_builder.build();
        Client::from_conf(config)
    }
}

#[derive(Debug, Parser)]
struct SchemaArgs {
    /// The file path to write the schema to. If not supplied the schema will be printed to stdout.
    #[arg(short, long)]
    path: Option<PathBuf>,
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
            let opa_client = OPAClient::new(args.opa_url);
            let database = setup_database(args.database_url).await.unwrap();
            let s3_client = aws_sdk_s3::Client::from_s3_client_args(args.s3_client);
            if args.s3_create_bucket {
                setup_bucket(&s3_client, args.s3_bucket.clone())
                    .await
                    .unwrap();
            }
            let schema = root_schema_builder()
                .extension(Tracing)
                .data(opa_client)
                .data(database)
                .data(s3_client)
                .data(args.s3_bucket)
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
