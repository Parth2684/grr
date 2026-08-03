use database::connection::get_connection;
use sqlx::{Pool, Sqlite};

pub mod loc;

pub struct AppState {
    db: Pool<Sqlite>
}

impl AppState {
    pub async fn new() -> Self {
        let db = get_connection().await;
        AppState {
            db
        }
    }
}