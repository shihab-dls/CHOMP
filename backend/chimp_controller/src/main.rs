#![forbid(unsafe_code)]
#![doc=include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Utilities for handling new images from the targeting service
mod new_image;
/// A collection of GraphQL queries.
pub mod queries;
/// A collection of GraphQL schemas.
mod schemas;

use clap::Parser;
use futures_util::StreamExt;
use new_image::{setup_image_creation_stream, setup_targeting_subscription_client};
use tokio::select;
use url::Url;

/// An shim service instructing CHiMP to perform inference on new targeting images.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// The URL of the Targeting service GraphQL query endpoint.
    targeting_url: Url,
    /// The URL of the Targeting service GraphQL subscription endpoint.
    targeting_subscription_url: Url,
    /// The authorization token to make requests to the targeting service with.
    #[arg(long, env)]
    targeting_token: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = Cli::parse();

    let mut targeting_subscription_client = setup_targeting_subscription_client(
        &args.targeting_subscription_url,
        &args.targeting_token,
    )
    .await
    .unwrap();
    let mut image_creation_stream = setup_image_creation_stream(&mut targeting_subscription_client)
        .await
        .unwrap();

    loop {
        select! {
            Some(image_created) = image_creation_stream.next() => {
                println!("Image created: {image_created:?}")
            }
        }
    }
}
