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

fn find_well_center(image: WellImage) -> Result<Circle, anyhow::Error> {
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

pub async fn well_centering(
    image: WellImage,
    job: Job,
    well_location_tx: UnboundedSender<(Circle, Job)>,
    error_tx: UnboundedSender<(anyhow::Error, Job)>,
) {
    println!("Finding Well Center for {job:?}");
    match find_well_center(image) {
        Ok(well_center) => well_location_tx.send((well_center, job)).unwrap(),
        Err(err) => error_tx.send((err, job)).unwrap(),
    }
}
