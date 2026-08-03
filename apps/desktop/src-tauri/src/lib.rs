use database::connection::get_connection;
use sqlx::{Pool, Sqlite};
use tauri::Manager;



struct AppState {
    db: Pool<Sqlite>
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let db = get_connection().await;
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AppState { 
                db
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
