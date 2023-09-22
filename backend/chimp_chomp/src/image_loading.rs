use anyhow::anyhow;
use aws_sdk_s3::Client;
use derive_more::Deref;
use ndarray::{Array, Ix3};
use opencv::{
    core::{Size_, Vec3f, Vector, CV_32FC3},
    imgcodecs::{imdecode, IMREAD_COLOR},
    imgproc::{cvt_color, resize, COLOR_BGR2GRAY, COLOR_BGR2RGB, INTER_LINEAR},
    prelude::{Mat, MatTraitConst},
};

/// A grayscale image of the well in [W, H, C] format.
#[derive(Debug, Deref)]
pub struct WellImage(pub Mat);

/// A RGB image of the well in [C, W, H] format.
#[derive(Debug, Deref)]
pub struct ChimpImage(Array<f32, Ix3>);

/// Converts an image from a [`Mat`] in BGR and ordered in [W, H, C] to a [`Array`] in RGB and ordered in [C, W, H] and resizes it to the input dimensions of the model.
fn prepare_chimp(image: &Mat, width: i32, height: i32) -> ChimpImage {
    let mut resized_image = Mat::default();
    resize(
        &image,
        &mut resized_image,
        Size_ { width, height },
        0.0,
        0.0,
        INTER_LINEAR,
    )
    .unwrap();

    let mut resized_rgb_image = Mat::default();
    cvt_color(&resized_image, &mut resized_rgb_image, COLOR_BGR2RGB, 0).unwrap();
    let mut resized_rgb_f32_image = Mat::default();

    resized_rgb_image
        .convert_to(
            &mut resized_rgb_f32_image,
            CV_32FC3,
            f64::from(std::u8::MAX).recip(),
            0.0,
        )
        .unwrap();
    let chimp_image = Array::from_iter(
        resized_rgb_f32_image
            .iter::<Vec3f>()
            .unwrap()
            .flat_map(|(_, pixel)| pixel),
    )
    .into_shape((
        resized_rgb_f32_image.mat_size()[0] as usize,
        resized_rgb_f32_image.mat_size()[1] as usize,
        resized_rgb_f32_image.channels() as usize,
    ))
    .unwrap()
    .permuted_axes((2, 0, 1))
    .as_standard_layout()
    .to_owned();

    ChimpImage(chimp_image)
}

/// Converts an image from BGR to grayscale.
fn prepare_well(image: &Mat) -> WellImage {
    let mut well_image = Mat::default();
    cvt_color(&image, &mut well_image, COLOR_BGR2GRAY, 0).unwrap();
    WellImage(well_image)
}

/// Reads an image from a S3 bucket.
///
/// Returns and [`anyhow::Error`] if the image could not be read or is empty.
async fn read_image_s3(client: Client, bucket: String, key: String) -> Result<Mat, anyhow::Error> {
    let object = client.get_object().bucket(bucket).key(key).send().await?;
    let bytes = Vector::from_slice(&object.body.collect().await.unwrap().to_vec());
    let image = imdecode(&bytes, IMREAD_COLOR)?;

    if image.empty() {
        return Err(anyhow!("No image data was loaded"));
    }
    Ok(image)
}

/// Reads an image from file and prepares both a [`ChimpImage`] and a [`WellImage`].
///
/// Returns an [`anyhow::Error`] if the image could not be read or is empty.
pub async fn load_image(
    client: Client,
    bucket: String,
    key: String,
    chimp_width: u32,
    chimp_height: u32,
) -> Result<(ChimpImage, WellImage), anyhow::Error> {
    let image = read_image_s3(client, bucket, key).await?;
    let well_image = prepare_well(&image);
    let chimp_image = prepare_chimp(&image, chimp_width as i32, chimp_height as i32);

    Ok((chimp_image, well_image))
}
