use std::{fs, path::PathBuf};
use sqlx::{Pool, Sqlite, sqlite::{SqliteConnectOptions, SqlitePoolOptions}};

pub fn get_database_path() -> PathBuf {
    let app_local_data_dir = dirs::data_local_dir()
        .expect("Error getting local data directory")
        .join("com.parth.grr");
    if !app_local_data_dir.exists() {
        fs::create_dir_all(&app_local_data_dir).expect("Error creating local data folder for the application");
    }
    app_local_data_dir.join("data.db")
}

pub async fn get_connection() -> Pool<Sqlite> {
    let db_path = get_database_path();
    let connect_options = SqliteConnectOptions::new()
        .create_if_missing(true)
        .filename(db_path);

    SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .connect_with(connect_options)
        .await
        .expect("Error connecting to database")
}