use std::io::Write;

use anyhow::anyhow;
use tauri::AppHandle;

use crate::util::app_path::create_config_dir;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ConnectionConfig {
    nick_name: String,
    server: String,
    channel: String,
    password: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct UploadImageConfig {
    url_post: String,
    url_get: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    connection_config: ConnectionConfig,
    upload_image: UploadImageConfig,
}

pub fn load_settings(app_handle: AppHandle) -> Result<Settings, anyhow::Error> {
    let path = create_config_dir(app_handle);
    if let Ok(path) = path {
        let f = std::fs::File::open(path.join(".config.txt"))?;
        let settings: Settings = serde_yaml::from_reader(f)?;
        return Ok(settings);
    }
    Err(anyhow!("No settings found"))
}

pub fn save_settings(app_handle: AppHandle, settings: &Settings) -> Result<(), anyhow::Error> {
    let path = create_config_dir(app_handle);
    if let Ok(path) = path {
        let mut f = std::fs::File::create(path.join(".config.txt"))?;
        let settings: String = serde_yaml::to_string(&settings)?;
        f.write_all(settings.as_bytes())?;
        return Ok(());
    }
    Err(anyhow!("Incorrect path"))
}
