use crate::image_loading::WellImage;
use anyhow::Context;
use chimp_protocol::{Circle, Job, Point};
use opencv::{
    core::{Vec4f, Vector},
    imgproc::{hough_circles, HOUGH_GRADIENT},
    prelude::MatTraitConst,
};
use std::ops::Deref;
use tokio::sync::mpsc::UnboundedSender;

/// Uses a canny edge detector and a hough circle transform to localise a [`Circle`] of high contrast in the image.
///
/// The circle is assumed to have a radius in [⅜ l, ½ l), where `l` denotes the shortest edge lenth of the image.
/// The circle with the most counts is selected.
///
/// Returns an [`anyhow::Error`] if no circles were found.
fn find_well_location(image: WellImage) -> Result<Circle, anyhow::Error> {
    let min_side = *image.deref().mat_size().iter().min().unwrap();
    let mut circles = Vector::<Vec4f>::new();
    hough_circles(
        &*image,
        &mut circles,
        HOUGH_GRADIENT,
        4.0,
        1.0,
        100.0,
        100.0,
        min_side * 3 / 8,
        min_side / 2,
    )
    .unwrap();
    let well_location = circles
        .into_iter()
        .max_by(|&a, &b| a[3].total_cmp(&b[3]))
        .context("No circles found in image")?;
    Ok(Circle {
        center: Point {
            x: well_location[0] as usize,
            y: well_location[1] as usize,
        },
        radius: well_location[2],
    })
}

/// Takes a grayscale image of the well and finds the center and radius.
///
/// The extracted [`Circle`] is sent over a [`tokio::sync::mpsc::unbounded_channel`] if sucessful.
/// An [`anyhow::Error`] is sent if no circles were found.
pub async fn well_centering(
    image: WellImage,
    job: Job,
    well_location_tx: UnboundedSender<(Circle, Job)>,
    error_tx: UnboundedSender<(anyhow::Error, Job)>,
) {
    println!("Finding Well Center for {job:?}");
    match find_well_location(image) {
        Ok(well_center) => well_location_tx.send((well_center, job)).unwrap(),
        Err(err) => error_tx.send((err, job)).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{image_loading::WellImage, well_centering::find_well_location};
    use approx::assert_relative_eq;
    use opencv::{
        core::{Mat, Point_, Scalar, CV_8UC1},
        imgproc::{circle, LINE_8},
    };

    #[test]
    fn well_center_found() {
        const CENTER_X: usize = 654;
        const CENTER_Y: usize = 321;
        const RADIUS: f32 = 480.0;
        const THICKNESS: i32 = 196;

        let mut test_image = Mat::new_nd_with_default(
            &[1024, 1224],
            CV_8UC1,
            Scalar::new(
                std::u8::MAX as f64,
                std::u8::MAX as f64,
                std::u8::MAX as f64,
                std::u8::MAX as f64,
            ),
        )
        .unwrap();
        circle(
            &mut test_image,
            Point_ {
                x: CENTER_X as i32,
                y: CENTER_Y as i32,
            },
            RADIUS as i32 + THICKNESS / 2,
            Scalar::new(0_f64, 0_f64, 0_f64, std::u8::MAX as f64),
            THICKNESS,
            LINE_8,
            0,
        )
        .unwrap();

        let location = find_well_location(WellImage(test_image)).unwrap();

        assert_relative_eq!(
            CENTER_X as f64,
            location.center.x as f64,
            max_relative = 8.0
        );
        assert_relative_eq!(
            CENTER_Y as f64,
            location.center.y as f64,
            max_relative = 8.0
        );
        assert_relative_eq!(RADIUS, location.radius, max_relative = 8.0)
    }
}
