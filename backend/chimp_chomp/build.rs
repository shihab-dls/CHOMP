use std::{env, fs::copy, path::PathBuf};

const MODEL_FILE: &str = "chimp.onnx";

fn main() {
    let model_src = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join(MODEL_FILE);
    let model_dst = PathBuf::from(env::var("OUT_DIR").unwrap())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(MODEL_FILE);
    copy(model_src, model_dst).unwrap();
}
