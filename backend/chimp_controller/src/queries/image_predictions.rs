use chimp_protocol::Request;
use cynic::QueryFragment;
use url::Url;
use uuid::Uuid;

/// The metadata of an existing prediction
#[derive(Debug, PartialEq, Eq, QueryFragment)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct Prediction {
    /// The ID of the operator who created the prediction
    pub operator_id: String,
}

/// The metadata of an existing image, including the collection of predictions
#[derive(Debug, QueryFragment)]
#[cynic(
    schema = "targeting",
    schema_module = "crate::schemas::targeting",
    graphql_type = "Image"
)]
pub struct ExistingImage {
    /// The ID of the plate the imaged well is on
    pub plate: Uuid,
    /// The number of the imaged well
    pub well: i32,
    /// A URL from which the image can be retrieved
    pub download_url: Url,
    /// A collection of predictions for the well contents
    pub predictions: Vec<Prediction>,
}

impl From<ExistingImage> for Request {
    fn from(value: ExistingImage) -> Self {
        Self {
            plate: value.plate,
            well: value.well,
            download_url: value.download_url,
        }
    }
}

/// The root query type of the targeting service API
#[derive(Debug, QueryFragment)]
#[cynic(
    schema = "targeting",
    schema_module = "crate::schemas::targeting",
    graphql_type = "RootQuery"
)]
pub struct ImagesQuery {
    /// A collection of existing images
    pub images: Vec<ExistingImage>,
}
