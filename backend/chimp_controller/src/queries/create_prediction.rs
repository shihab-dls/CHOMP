use chimp_protocol::{BBox, Point, SuccesfulResponse};
use cynic::{InputObject, QueryFragment, QueryVariables};
use uuid::Uuid;

/// The response recieved on creation of a prediction.
#[derive(Debug, Clone, QueryFragment)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct Prediction {
    /// The universally unique identity of the prediction.
    pub id: Uuid,
}

/// A point in 2D space.
#[derive(Debug, Clone, InputObject)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct PointInput {
    /// The horizontal position of the point.
    pub x: i32,
    /// The vertical postioon of the point.
    pub y: i32,
}

impl From<Point> for PointInput {
    fn from(value: Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

/// A well, idenitified by the plate it resides on and the location.
#[derive(Debug, Clone, InputObject)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct WellInput {
    /// The ID of the plate the imaged well is on
    pub plate: Uuid,
    /// The number of the imaged well
    pub well: i32,
}

/// A predicted crystal
#[derive(Debug, Clone, InputObject)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct CrystalInput {
    /// The bounding box encapsulating the crystal.
    pub bounding_box: BoundingBoxInput,
}

/// A rectangular bounding box, aligned with the vertical and horizontal axis.
#[derive(Debug, Clone, InputObject)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct BoundingBoxInput {
    /// The left most edge of the bounding box.
    pub left: i32,
    /// The right most edge of the bounding box.
    pub right: i32,
    /// The top most edge of the bounding box.
    pub top: i32,
    /// The bottom most edge of the bounding box.
    pub bottom: i32,
}

impl From<BBox> for BoundingBoxInput {
    fn from(value: BBox) -> Self {
        Self {
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
        }
    }
}

/// A predicted drop.
#[derive(Debug, Clone, InputObject)]
#[cynic(schema = "targeting", schema_module = "crate::schemas::targeting")]
pub struct DropInput {
    /// A collection of predicted crystals.
    pub crystals: Vec<CrystalInput>,
    /// The predicted optimal insertion point.
    pub insertion_point: PointInput,
    /// The predicted bounding box surrounding the drop.
    pub bounding_box: BoundingBoxInput,
}

/// The arguments to the prediction creation mutation.
#[derive(QueryVariables)]
#[cynic(schema_module = "crate::schemas::targeting")]
pub struct CreatePredictionVariables {
    /// The plate which the prediction is attributed to.
    pub plate: WellInput,
    /// The predicted centroid of the well.
    pub well_centroid: PointInput,
    /// The predicted radius of the well.
    pub well_radius: i32,
    /// A collection of predicted drops and their contents.
    pub drops: Vec<DropInput>,
}

impl From<SuccesfulResponse> for CreatePredictionVariables {
    fn from(value: SuccesfulResponse) -> Self {
        Self {
            plate: WellInput {
                plate: value.plate,
                well: value.well,
            },
            well_centroid: value.well_location.center.into(),
            well_radius: value.well_location.radius,
            drops: vec![DropInput {
                bounding_box: value.drop.into(),
                insertion_point: value.insertion_point.into(),
                crystals: value
                    .crystals
                    .into_iter()
                    .map(|crystal| CrystalInput {
                        bounding_box: crystal.into(),
                    })
                    .collect(),
            }],
        }
    }
}

/// The root mutation type of the targeting service API
#[derive(Debug, Clone, QueryFragment)]
#[cynic(
    schema = "targeting",
    schema_module = "crate::schemas::targeting",
    graphql_type = "RootMutation",
    variables = "CreatePredictionVariables"
)]
pub struct CreatePredictionMutation {
    /// A mutation to create a prediction for an image
    #[arguments(plate: $plate, wellCentroid: $well_centroid, wellRadius: $well_radius, drops: $drops)]
    pub create_prediction: Prediction,
}
