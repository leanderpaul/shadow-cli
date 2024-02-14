use crate::{constants, error::Error, utils};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
  pub fiction_dir: String,
}

lazy_static! {
  static ref CONFIG: Option<Config> = load_config();
}

pub fn get_config() -> Result<Config, Error> {
  match CONFIG.clone() {
    Some(config) => Ok(config),
    None => Err(Error::new("Not a shadow repository or any of the parent directories")),
  }
}

pub fn get_config_or_exit() -> Config {
  match get_config() {
    Ok(config) => config,
    Err(error) => error.print_and_exit(),
  }
}

pub fn save_config(config: &Config) {
  let config_buffer = match rmp_serde::to_vec(config) {
    Ok(buffer) => buffer,
    Err(_) => Error::new("Failed to serialize the config").print_and_exit(),
  };

  let config_file_path = format!("{}/config", crate::constants::CONFIG_DIR);
  utils::save_to_file(&config_file_path, &config_buffer);
}

fn load_config() -> Option<Config> {
  let mut dir = match env::current_dir() {
    Ok(dir) => dir,
    Err(_) => Error::new("Failed to get the current directory").print_and_exit(),
  };

  loop {
    let mut file_path = dir.clone();
    file_path.push(constants::CONFIG_DIR);
    file_path.push(constants::CONFIG_FILE);

    if file_path.is_file() {
      return get_config_from_file(&file_path);
    }

    let has_parent = dir.pop();
    if !has_parent {
      return None;
    }
  }
}

fn get_config_from_file(file: &PathBuf) -> Option<Config> {
  let contents = match fs::read_to_string(file) {
    Ok(contents) => contents,
    Err(_) => return None,
  };
  match serde_json::from_str(&contents) {
    Ok(config) => Some(config),
    Err(_) => None,
  }
}
