use common::{AppState, hash::hash};
use embeddings::{Info, Precision, recommend::get_recommendation};
use futures::future::join_all;
use inquire::MultiSelect;
use reqwest::{Client, Url, header::RANGE};
use std::{collections::VecDeque, fs, path::PathBuf, sync::Arc};

use crate::console::Console;


pub async fn download_command_interactive(state: Arc<AppState>) -> Result<(), String> {
    let recommendation = get_recommendation();
    let default: usize = match recommendation {
        Precision::Fp32 => 0,
        Precision::Fp16 => 1,
        Precision::Int8 => 2,
    };
    let options = [
        (
            "FP32",
            Precision::Fp32,
            "Highest accuracy • Size: 1.98 GB • Highest memory usage • Slightly Slower on GPU than fp16",
        ),
        (
            "FP16",
            Precision::Fp16,
            "Near-FP32 accuracy • Size: 988 MB • Lower memory usage • Slightly Slower on CPU than fp32",
        ),
        (
            "INT8",
            Precision::Int8,
            "Smallest model • Size: 495 MB • May reduce accuracy • Fastest on CPU but least accurate | Only use if hardware is really bad",
        ),
    ]
    .into_iter()
    .map(|(name, precision, description)| {
        if precision == recommendation {
            format!("{name} (recommended) — {description}")
        } else {
            format!("{name} — {description}")
        }
    })
    .collect::<Vec<String>>();

    let to_download = MultiSelect::new("Select Model Precision for Vector embeddings", options)
        .with_default(&[default])
        .with_help_message("Use Space to select, Enter to confirm")
        .prompt()
        .unwrap();

    let precisions: Vec<Precision> = to_download
        .iter()
        .map(|x| {
            if x.starts_with("FP32") {
                Console::info("Downloading FP32");
                Precision::Fp32
            } else if x.starts_with("FP16") {
                Console::info("Downloading FP16");
                Precision::Fp16
            } else {
                Console::info("Downloading INT8");
                Precision::Int8
            }
        })
        .collect();

    if precisions.is_empty() {
        return Err(String::from("Error No Model Selected"));
    }

    let model_directory = &state.local_data_dir.join("models");

    let mut futures: VecDeque<_> = precisions
        .iter()
        .flat_map(|precision| {
            let infos = precision.get_model_info();
            if !model_directory.exists() {
                fs::create_dir_all(model_directory).expect("Error creating model directory");
            }
            
            infos
                .iter()
                .filter_map(|info| {
                    let model = model_directory.join(&info.name);
                    if model.exists() {
                        let local_hash = hash(&model);
                        if local_hash != info.sha256 {
                            let continue_from = {
                                match fs::metadata(&model) {
                                    Ok(data) => data.len() as u32,
                                    Err(err) => {
                                        Console::warn(format!("Error getting file size: {}", err));
                                        fs::remove_file(&model).ok();
                                        0
                                    }
                                }
                            };
                            Some(download_model(info.link.clone(), continue_from, model, info.name))
                        } else {
                            Console::info(format!("{} exists", info.name));
                            None
                        }
                    } else {
                        Some(download_model(info.link.clone(), 0, model, info.name))
                    }
                })
                .collect::<VecDeque<_>>()
        })
        .collect();

    let tokenizer = Info::get_tokenizer_info();
    let tokenizer_path = model_directory.join(tokenizer.name);

    let tokenizer_hash_verify = hash(&tokenizer_path) == tokenizer.sha256;
    
    if !tokenizer_path.exists() | !tokenizer_hash_verify {
        fs::remove_file(&tokenizer_path).ok();
        futures.push_front(download_model(tokenizer.link, 0, tokenizer_path))
    }

    
    let handles = join_all(futures).await;

    let mut errors = Vec::new();
    
    for handle in handles {
        match handle {
            Ok(name) => {
                Console::success(format!("{:?} successfully downloaded", name))
            }
            Err(err) => {
                errors.push(err)
            }
        }
    } 

    if !errors.is_empty() {
        return Err(errors.join("\n"))
    }
    
    Ok(())
}

async fn download_model(link: Url, existing_size: u32, download_path: PathBuf, name: String) -> Result<String, String> {
    let client = Client::new();
    let mut request = client.get(link);

    if existing_size > 0 {
        request = request.header(RANGE, format!("bytes={existing_size}-"));
    }

    let response = request.send().await
        .map_err(|err| {
            Console::error(err);
            format!("Error downloading {:?}", name)
        })?;

    
    Ok("".into())
}
