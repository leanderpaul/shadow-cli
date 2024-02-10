use crate::app_error::AppError;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
  #[serde(rename = "encryptionKey")]
  pub encryption_key: String,

  #[serde(rename = "fictionDir")]
  pub fiction_dir: Option<String>,
}

lazy_static! {
  static ref CONFIG: Option<Config> = load_config();
}

pub fn get_config() -> Result<Config, AppError> {
  return match CONFIG.clone() {
    Some(config) => Ok(config),
    None => Err(AppError::new("Not a shadow repository or any of the parent directories")),
  };
}

pub fn get_config_or_exit() -> Config {
  let config = get_config();

  return match config {
    Ok(config) => config,
    Err(error) => error.print_and_exit(),
  };
}

pub fn save_config(config: &Config) -> Result<(), AppError> {
  let config = serde_json::to_string_pretty(config);
  if config.is_err() {
    return Err(AppError::new("Failed to serialize the config"));
  }

  let contents = config.unwrap();
  let result = fs::write("shadow.config.json", contents);
  if result.is_err() {
    return Err(AppError::new("Failed to write the config file"));
  }

  return Ok(());
}

fn load_config() -> Option<Config> {
  let config = fs::read_to_string("shadow.config.json");
  if config.is_err() {
    return None;
  }

  let config = config.unwrap();
  return match serde_json::from_str(&config) {
    Ok(config) => Some(config),
    Err(_) => None,
  };
}
