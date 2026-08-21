use std::{path::PathBuf, sync::Arc, time::Duration};

use indicatif::ProgressBar;
use lang_parse::download::{Language, download_languages};
use clap::{Parser, Subcommand};

use common::{AppState};

use crate::{commands::{count::count_lines, download::{download_command_interactive, download_languages_interactive}}, console::Console};

mod commands;
mod console;

#[derive(Parser)]
#[command(version, about="grr is a code analysis and rag tool", long_about = "")]
#[command(name = "grr")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}


#[derive(Subcommand, Clone)]
enum DownloadSubcommand {
    /// Download required models
    Models,
    /// Download tooling for langauges
    /// Run without langauages flag to select languages manually
    Languages {
        #[arg(long, short, value_delimiter = ',', help = " Languages to install. \n example: \n grr download languages -l rust,typescript OR \n grr download languages -l rust -l typescript\n")]
        languages: Vec<Language>
    }
}

#[derive(Parser, Clone)]
struct DownloadCommand {
    #[command(subcommand)]
    command: DownloadSubcommand
}

#[derive(Subcommand)]
enum Commands {
    /// Count lines of code in a project
    Count {
        /// Path to project 
        path: PathBuf,
        /// Comma Separated values for directories or files to ignore
        #[arg(long, short, value_delimiter = ',')]
        exclude: Vec<String>,
    },
    /// Download Command for required tools/models
    Download(DownloadCommand),
    /// Analyze a codebase
    Analyze {
        /// Path to project
        path: PathBuf
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new().await);
    let cli = Cli::parse();
    match cli.commands {
        Commands::Count { path, exclude } => {
            count_lines(path, exclude);
        },
        Commands::Download(what) => {
            match what.command {
                DownloadSubcommand::Models => {
                    if let Err(err) = download_command_interactive(Arc::clone(&state)).await {
                        Console::error(err);
                    };
                }
                DownloadSubcommand::Languages { languages } => {
                    if languages.is_empty() {
                        if let Err(err) = download_languages_interactive().await {
                            Console::error(err);
                        }
                    }
                    else {
                        let pb = ProgressBar::new_spinner();
                        pb.enable_steady_tick(Duration::from_millis(500));
                        for language in &languages {
                            pb.set_message(format!("Downloading: {}", language));
                        }
                        if let Err(err) = download_languages(languages).await {
                            Console::error(err);
                        }
                        pb.finish_and_clear();
                    }
                }
            }
        }
        Commands::Analyze { path } => {
            todo!()
        }
    }
}


// #[cfg(test)]
// mod test {
//     #[test]
//     fn get_json() {
//         use serde_json::{Value};
//         use std::{fs};

//         fn limit_arrays(value: &mut Value, limit: usize) {
//             match value {
//                 Value::Array(array) => {
//                     // Keep only the first `limit` elements
//                     array.truncate(limit);

//                     // Recursively process nested structures
//                     for item in array {
//                         limit_arrays(item, limit);
//                     }
//                 }

//                 Value::Object(object) => {
//                     // Recursively process every value in the object
//                     for value in object.values_mut() {
//                         limit_arrays(value, limit);
//                     }
//                 }

//                 _ => {}
//             }
//         }

//         fn test() -> Result<(), Box<dyn std::error::Error>> {


//             let input = "rustanalyzer.json";
//             let output = "structure.json";

//             // Read input JSON
//             let contents = fs::read_to_string(input)?;

//             // Parse JSON
//             let mut json: Value = serde_json::from_str(&contents)?;

//             // Keep top 5 elements from every array
//             limit_arrays(&mut json, 5);

//             // Write formatted JSON
//             let output_json = serde_json::to_string_pretty(&json)?;

//             fs::write(output, output_json)?;

//             println!("Written to {output}");

//             Ok(())
//         }

//         test();
//         assert_eq!(0,0)
//     }
// }
