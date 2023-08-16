use crate::{
    image_loading::{load_image, ChimpImage, WellImage},
    postprocessing::Contents,
};
use anyhow::anyhow;
use aws_sdk_s3::Client;
use chimp_protocol::{Circle, Request, Response};
use derive_more::{Deref, From};
use futures::StreamExt;
use lapin::{
    acker::Acker,
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, BasicRejectOptions,
        QueueDeclareOptions,
    },
    types::{FieldTable, ShortString},
    BasicProperties, Channel, Connection, Consumer,
};
use tokio::sync::mpsc::{OwnedPermit, UnboundedSender};
use url::Url;
use uuid::Uuid;

/// Creates a RabbitMQ [`Connection`] with [`Default`] [`lapin::ConnectionProperties`].
///
/// Returns a [`anyhow::Error`] if the URL could not be built or a connection could not be established.
pub async fn setup_rabbitmq_client(address: Url) -> Result<Connection, anyhow::Error> {
    Ok(
        lapin::Connection::connect(address.as_str(), lapin::ConnectionProperties::default())
            .await?,
    )
}

/// Joins a RabbitMQ channel, creating a [`Consumer`] with [`Default`] [`BasicConsumeOptions`] and [`FieldTable`].
/// The consumer tag is generated following the format `chimp_chomp_${`[`Uuid::new_v4`]`}`.
///
/// Returns a [`lapin::Error`] if the requested channel is not available.
pub async fn setup_job_consumer(
    rabbitmq_channel: Channel,
    channel: impl AsRef<str>,
) -> Result<Consumer, lapin::Error> {
    let worker_id = Uuid::new_v4();
    let worker_tag = format!("chimp_chomp_{worker_id}");
    rabbitmq_channel
        .queue_declare(
            channel.as_ref(),
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    rabbitmq_channel
        .basic_consume(
            channel.as_ref(),
            &worker_tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
}

/// The target of a response.
#[derive(Debug)]
pub struct ResponseTarget {
    /// The acker used to acknowledge the request.
    acker: Acker,
    /// The queue which should recieve the reply message.
    reply_to: ReplyTo,
}

/// The reply channel specified by the requester.
#[derive(Debug, Deref, From)]
pub struct ReplyTo(ShortString);

/// Reads a message from the [`lapin::Consumer`] then loads and prepares the requested image for downstream processing.
///
/// An [`OwnedPermit`] to send to the chimp [`tokio::sync::mpsc::channel`] is required such that backpressure is be propagated to message consumption.
///
/// The prepared images are sent over a [`tokio::sync::mpsc::channel`] and [`tokio::sync::mpsc::unbounded_channel`] if sucessful.
/// An [`anyhow::Error`] is sent if the image could not be read or is empty.
#[allow(clippy::too_many_arguments)]
pub async fn consume_job(
    mut consumer: Consumer,
    s3_client: Client,
    s3_bucket: String,
    input_width: u32,
    input_height: u32,
    chimp_permit: OwnedPermit<(ChimpImage, Request)>,
    well_image_tx: UnboundedSender<(WellImage, Request)>,
    response_target_tx: UnboundedSender<(ResponseTarget, Request)>,
    error_tx: UnboundedSender<(anyhow::Error, Request)>,
) {
    let delivery = consumer.next().await.unwrap().unwrap();

    let acker = delivery.acker;
    let reply_to = match delivery.properties.reply_to().clone() {
        Some(reply_to) => Ok(reply_to),
        None => {
            acker.reject(BasicRejectOptions::default()).await.unwrap();
            Err(anyhow!("Request did not define reply queue"))
        }
    }
    .unwrap()
    .into();
    let request = match Request::from_slice(&delivery.data) {
        Ok(request) => Ok(request),
        Err(error) => {
            acker.reject(BasicRejectOptions::default()).await.unwrap();
            Err(error)
        }
    }
    .unwrap();
    println!("Consumed Request: {request:?}");

    response_target_tx
        .send((ResponseTarget { acker, reply_to }, request.clone()))
        .unwrap();

    match load_image(
        s3_client,
        s3_bucket,
        request.key.clone(),
        input_width,
        input_height,
    )
    .await
    {
        Ok((chimp_image, well_image)) => {
            chimp_permit.send((chimp_image, request.clone()));
            well_image_tx
                .send((well_image, request))
                .map_err(|_| anyhow::Error::msg("Could not send well image"))
                .unwrap()
        }
        Err(err) => error_tx.send((err, request)).unwrap(),
    };
}

/// Takes the results of postprocessing and well centering and publishes a [`Response::Success`] to the RabbitMQ [`Channel`] provided by the [`ResponseTarget`].
pub async fn produce_response(
    request: Request,
    response_target: ResponseTarget,
    contents: Contents,
    well_location: Circle,
    rabbitmq_channel: Channel,
) {
    println!("Producing response for: {request:?}");
    rabbitmq_channel
        .basic_publish(
            "",
            response_target.reply_to.as_str(),
            BasicPublishOptions::default(),
            &Response::Success {
                job_id: request.id,
                insertion_point: contents.insertion_point,
                well_location,
                drop: contents.drop,
                crystals: contents.crystals,
            }
            .to_vec()
            .unwrap(),
            BasicProperties::default(),
        )
        .await
        .unwrap()
        .await
        .unwrap();
    response_target
        .acker
        .ack(BasicAckOptions::default())
        .await
        .unwrap();
}

/// Takes an error generated in one of the prior stages and publishes a [`Response::Failure`] to the RabbitMQ [`Channel`] provided by the [`ResponseTarget`].
pub async fn produce_error(
    request: Request,
    response_target: ResponseTarget,
    error: anyhow::Error,
    rabbitmq_channel: Channel,
) {
    println!("Producing error for: {request:?}");
    rabbitmq_channel
        .basic_publish(
            "",
            response_target.reply_to.as_str(),
            BasicPublishOptions::default(),
            &Response::Failure {
                job_id: request.id,
                error: error.to_string(),
            }
            .to_vec()
            .unwrap(),
            BasicProperties::default(),
        )
        .await
        .unwrap()
        .await
        .unwrap();
    response_target
        .acker
        .ack(BasicAckOptions::default())
        .await
        .unwrap();
}
