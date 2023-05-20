
use std::path::PathBuf;
use std::fs;
use anyhow::{anyhow};


pub fn get_config_dir() -> Result<PathBuf, anyhow::Error>
{
  if let Some(config_dir) = tauri::api::path::config_dir() {
    return Ok(config_dir.join("share-app"));
  }
  Err(anyhow!("No config path"))
}

pub fn get_config_dir_temp() -> Result<PathBuf, anyhow::Error>
{
  if let Ok(config_dir) = get_config_dir() {
    return Ok(config_dir.join("temp"));
  }
  Err(anyhow!("No config path"))

}

pub fn create_config_temp() -> Result<(), anyhow::Error>
{
  if let Ok(config_dir) = get_config_dir_temp() {
    if config_dir.is_dir() == false {
        return fs::create_dir(config_dir).map_err(Into::into)
    }
  }
  Ok(())
}

pub fn delete_config_temp() -> Result<(), anyhow::Error>
{
  if let Ok(config_dir) = get_config_dir_temp() {
    return fs::remove_dir(config_dir).map_err(Into::into)
  }
  Ok(())
}