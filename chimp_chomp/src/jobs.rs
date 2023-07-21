use crate::{
    image_loading::{load_image, ChimpImage, WellImage},
    postprocessing::Contents,
};
use anyhow::anyhow;
use chimp_protocol::{Circle, Job, Response};
use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, Consumer,
};
use tokio::sync::mpsc::{OwnedPermit, UnboundedSender};
use url::Url;
use uuid::Uuid;

/// Creates a RabbitMQ [`Connection`] with [`Default`] [`lapin::ConnectionProperties`].
///
/// Returns a [`anyhow::Error`] if the URL could not be built or a connection could not be established.
pub async fn setup_rabbitmq_client(
    mut address: Url,
    username: Option<&str>,
    password: Option<&str>,
) -> Result<Connection, anyhow::Error> {
    address
        .set_username(username.unwrap_or_default())
        .map_err(|_| anyhow!("Could not set username"))?;
    address
        .set_password(password)
        .map_err(|_| anyhow!("Could not set password"))?;
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
        .basic_consume(
            channel.as_ref(),
            &worker_tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
}

/// Reads a message from the [`lapin::Consumer`] then loads and prepares the requested image for downstream processing.
///
/// An [`OwnedPermit`] to send to the chimp [`tokio::sync::mpsc::channel`] is required such that backpressure is be propagated to message consumption.
///
/// The prepared images are sent over a [`tokio::sync::mpsc::channel`] and [`tokio::sync::mpsc::unbounded_channel`] if sucessful.
/// An [`anyhow::Error`] is sent if the image could not be read or is empty.
pub async fn consume_job(
    mut consumer: Consumer,
    input_width: u32,
    input_height: u32,
    chimp_permit: OwnedPermit<(ChimpImage, Job)>,
    well_image_tx: UnboundedSender<(WellImage, Job)>,
    error_tx: UnboundedSender<(anyhow::Error, Job)>,
) {
    let delivery = consumer.next().await.unwrap().unwrap();
    delivery.ack(BasicAckOptions::default()).await.unwrap();

    let job = Job::from_slice(&delivery.data).unwrap();
    println!("Consumed Job: {job:?}");

    match load_image(job.file.clone(), input_width, input_height) {
        Ok((chimp_image, well_image)) => {
            chimp_permit.send((chimp_image, job.clone()));
            well_image_tx
                .send((well_image, job))
                .map_err(|_| anyhow::Error::msg("Could not send well image"))
                .unwrap()
        }
        Err(err) => error_tx.send((err, job)).unwrap(),
    };
}

/// Takes the results of postprocessing and well centering and publishes a [`Response::Success`] to the RabbitMQ [`Channel`] provided by the [`Job`].
pub async fn produce_response(
    contents: Contents,
    well_location: Circle,
    job: Job,
    rabbitmq_channel: Channel,
) {
    println!("Producing response for: {job:?}");
    rabbitmq_channel
        .basic_publish(
            "",
            &job.predictions_channel,
            BasicPublishOptions::default(),
            &Response::Success {
                job_id: job.id,
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
}

/// Takes an error generated in one of the prior stages and publishes a [`Response::Failure`] to the RabbitMQ [`Channel`] provided by the [`Job`].
pub async fn produce_error(error: anyhow::Error, job: Job, rabbitmq_channel: Channel) {
    println!("Producing error for: {job:?}");
    rabbitmq_channel
        .basic_publish(
            "",
            &job.predictions_channel,
            BasicPublishOptions::default(),
            &Response::Failure {
                job_id: job.id,
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
}
