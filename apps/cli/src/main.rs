use std::path::PathBuf;

use clap::{Parser, Subcommand};
use cli_table::{
    Cell, CellStruct, Style, Table,
    format::{Border, Justify},
    print_stdout,
};
use common::{AppState, loc::count_loc};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Count {
        path: String,
        #[arg(long, short, value_delimiter = ',')]
        exclude: Vec<String>,
    },
}

#[tokio::main]
async fn main() {
    let app = AppState::new().await;
    let cli = Cli::parse();
    match cli.commands {
        Commands::Count { path, exclude } => {
            // exclude.iter_mut().for_each(|str| str.as_str(););
            let path: PathBuf = PathBuf::from(path);
            let exclude_strs: Vec<&str> = exclude
                .iter()
                .map(|s| s.as_str()) // or .map(AsRef::as_ref)
                .collect();
            let locs = count_loc(path, exclude_strs);
            let table: Vec<Vec<CellStruct>> = locs
                .into_iter()
                .map(|loc| {
                    vec![
                        loc.language.cell().justify(Justify::Center),
                        loc.files.cell().justify(Justify::Center),
                        loc.code.cell().justify(Justify::Center),
                        loc.comments.cell().justify(Justify::Center),
                        loc.blanks.cell().justify(Justify::Center),
                        loc.lines.cell().justify(Justify::Center),
                    ]
                })
                .collect();
            let table = table
                .table()
                .title(vec![
                    "Language".cell().bold(true),
                    "Files".cell().bold(true),
                    "Code".cell().bold(true),
                    "Comments".cell().bold(true),
                    "Blanks".cell().bold(true),
                    "Total".cell().bold(true),
                ])
                .border(Border::builder().build());
            print_stdout(table).ok();
        }
    }
}
