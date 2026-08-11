use reqwest::Url;

pub mod recommend;
mod test;

#[derive(PartialEq, Eq, Debug)]
pub enum Precision {
    Fp32,
    Fp16,
    Int8,
}

pub struct Info {
    pub name: String,
    pub size: u32,
    pub link: Url,
    pub sha256: String,
}

impl Precision {
    pub fn get_model_info(&self) -> Vec<Info> {
        match self {
            Precision::Fp32 => vec![
                Info {
                    name: String::from("embedding_fp32.onnx"),
                    size: 258729,
                    sha256: String::from("c6cdc3dc8d278a37136a6b47848f62185a4dd438a5e1fc54e1d2e0e8bc122f76"),
                    link: Url::parse("https://huggingface.co/Parth2684/jina-code-embeddings-0.5b-onnx/resolve/main/jina-code-embeddings-0.5b/quantized/embedding_fp32.onnx").unwrap()
                },
                Info {
                    name: String::from("embedding_fp32.onnx.data"),
                    size: 1976106496,
                    sha256: String::from("5a28af5053664beec046176ae019a5eaeb9a97e3998031dd0096d0bc167be083"),
                    link: Url::parse("https://huggingface.co/Parth2684/jina-code-embeddings-0.5b-onnx/resolve/main/jina-code-embeddings-0.5b/quantized/embedding_fp32.onnx.data").unwrap()
                }
            ],
            Precision::Fp16 => vec![
                Info {
                    name: String::from("embedding_fp16.onnx"),
                    size: 246335,
                    sha256: String::from("a5781ed55b0c0f1476617848b0b1527be640f746d2bbf59e93f907a444c13ead"),
                    link: Url::parse("https://huggingface.co/Parth2684/jina-code-embeddings-0.5b-onnx/resolve/main/jina-code-embeddings-0.5b/quantized/embedding_fp16.onnx").unwrap()
                },
                Info {
                    name: String::from("embedding_fp16.onnx.data"),
                    size: 988053248,
                    sha256: String::from("508291cdb3f872035ffff410a94afaab38415cfee36a8a18a0314620400ab079"),
                    link: Url::parse("https://huggingface.co/Parth2684/jina-code-embeddings-0.5b-onnx/resolve/main/jina-code-embeddings-0.5b/quantized/embedding_fp16.onnx.data").unwrap()
                }
            ],
            Precision::Int8 => vec![
                Info {
                    name: String::from("embedding_int8_dynamic.onnx"),
                    size: 494562250,
                    sha256: String::from("c52c78d0048bce5ab8e84a8d9e64cfe30c0db156662de4916a3d97a9c2505420"),
                    link: Url::parse("https://huggingface.co/Parth2684/jina-code-embeddings-0.5b-onnx/resolve/main/jina-code-embeddings-0.5b/quantized/embedding_int8_dynamic.onnx").unwrap()
                }
            ]
        }
    }
}

impl Info {
    pub fn get_tokenizer_info() -> Info {
        Info {
            name: String::from("tokenizer.json"),
            size: 11417884,
            link: Url::parse("https://huggingface.co/Parth2684/jina-code-embeddings-0.5b-onnx/resolve/main/jina-code-embeddings-0.5b/tokenizer/tokenizer.json").unwrap(),
            sha256: String::from("f0c3c6d1699d808eea431043c5d9db4e6c191698789c69a5d35f11aeabc2304a")
        }
    }
}


