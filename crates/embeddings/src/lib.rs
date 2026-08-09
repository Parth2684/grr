pub mod recommend;
mod test;

#[derive(PartialEq, Eq)]
pub enum Precision {
    Fp32,
    Fp16,
    Int8,
}

struct ModelInfo {
    name: String,
    size: u32,
    link: String,
    sha256: String,
}

impl Precision {
    pub fn get_model_info(&self) -> Vec<ModelInfo> {
        match self {
            &Precision::Fp32 => vec![ModelInfo {
                name: String::from("embedding_fp32.onnx"),
                size: 258729,
                sha256: 
            }],
        }
        todo!()
    }
}
