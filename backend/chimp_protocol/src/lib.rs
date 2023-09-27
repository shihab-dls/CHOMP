#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc=include_str!("../README.md")]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A CHiMP processing request definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    /// A unique identifier for the request, to be returned in the [`Response`].
    pub id: Uuid,
    /// The key of an object containing the image to perform inference on.
    pub key: String,
}

impl Request {
    /// Deserialize an instance [`Request`] from bytes of JSON text.
    pub fn from_slice(v: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(v)
    }

    /// Serialize the [`Request`] as a JSON byte vector
    pub fn to_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }
}

/// A set of predictions which apply to a single image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    /// The image was processed successfully, producing the contained predictions.
    Success {
        /// The unique identifier of the [`Request`].
        job_id: Uuid,
        /// The proposed point for solvent insertion.
        insertion_point: Point,
        /// The location of the well centroid and radius.
        well_location: Circle,
        /// A bounding box emcompasing the solvent.
        drop: BBox,
        /// A set of bounding boxes, each emcompasing a crystal.
        crystals: Vec<BBox>,
    },
    /// Image processing failed, with the contained error.
    Failure {
        /// The unique identifier of the [`Request`].
        job_id: Uuid,
        /// A description of the error encountered.
        error: String,
    },
}

impl Response {
    /// Deserialize an instance [`Response`] from bytes of JSON text.
    pub fn from_slice(v: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(v)
    }

    /// Serialize the [`Response`] as a JSON byte vector
    pub fn to_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }
}

/// A point in 2D space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    /// The position of the point in the X axis.
    pub x: i32,
    /// The position of the point in the Y axis.
    pub y: i32,
}

/// A circle, defined by the center point and radius.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    /// The position of the circles center.
    pub center: Point,
    /// The radius of the circle.
    pub radius: i32,
}

/// A bounding box which encompasing a region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BBox {
    /// The position of the upper bound in the Y axis.
    pub top: i32,
    /// The position of the lower bound in the Y axis.
    pub bottom: i32,
    /// The position of the upper bound in the X axis.
    pub right: i32,
    /// The position of the lower bound in the X axis.
    pub left: i32,
}
