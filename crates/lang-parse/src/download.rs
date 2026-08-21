use std::{fmt::Display, pin::Pin, process::{Command, Stdio}};

use futures::future::join_all;
use serde::{Deserialize, Serialize};
use clap::{ValueEnum};
use strum::{EnumIter};
use tokio::process::Command as TokioCommand;

#[derive(Serialize, Deserialize, ValueEnum, Clone, EnumIter)]
pub enum Language {
    #[allow(missing_docs)]  #[serde(alias = "Rust")] #[value(name = "rust")]  Rust,
    #[allow(missing_docs)]  #[serde(alias = "TypeScript")] #[value(name = "typescript")]  TypeScript,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rust => f.write_str("Rust"),
            Language::TypeScript => f.write_str("Javascript/Typescript")
        }
    }
}

type DownloadVec = Vec<Pin<Box<dyn Future<Output = Result<(), String>> + Send>>>;

pub async fn download_languages(languages: Vec<Language>) -> Result<(), String> {
    let mut err_string = String::new();
    let mut download_vec: DownloadVec = Vec::new();

    languages
        .into_iter()
        .for_each(|lang| {
            match lang {
                Language::Rust => {
                    download_vec.push(Box::pin(download_rust()));
                }
                Language::TypeScript => {
                    download_vec.push(Box::pin(download_js_ts_scip()));
                }
            }
        });

    let results = join_all(download_vec).await;

    results
        .into_iter()
        .for_each(|result| {
            if let Err(err) = result {
                err_string.push_str(&format!("\n{}",err ));
            }
        });
    
    if !err_string.is_empty() {
        Err(err_string)
    }else{
        Ok(())
    }
}

fn check_ra () -> bool {
    Command::new("rust-analyzer")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|output| {
            output.success()
        })
        .unwrap_or(false)
}

async fn download_rust() -> Result<(), String> {
    if check_ra() {
        println!("Rust-analyzer already exists");
        Ok(())
    }
    else {
        let ra_install = TokioCommand::new("rustup")
            .args(["component", "add", "rust-analyzer"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await;
        match ra_install {
            Err(err) => {
                Err(err.to_string())
            }
            Ok(status) => {
                if !status.success() {
                    Err(String::from("Error adding rust-analyzer. Please install it manually"))
                }else {
                    println!("success downloading rust-analyzer");
                    Ok(())
                }
            }
        }
    }
}


fn js_ts_scip_present() -> bool {
    Command::new("scip-typescript")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .map(|status| {
            status.success()
        }).unwrap_or(false)
}

async fn download_js_ts_scip() -> Result<(), String> {
    if js_ts_scip_present() {
        println!("Scip for Javascript/Typescript exists");
        Ok(())
    }else {
        let js_ts_install = TokioCommand::new("npm")
            .args(["install", "-g", "@sourcegraph/scip-typescript"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .status()
            .await;
        match js_ts_install {
            Err(err) => {
                Err(err.to_string())
            }
            Ok(status) => {
                if !status.success() {
                    Err(String::from("Error adding scip for js/ts. Please install it manually"))
                }else {
                    println!("success downloading scip for js/ts");
                    Ok(())
                }
            }
        }
    }
}