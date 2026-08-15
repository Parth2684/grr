use serde::{Deserialize, Serialize};
use clap::{ValueEnum};

#[derive(Serialize, Deserialize, ValueEnum , Clone)]
pub enum Language {
    #[allow(missing_docs)]  #[serde(alias = "Rust")] #[value(alias = "Rust")]  Rust,
    #[allow(missing_docs)]  #[serde(alias = "TypeScript")] #[value(alias = "TypeScript")]  TypeScript,
    #[allow(missing_docs)]  #[serde(alias = "JSON")] #[value(alias = "Json")]  Json,
}

pub fn download_languages(languages: Vec<Language>) -> Result<(), String> {
    languages
        .into_iter()
        .for_each(|lang| {
            match lang {
                Language::Rust => {
                    todo!()
                }
                _ => {
                    todo!()
                }
            }
        });
    Ok(())
}


fn download_rust() {
    
}