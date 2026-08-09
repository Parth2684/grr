use common::{AppState, hash::hash};
use embeddings::{Precision, recommend::get_recommendation};
use inquire::MultiSelect;
use reqwest::{Url};
use std::{fs, path::{Path}};

pub fn download_command_interactive(state: &AppState) -> Result<(), String> {
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
                println!("Downloading FP32");
                Precision::Fp32
            } else if x.starts_with("FP16") {
                println!("Downloading FP16");
                Precision::Fp16
            } else {
                println!("Downloading INT8");
                Precision::Int8
            }
        })
        .collect();

    if precisions.is_empty() {
        return Err(String::from("Error No Model Selected"));
    }

    precisions.iter().for_each(|precision| {
        let infos = precision.get_model_info();
        let model_directory = &state.local_data_dir.join("models");
        if !model_directory.exists() {
            fs::create_dir_all(model_directory).expect("Error creating model directory");
        }
        infos
            .iter()
            .for_each(|info| {
                let model = model_directory.join(&info.name);
                if model.exists() {
                    let local_hash = hash(&model);
                    if local_hash != info.sha256 {
                        download_model(info.link.clone(), info.size, model);
                    }else {
                        println!("{} exists", info.name);
                    }
                }else {
                    download_model(info.link.clone(), info.size, model);
                }
            });
    });

    Ok(())
}

fn download_model<P: AsRef<Path>>(link: Url, size: u32, download_path: P){
    todo!()
}