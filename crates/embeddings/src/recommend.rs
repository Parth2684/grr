use ort::ep::{CUDA, CoreML, ExecutionProvider, TensorRT};

use crate::Precision;




pub fn get_recommendation() -> Precision {
    if TensorRT::default().is_available().unwrap_or(false) || CoreML::default().is_available().unwrap_or(false) {
        Precision::Fp16
    }else {
        Precision::Fp32
    }
}

pub fn download_models(models: Vec<Precision>) {
    
}