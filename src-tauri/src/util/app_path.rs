use std::path::PathBuf;
use std::path::Path;
use tauri::Manager;
pub fn get_config_dir(app_handle: tauri::AppHandle) -> Result<PathBuf, anyhow::Error> {
    app_handle.path().resolve("share-app", tauri::path::BaseDirectory::Config).map_err(|e| anyhow::Error::msg(e))
}

pub fn create_config_dir(app_handle: tauri::AppHandle)-> Result<PathBuf, anyhow::Error> 
{
    match get_config_dir(app_handle) {
        Ok(r) => {
            if Path::exists(&r) {
                return Ok(r);
            }
            let _ = std::fs::create_dir_all(std::path::Path::new(&r)).map_err(|e| anyhow::Error::msg(e));
            Ok(r)
        },
        Err(e) => Err(e)
    }
}