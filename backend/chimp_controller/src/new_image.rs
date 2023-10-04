use crate::queries::image_created::ImageCreatedSubscription;
use async_tungstenite::tungstenite::{client::IntoClientRequest, http::HeaderValue, Message};
use cynic::{StreamingOperation, SubscriptionBuilder};
use futures_util::{
    task::{FutureObj, Spawn, SpawnError},
    StreamExt,
};
use graphql_ws_client::{
    graphql::Cynic, AsyncWebsocketClient, CynicClientBuilder, SubscriptionStream,
};
use tokio::runtime::Handle;
use url::Url;

/// A new type wrapping the current [`Handle`], used to spawn new .
#[derive(Debug)]
pub struct Spawner(Handle);

impl Spawner {
    /// Creates a new [`Spawner`] using the current [`Handle`].
    pub fn new() -> Self {
        Self(Handle::current())
    }
}

impl Spawn for Spawner {
    fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        self.0.spawn(future);
        Ok(())
    }
}

/// Creates a [`AsyncWebsocketClient`], which can subscribe to the targeting service.
pub async fn setup_targeting_subscription_client(
    targeting_url: &Url,
    authorization_token: &str,
) -> Result<AsyncWebsocketClient<Cynic, Message>, anyhow::Error> {
    let mut request = targeting_url.into_client_request()?;
    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_static("graphql-transport-ws"),
    );
    request.headers_mut().insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {authorization_token}"))?,
    );

    let (connection, _) = async_tungstenite::tokio::connect_async(request).await?;
    let (sink, stream) = connection.split();

    Ok(CynicClientBuilder::new()
        .build(stream, sink, Spawner::new())
        .await?)
}

/// Creates a [`SubscriptionStream`] on the image creation endpoint of the targeting service.
pub async fn setup_image_creation_stream(
    targeting_subscription_client: &mut AsyncWebsocketClient<Cynic, Message>,
) -> Result<SubscriptionStream<Cynic, StreamingOperation<ImageCreatedSubscription>>, anyhow::Error>
{
    let subscription = ImageCreatedSubscription::build(());
    Ok(targeting_subscription_client
        .streaming_operation(subscription)
        .await?)
}
