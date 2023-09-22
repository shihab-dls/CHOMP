use aws_credential_types::{provider::SharedCredentialsProvider, Credentials};
use aws_sdk_s3::{config::Region, Client};
use clap::Parser;
use url::Url;

/// Arguments for configuring the S3 Client.
#[derive(Debug, Parser)]
pub struct S3ClientArgs {
    /// The URL of the S3 endpoint to retrieve images from.
    #[arg(long, env)]
    endpoint_url: Option<Url>,
    /// The ID of the access key used for S3 authorization.
    #[arg(long, env)]
    access_key_id: Option<String>,
    /// The secret access key used for S3 authorization.
    #[arg(long, env)]
    secret_access_key: Option<String>,
    /// Forces path style endpoint URIs for S3 queries.
    #[arg(long, env)]
    force_path_style: Option<bool>,
    /// The AWS region of the S3 bucket.
    #[arg(long, env)]
    region: Option<String>,
}

pub trait FromS3ClientArgs {
    /// Creates a S3 [`Client`] with the supplied credentials using the supplied endpoint configuration.
    fn from_s3_client_args(args: S3ClientArgs) -> Self;
}

impl FromS3ClientArgs for Client {
    fn from_s3_client_args(args: S3ClientArgs) -> Self {
        let credentials = Credentials::new(
            args.access_key_id.unwrap_or_default(),
            args.secret_access_key.unwrap_or_default(),
            None,
            None,
            "chimp-chomp-cli",
        );
        let credentials_provider = SharedCredentialsProvider::new(credentials);
        let config = aws_sdk_s3::config::Builder::new()
            .set_credentials_provider(Some(credentials_provider))
            .set_endpoint_url(args.endpoint_url.map(String::from))
            .set_force_path_style(args.force_path_style)
            .set_region(Some(Region::new(
                args.region.unwrap_or(String::from("undefined")),
            )))
            .clone()
            .build();
        Client::from_conf(config)
    }
}
