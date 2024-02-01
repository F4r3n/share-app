
use std::path::PathBuf;
use anyhow::anyhow;

pub fn get_config_dir() -> Result<PathBuf, anyhow::Error>
{
  if let Some(config_dir) = tauri::api::path::config_dir() {
    return Ok(config_dir.join("share-app"));
  }
  Err(anyhow!("No config path"))
}
