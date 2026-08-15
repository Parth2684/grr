use std::sync::Arc;

use clap::{Parser, Subcommand};

use common::{AppState};

use crate::{commands::{count::count_lines, download::download_command_interactive}, console::Console};

mod commands;
mod console;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(name = "grr")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}


#[derive(Subcommand, Clone)]
enum DownloadSubcommand {
    Model,
}

#[derive(Parser, Clone)]
struct DownloadCommand {
    #[command(subcommand)]
    command: DownloadSubcommand
}

#[derive(Subcommand)]
enum Commands {
    Count {
        path: String,
        #[arg(long, short, value_delimiter = ',')]
        exclude: Vec<String>,
    },
    Download(DownloadCommand)
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new().await);
    let cli = Cli::parse();
    match cli.commands {
        Commands::Count { path, exclude } => {
            count_lines(path, exclude);
        },
        Commands::Download(_) => {
            if let Err(err) = download_command_interactive(Arc::clone(&state)).await {
                Console::error(err);
            };
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