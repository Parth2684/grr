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
