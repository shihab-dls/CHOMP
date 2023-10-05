#![forbid(unsafe_code)]
#![doc=include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Utilities for handling messages from CHiMP
mod chimp_messages;
/// Utilities for handling images which existed before this started
mod existing_images;
/// Utilities for handling redictionmages from the targeting service
mod new_image;
/// Utilities for handling new predictions from CHiMP
mod new_prediction;
/// A collection of GraphQL queries.
pub mod queries;
/// A collection of GraphQL schemas.
mod schemas;

use crate::{
    chimp_messages::setup_chimp_client,
    existing_images::{get_unprocessed_images, handle_existing_image},
    new_image::{
        handle_new_image, setup_image_creation_stream, setup_targeting_subscription_client,
    },
    new_prediction::handle_new_prediction,
};
use clap::Parser;
use futures_util::StreamExt;
use tokio::{select, task::JoinSet};
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
    /// The URL of the RabbitMQ server.
    rabbitmq_url: Url,
    /// The RabbitMQ queue on which jobs are assigned.
    rabbitmq_channel: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = Cli::parse();

    let targeting_client = reqwest::Client::new();
    let mut targeting_subscription_client = setup_targeting_subscription_client(
        &args.targeting_subscription_url,
        &args.targeting_token,
    )
    .await
    .unwrap();
    let mut image_creation_stream = setup_image_creation_stream(&mut targeting_subscription_client)
        .await
        .unwrap();

    let (request_publisher, prediction_consumer) =
        setup_chimp_client(args.rabbitmq_url, args.rabbitmq_channel)
            .await
            .unwrap();
    let mut prediction_stream = prediction_consumer.into_prediction_stream();

    let mut tasks = JoinSet::new();

    let unprocessed_images = get_unprocessed_images(
        &targeting_client,
        args.targeting_url.clone(),
        &args.targeting_token.clone(),
    )
    .await
    .unwrap();

    for unprocessed_image in unprocessed_images {
        println!("Processing: {unprocessed_image:?}");
        tasks.spawn(handle_existing_image(
            unprocessed_image,
            request_publisher.clone(),
        ));
    }

    loop {
        select! {
            Some(image_created) = image_creation_stream.next() => {
                tasks.spawn(handle_new_image(image_created, request_publisher.clone()));
            },

            Some(prediction) = prediction_stream.next() => {
                tasks.spawn(handle_new_prediction(prediction, targeting_client.clone(), args.targeting_url.clone(), args.targeting_token.clone()));
            }
        }
    }
}
