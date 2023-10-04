use chimp_protocol::Request;
use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    protocol::basic::AMQPProperties,
    types::FieldTable,
    Channel, Connection, ConnectionProperties,
};
use url::Url;
use uuid::Uuid;

/// Creates a [`RequestPublisher`] for communication with CHiMP
pub async fn setup_chimp_client(
    rabbitmq_url: Url,
    job_channel: String,
) -> Result<RequestPublisher, anyhow::Error> {
    let connection =
        Connection::connect(rabbitmq_url.as_str(), ConnectionProperties::default()).await?;

    let channel = connection.create_channel().await?;

    let reply_queue_id = Uuid::new_v4();
    channel
        .queue_declare(
            &reply_queue_id.to_string(),
            QueueDeclareOptions {
                exclusive: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    Ok(RequestPublisher {
        channel,
        job_channel,
        reply_queue_id,
    })
}

/// A [`Channel`] wrapper for publishing CHiMP [`Request`]s.
#[derive(Debug, Clone)]
pub struct RequestPublisher {
    /// The AMQP channel.
    channel: Channel,
    /// The channel ID of the CHiMP request queue.
    job_channel: String,
    /// The queue to be used for directly replying to this service.
    reply_queue_id: Uuid,
}

impl RequestPublisher {
    /// Sends a CHiMP [`Request`] to the configured channel, with direct reply-to configuration.
    pub async fn publish(&self, request: Request) -> Result<(), anyhow::Error> {
        self.channel
            .basic_publish(
                "",
                &self.job_channel,
                BasicPublishOptions::default(),
                &request.to_vec()?,
                AMQPProperties::default().with_reply_to(self.reply_queue_id.to_string().into()),
            )
            .await?
            .await?;

        Ok(())
    }
}
