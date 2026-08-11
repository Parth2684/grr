use common::{AppState, hash::hash};
use embeddings::{Info, Precision, recommend::get_recommendation};
use futures::{StreamExt, future::join_all};
use indicatif::{
    MultiProgress, MultiProgressAlignment, ProgressBar, ProgressDrawTarget, ProgressStyle,
};
use inquire::MultiSelect;
use reqwest::{Client, StatusCode, Url, header::RANGE};
use std::{
    collections::VecDeque,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    pin::Pin,
    sync::Arc,
};

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

    let multi_progress = Arc::new(MultiProgress::new());
    multi_progress.set_alignment(MultiProgressAlignment::Top);
    multi_progress.set_draw_target(ProgressDrawTarget::stderr());

    let mut count = 0;

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
                            count += 1;
                            Some(download_model(
                                info.link.clone(),
                                continue_from as u64,
                                model,
                                info.name.clone(),
                                Arc::clone(&multi_progress),
                                count,
                            ))
                        } else {
                            Console::info(format!("{} exists", info.name));
                            None
                        }
                    } else {
                        count += 1;
                        Some(download_model(
                            info.link.clone(),
                            0,
                            model,
                            info.name.clone(),
                            Arc::clone(&multi_progress),
                            count,
                        ))
                    }
                })
                .collect::<VecDeque<_>>()
        })
        .collect();

    let tokenizer = Info::get_tokenizer_info();
    let tokenizer_path = model_directory.join(tokenizer.name);

    if !tokenizer_path.exists() || hash(&tokenizer_path) != tokenizer.sha256 {
        fs::remove_file(&tokenizer_path).ok();
        count += 1;
        futures.push_front(download_model(
            tokenizer.link,
            0,
            tokenizer_path,
            String::from("tokenizer"),
            Arc::clone(&multi_progress),
            count,
        ))
    } else {
        Console::info("tokenizer exists");
    }

    let handles = join_all(futures).await;

    let mut errors = Vec::new();

    for handle in handles {
        match handle {
            Ok(name) => Console::success(format!("{:?} successfully downloaded", name)),
            Err(err) => errors.push(err),
        }
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    Ok(())
}

fn download_model(
    link: Url,
    existing_size: u64,
    download_path: PathBuf,
    name: String,
    progress: Arc<MultiProgress>,
    count: usize,
) -> Pin<Box<dyn Future<Output = Result<String, String>>>> {
    Box::pin(async move {
        let client = Client::new();
        let mut request = client.get(link.clone());

        let token = std::env::var("HF_TOKEN").ok();
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        if existing_size > 0 {
            request = request.header(RANGE, format!("bytes={existing_size}-"));
        }

        let response = request.send().await.map_err(|err| {
            Console::error(err);
            format!("Error downloading {:?}", name)
        })?;

        let status = response.status();

        if existing_size > 0 && status == StatusCode::PARTIAL_CONTENT {
            Console::info("Resuming Download");
        } else if existing_size > 0 && status == StatusCode::OK {
            Console::info("Resuming download not working downloading again");
            fs::remove_file(&download_path).ok();
            return download_model(link, 0, download_path, name, Arc::clone(&progress), count)
                .await;
        }

        let total_size = response.content_length().map(|s| s + existing_size);

        let pb = match total_size {
            Some(size) => {
                let pb = progress.insert(0, ProgressBar::new(size));
                pb.set_position(existing_size);
                pb.set_style(
                    ProgressStyle::with_template(
                        "
                        {msg:<35} {spinner:.green} [{elapsed_precise}] \
                        [{bar:40.cyan/blue}] \
                        {bytes}/{total_bytes} \
                        ({bytes_per_sec}, {eta})
                    ",
                    )
                    .map_err(|err| format!("Error showing template: {:?}", err))?,
                );
                pb
            }
            None => {
                let pb = progress.insert(0, ProgressBar::new_spinner());
                pb.set_style(
                    ProgressStyle::with_template(
                        "
                            {msg:25} {spinner:.green} [{elapsed_precise}] \
                            {bytes} {bytes_per_second}
                        ",
                    )
                    .map_err(|err| format!("Error showing template: {:?}", err))?,
                );
                pb
            }
        };

        pb.set_message(name.clone());
        pb.set_position(existing_size);

        let mut file = if existing_size > 0 {
            File::options()
                .append(true)
                .open(&download_path)
                .map_err(|err| {
                    Console::error(format!("{err:?}"));
                    format!("Error downloading: {:?}", name)
                })?
        } else {
            File::create(&download_path).map_err(|err| {
                Console::error(format!("{:?}", err.to_string()));
                format!("Error downloading: {:?}", name)
            })?
        };

        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            match chunk {
                Err(err) => return Err(err.to_string()),
                Ok(data) => match file.write_all(&data) {
                    Err(err) => return Err(err.to_string()),
                    Ok(_) => {
                        pb.inc(data.len() as u64);
                    }
                },
            }
        }

        file.flush().map_err(|err| {
            Console::error(err.to_string());
            format!("Error flushing file: {:?}", name)
        })?;

        pb.finish();
        Ok(name)
    })
}
