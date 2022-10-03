
use std::path::PathBuf;
use std::fs;

pub fn get_config_dir() -> Option<PathBuf>
{
  if let Some(config_dir) = tauri::api::path::config_dir() {
    return Some(config_dir.join("share-app"));
  }
  None
}

pub fn get_config_dir_temp() -> Option<PathBuf>
{
  if let Some(config_dir) = get_config_dir() {
    return Some(config_dir.join("temp"));
  }
  None
}

pub fn create_config_temp() -> Result<(), anyhow::Error>
{
  if let Some(config_dir) = get_config_dir_temp() {
    if config_dir.is_dir() == false {
        return fs::create_dir(config_dir).map_err(Into::into)
    }
  }
  Ok(())
}

pub fn delete_config_temp() -> Result<(), anyhow::Error>
{
  if let Some(config_dir) = get_config_dir_temp() {
    return fs::remove_dir(config_dir).map_err(Into::into)
  }
  Ok(())
}