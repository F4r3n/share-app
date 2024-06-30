use std::path::PathBuf;
use tauri::Manager;
pub fn get_config_dir(app_handle: tauri::AppHandle) -> Result<PathBuf, anyhow::Error> {
    app_handle.path().resolve("share-app", tauri::path::BaseDirectory::Config).map_err(|e| anyhow::Error::msg(e))
}
