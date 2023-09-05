use crate::inference::{BBoxes, Labels, Masks};
use anyhow::Context;
use chimp_protocol::{BBox, Point, Request};
use itertools::izip;
use ndarray::{Array2, ArrayView, ArrayView2, Ix1};
use opencv::{
    core::CV_8U,
    imgproc::{distance_transform, DIST_L1, DIST_MASK_3},
    prelude::{Mat, MatTraitConst},
};
use tokio::sync::mpsc::UnboundedSender;

/// The predicted contents of a well image.
#[derive(Debug)]
pub struct Contents {
    /// The optimal point at which solvent should be inserted.
    pub insertion_point: Point,
    /// A bounding box enclosing the drop of solution.
    pub drop: BBox,
    /// A set of bounding boxes enclosing each crystal in the drop.
    pub crystals: Vec<BBox>,
}

/// The threshold to apply to the raw MaskRCNN [`Masks`] to generate a binary mask.
const PREDICTION_THRESHOLD: f32 = 0.5;

/// Creates a mask of valid insertion positions by adding all pixels in the drop mask and subsequently subtracting those in the crystal masks.
fn insertion_mask(
    drop_mask: ArrayView2<f32>,
    crystal_masks: Vec<ArrayView2<'_, f32>>,
) -> Array2<bool> {
    let mut mask = drop_mask.mapv(|prediction| prediction > PREDICTION_THRESHOLD);
    crystal_masks.into_iter().for_each(|crystal_mask| {
        mask.zip_mut_with(&crystal_mask, |valid, prediction| {
            *valid &= *prediction < PREDICTION_THRESHOLD
        })
    });
    mask
}

/// Converts an [`Array2<bool>`] into an [`Mat`] of type [`CV_8U`] with the same dimensions.
fn ndarray_mask_into_opencv_mat(mask: Array2<bool>) -> Mat {
    Mat::from_exact_iter(
        mask.mapv(|pixel| if pixel { std::u8::MAX } else { 0 })
            .into_iter(),
    )
    .unwrap()
    .reshape_nd(
        1,
        &mask
            .shape()
            .iter()
            .map(|&dim| dim as i32)
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// Performs a distance transform to find the point in the mask which is furthest from any invalid region.
///
/// Returns an [`anyhow::Error`] if no valid insertion point was found.
fn optimal_insert_position(insertion_mask: Mat) -> Result<Point, anyhow::Error> {
    let mut distances = Mat::default();
    distance_transform(&insertion_mask, &mut distances, DIST_L1, DIST_MASK_3, CV_8U).unwrap();
    let (furthest_point, _) = distances
        .iter::<u8>()
        .unwrap()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .context("No valid insertion points")?;
    Ok(Point {
        x: furthest_point.x as usize,
        y: furthest_point.y as usize,
    })
}

/// Converts an [`ArrayView<f32, Ix1>`] of length 4 into a [`BBox`] according to the layout of a MaskRCNN box prediction.
fn bbox_from_array(bbox: ArrayView<f32, Ix1>) -> BBox {
    BBox {
        left: bbox[0],
        top: bbox[1],
        right: bbox[2],
        bottom: bbox[3],
    }
}

/// Finds the first instance which is labelled as a drop.
///
/// Returns an [`anyhow::Error`] if no drop instances were found.
fn find_drop_instance<'a>(
    labels: &Labels,
    bboxes: &BBoxes,
    masks: &'a Masks,
) -> Result<(BBox, ArrayView2<'a, f32>), anyhow::Error> {
    izip!(labels, bboxes.outer_iter(), masks.outer_iter())
        .find_map(|(label, bbox, mask)| (*label == 1).then_some((bbox_from_array(bbox), mask)))
        .context("No drop instances in prediction")
}

/// Finds all instances which are labelled as crystals.
fn find_crystal_instances<'a>(
    labels: &Labels,
    bboxes: &BBoxes,
    masks: &'a Masks,
) -> Vec<(BBox, ArrayView2<'a, f32>)> {
    izip!(labels, bboxes.outer_iter(), masks.outer_iter())
        .filter_map(|(label, bbox, mask)| (*label == 2).then_some((bbox_from_array(bbox), mask)))
        .collect()
}

/// Takes the results of inference on an image and uses it to produce useful regional data and an optimal insertion point.
///
/// Returns an [`anyhow::Error`] if no drop instances could be found or if no valid insertion point was found.
fn postprocess_inference(
    bboxes: BBoxes,
    labels: Labels,
    masks: Masks,
) -> Result<Contents, anyhow::Error> {
    let (drop, drop_mask) = find_drop_instance(&labels, &bboxes, &masks)?;
    let (crystals, crystal_masks) = find_crystal_instances(&labels, &bboxes, &masks)
        .into_iter()
        .unzip();
    let insertion_mask = ndarray_mask_into_opencv_mat(insertion_mask(drop_mask, crystal_masks));
    let insertion_point = optimal_insert_position(insertion_mask)?;
    Ok(Contents {
        drop,
        crystals,
        insertion_point,
    })
}

/// Takes the results of inference on an image and uses it to produce useful regional data and an optimal insertion point.
///
/// The extracted [`Contents`] are sent over a [`tokio::sync::mpsc::unbounded_channel`] if sucessful.
/// An [`anyhow::Error`] is sent if no drop instances were found or if no valid insertion point was found.
pub async fn inference_postprocessing(
    bboxes: BBoxes,
    labels: Labels,
    masks: Masks,
    request: Request,
    contents_tx: UnboundedSender<(Contents, Request)>,
    error_tx: UnboundedSender<(anyhow::Error, Request)>,
) {
    println!("Postprocessing: {request:?}");
    match postprocess_inference(bboxes, labels, masks) {
        Ok(contents) => contents_tx.send((contents, request)).unwrap(),
        Err(err) => error_tx.send((err, request)).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::optimal_insert_position;
    use opencv::{
        core::{Point_, Scalar, CV_8UC1},
        imgproc::{circle, LINE_8},
        prelude::Mat,
    };

    #[test]
    fn optimal_insert_found() {
        let mut test_image = Mat::new_nd_with_default(
            &[1024, 1224],
            CV_8UC1,
            Scalar::new(0_f64, 0_f64, 0_f64, std::u8::MAX as f64),
        )
        .unwrap();
        circle(
            &mut test_image,
            Point_::new(256, 512),
            128,
            Scalar::new(
                std::u8::MAX as f64,
                std::u8::MAX as f64,
                std::u8::MAX as f64,
                std::u8::MAX as f64,
            ),
            -1,
            LINE_8,
            0,
        )
        .unwrap();

        let position = optimal_insert_position(test_image).unwrap();

        assert_eq!(256, position.x);
        assert_eq!(512, position.y);
    }
}
