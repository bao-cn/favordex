use std::sync::Arc;
use tokio::sync::Semaphore;

mod commands;
mod models;
mod services;
mod ai_supplier;

#[derive(Clone)]
pub struct AppConfig {
    pub permit: Arc<Semaphore>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppConfig {
            permit: Arc::new(Semaphore::new(3)),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_bookmark_folders,
            commands::get_bookmarks_by_folder,
            commands::get_ai_models,
            commands::check_browsers,
            commands::backup_bookmarks,
            commands::check_backup,
            commands::get_bookmarks_num,
            commands::get_all_bookmarks,
            commands::organize_bookmarks,
            commands::overlay_bookmarks_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
