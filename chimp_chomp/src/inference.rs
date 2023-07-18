use crate::image_loading::ChimpImage;
use anyhow::Context;
use chimp_protocol::Job;
use itertools::{izip, Itertools};
use ndarray::{Array1, Array2, Array3, Axis, CowArray, Ix1, Ix2, Ix4};
use ort::{Environment, ExecutionProvider, GraphOptimizationLevel, Session, SessionBuilder, Value};
use std::{env::current_exe, ops::Deref};
use tokio::sync::mpsc::{error::TryRecvError, Receiver, UnboundedSender};

/// The raw box predictor output of a MaskRCNN.
pub type BBoxes = Array2<f32>;
/// The raw label output of a MaskRCNN.
pub type Labels = Array1<i64>;
/// The raw scores output of a MaskRCNN.
pub type Scores = Array1<f32>;
/// The raw masks output of a MaskRCNN.
pub type Masks = Array3<f32>;

/// Starts an inference session by setting up the ONNX Runtime environment and loading the model.
///
/// Returns an [`anyhow::Error`] if the environment could not be built or if the model could not be loaded.
pub fn setup_inference_session() -> Result<Session, anyhow::Error> {
    let environment = Environment::builder()
        .with_name("CHiMP")
        .with_execution_providers([ExecutionProvider::CPU(Default::default())])
        .build()?
        .into_arc();
    Ok(SessionBuilder::new(&environment)?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_model_from_file(
            current_exe()?
                .parent()
                .context("Executable has no parent directory")?
                .join("chimp.onnx"),
        )?)
}

/// Performs inference on a batch of images, dummy images are used to pad the tesnor if underfull.
///
/// Returns a set of predictions, where each instances corresponds to the an input image, order is maintained.
fn do_inference(
    session: &Session,
    images: &[ChimpImage],
    batch_size: usize,
) -> Vec<(BBoxes, Labels, Scores, Masks)> {
    let batch_images = images
        .iter()
        .map(|image| image.deref().view())
        .cycle()
        .take(batch_size)
        .collect::<Vec<_>>();
    let input_array = CowArray::from(ndarray::stack(Axis(0), &batch_images).unwrap().into_dyn());
    let input = Value::from_array(session.allocator(), &input_array).unwrap();
    let outputs = session.run(vec![input]).unwrap();
    outputs
        .into_iter()
        .take(images.len() * 4)
        .tuples()
        .map(|(bboxes, labels, scores, masks)| {
            let bboxes = bboxes
                .try_extract::<f32>()
                .unwrap()
                .view()
                .to_owned()
                .into_dimensionality::<Ix2>()
                .unwrap();
            let labels = labels
                .try_extract::<i64>()
                .unwrap()
                .view()
                .to_owned()
                .into_dimensionality::<Ix1>()
                .unwrap();
            let scores = scores
                .try_extract::<f32>()
                .unwrap()
                .view()
                .to_owned()
                .into_dimensionality::<Ix1>()
                .unwrap();
            let masks = masks
                .try_extract::<f32>()
                .unwrap()
                .view()
                .to_owned()
                .into_dimensionality::<Ix4>()
                .unwrap()
                .remove_axis(Axis(1));

            (bboxes, labels, scores, masks)
        })
        .collect()
}

/// Listens to a [`Receiver`] for instances of [`ChimpImage`] and performs batch inference on these.
///
/// Each pass, all available images in the [`tokio::sync::mpsc::channel`] - up to the batch size - are taken and passed to the model for inference.
/// Model predictions are sent over a [`tokio::sync::mpsc::unbounded_channel`].
pub async fn inference_worker(
    session: Session,
    batch_size: usize,
    mut image_rx: Receiver<(ChimpImage, Job)>,
    prediction_tx: UnboundedSender<(BBoxes, Labels, Scores, Masks, Job)>,
) {
    let mut images = Vec::new();
    let mut jobs = Vec::<Job>::new();
    loop {
        let (image, job) = image_rx.recv().await.unwrap();
        images.push(image);
        jobs.push(job);
        while images.len() < batch_size {
            match image_rx.try_recv() {
                Ok((image, job)) => {
                    images.push(image);
                    jobs.push(job);
                    Ok(())
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => Err(TryRecvError::Disconnected),
            }
            .unwrap();
        }
        println!("CHiMP Inference ({}): {:?}", images.len(), jobs);
        let predictions = do_inference(&session, &images, batch_size);
        izip!(predictions.into_iter(), jobs.iter()).for_each(
            |((bboxes, labels, scores, masks), job)| {
                prediction_tx
                    .send((bboxes, labels, scores, masks, job.clone()))
                    .unwrap();
            },
        );
        images.clear();
        jobs.clear();
    }
}
