use std::{fs, path::PathBuf};

// use database::connection::{get_local_data_path, get_sqlite_connection};
use sqlx::{Pool, Sqlite};

pub mod tokei;
pub mod hash;

#[derive(Clone)]
pub struct AppState {
    // pub local_data_dir: PathBuf,
    // pub sqlite_db: Pool<Sqlite>
}

impl AppState {
    pub async fn new() -> Self {
        // let local_data_dir = get_local_data_path();
        // let sqlite_db = get_sqlite_connection().await;
        
        AppState {
            // sqlite_db,
            // local_data_dir
        }
    }
}