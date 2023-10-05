use chimp_protocol::Request;
use cynic::QueryFragment;
use url::Url;
use uuid::Uuid;

/// The metadata of a created image
#[derive(Debug, QueryFragment)]
#[cynic(
    schema = "targeting",
    schema_module = "crate::schemas::targeting",
    graphql_type = "Image"
)]
pub struct CreatedImage {
    /// The ID of the plate the imaged well is on
    pub plate: Uuid,
    /// The number of the imaged well
    pub well: i32,
    /// A URL from which the image can be retrieved
    pub download_url: Url,
}

impl From<CreatedImage> for Request {
    fn from(value: CreatedImage) -> Self {
        Self {
            plate: value.plate,
            well: value.well,
            download_url: value.download_url,
        }
    }
}

/// The root subscription type of the targeting service API
#[derive(Debug, QueryFragment)]
#[cynic(
    schema = "targeting",
    schema_module = "crate::schemas::targeting",
    graphql_type = "RootSubscription"
)]
pub struct ImageCreatedSubscription {
    /// A subscription to image creation events
    pub image_created: CreatedImage,
}
