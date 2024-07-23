use std::path::Path;
use std::path::PathBuf;
use tauri::Manager;
pub fn get_config_dir(app_handle: tauri::AppHandle) -> Result<PathBuf, anyhow::Error> {
    app_handle
        .path()
        .resolve("share-app", tauri::path::BaseDirectory::Config)
        .map_err(anyhow::Error::msg)
}

pub fn create_config_dir(app_handle: tauri::AppHandle) -> Result<PathBuf, anyhow::Error> {
    match get_config_dir(app_handle) {
        Ok(r) => {
            if Path::exists(&r) {
                return Ok(r);
            }
            let _ = std::fs::create_dir_all(std::path::Path::new(&r)).map_err(anyhow::Error::msg);
            Ok(r)
        }
        Err(e) => Err(e),
    }
}
