use common::AppState;
use tauri::Manager;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let state = AppState::new().await;
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
