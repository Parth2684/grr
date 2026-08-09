use std::{fs, path::PathBuf};

use database::connection::get_connection;
use sqlx::{Pool, Sqlite};

pub mod loc;
pub mod hash;

#[derive(Clone)]
pub struct AppState {
    pub local_data_dir: PathBuf,
    pub db: Pool<Sqlite>
}

impl AppState {
    pub async fn new() -> Self {
        let db = get_connection().await;
        let local_data_dir = dirs::data_local_dir().expect("Error getting local data dir of system").join("com.parth.grr");
        if !local_data_dir.exists() {
            fs::create_dir_all(&local_data_dir).expect("Error creating local data folder for the application");
        }
        
        AppState {
            db,
            local_data_dir
        }
    }
}