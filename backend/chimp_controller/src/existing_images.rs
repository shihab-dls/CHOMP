use crate::{
    chimp_messages::RequestPublisher,
    queries::image_predictions::{ExistingImage, ImagesQuery, Prediction},
};
use anyhow::anyhow;
use cynic::{http::ReqwestExt, QueryBuilder};
use reqwest::Method;
use url::Url;

/// Retrieves a collection of unprocessed images from the targeting service.
pub async fn get_unprocessed_images(
    targeting_client: &reqwest::Client,
    targeting_url: Url,
    authorization_token: &str,
) -> Result<Vec<ExistingImage>, anyhow::Error> {
    let query = ImagesQuery::build(());
    let response = targeting_client
        .request(Method::POST, targeting_url)
        .header("Authorization", format!("Bearer {authorization_token}"))
        .run_graphql(query)
        .await?;
    let images = response.data.ok_or(anyhow!("Empty response"))?.images;
    Ok(images
        .into_iter()
        .filter(|image| {
            !image.predictions.contains(&Prediction {
                operator_id: "CHiMP".to_string(),
            })
        })
        .collect())
}

/// Recieves an existing image and produces a [`chimp_protocol::Request`] for CHiMP to perform prediction.
pub async fn handle_existing_image(existing_image: ExistingImage, job_publisher: RequestPublisher) {
    job_publisher.publish(existing_image.into()).await.unwrap()
}
