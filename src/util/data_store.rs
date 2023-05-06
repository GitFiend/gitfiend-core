use crate::config::{APPLICATION, ORGANISATION, QUALIFIER};
use crate::server::git_request::ReqOptions;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use ts_rs::TS;

pub fn get_data_store(_: &ReqOptions) -> UserConfigResult {
  load_config()
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DataStoreValues {
  pub data: HashMap<String, String>,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ResultStatus {
  pub success: bool,
  pub message: String,
}

impl ResultStatus {
  pub fn success(message: &str) -> ResultStatus {
    ResultStatus {
      success: true,
      message: message.to_string(),
    }
  }

  pub fn failure(message: &str) -> ResultStatus {
    ResultStatus {
      success: false,
      message: message.to_string(),
    }
  }
}

pub fn set_data_store(o: &DataStoreValues) -> ResultStatus {
  let DataStoreValues { data } = o;

  match get_config_file_path() {
    None => ResultStatus::failure("Failed to get config file path"),
    Some(config_file_path) => {
      println!("config_file_path: {:?}", config_file_path);

      match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(config_file_path)
      {
        Err(e) => ResultStatus::failure(&format!("Failed to open config file: {}", e)),
        Ok(mut config_file) => match serde_json::to_string_pretty(&data) {
          Err(e) => ResultStatus::failure(&format!("Failed to serialize data: {}", e)),
          Ok(config_text) => {
            println!("config_text: {}", config_text);

            match config_file.write_all(config_text.as_bytes()) {
              Err(e) => ResultStatus::failure(&format!("Failed to write to config file: {}", e)),
              Ok(_) => ResultStatus::success("Data store updated"),
            }
          }
        },
      }
    }
  }
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum UserConfigResult {
  Error(String),
  Config(HashMap<String, String>),
}

fn load_config() -> UserConfigResult {
  match get_config_file_path() {
    None => UserConfigResult::Error("Failed to get config file path".to_string()),
    Some(config_file_path) => match File::open(config_file_path) {
      Err(e) => UserConfigResult::Error(format!("Failed to open config file: {}", e)),
      Ok(file) => {
        let mut reader = BufReader::new(file);
        let mut text = String::new();
        match reader.read_to_string(&mut text) {
          Err(e) => UserConfigResult::Error(format!("Failed to read config file: {}", e)),
          Ok(_) => match serde_json::from_str::<HashMap<String, String>>(&text) {
            Err(e) => UserConfigResult::Error(format!("Failed to parse config file: {}", e)),
            Ok(config) => UserConfigResult::Config(config),
          },
        }
      }
    },
  }
}

fn get_config_file_path() -> Option<PathBuf> {
  if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANISATION, APPLICATION) {
    let dir = proj_dirs.config_dir();

    create_dir_all(dir).ok()?;

    Some(dir.join("data_store.json"))
  } else {
    None
  }
}
