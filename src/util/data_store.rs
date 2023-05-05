use crate::config::{APPLICATION, ORGANISATION, QUALIFIER};
use crate::server::git_request::ReqOptions;
use directories::ProjectDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use ts_rs::TS;

pub fn get_data_store(_: &ReqOptions) -> Option<HashMap<String, String>> {
  load_config()
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DataStoreValues {
  pub data: HashMap<String, String>,
}

pub fn set_data_store(o: &DataStoreValues) -> Option<()> {
  let DataStoreValues { data } = o;

  let config_file_path = get_config_file_path()?;

  let mut config_file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(config_file_path)
    .ok()?;

  let config_text = serde_json::to_string_pretty(&data).ok()?;

  config_file.write_all(config_text.as_bytes()).ok()?;

  Some(())
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ConfigValueOptions {
  pub key: String,
}

pub fn get_config_value(option: &ConfigValueOptions) -> Option<String> {
  let ConfigValueOptions { key } = option;

  let config = load_config()?;

  config.get(key).cloned()
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SetConfigOptions {
  pub key: String,
  pub value: String,
}

pub fn set_config_value(option: &SetConfigOptions) -> Option<()> {
  let SetConfigOptions { key, value } = option;

  let mut config = load_config()?;
  config.insert(key.clone(), value.clone());

  let config_file_path = get_config_file_path()?;

  let mut config_file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(config_file_path)
    .ok()?;

  let config_text = serde_json::to_string_pretty(&config).ok()?;

  config_file.write_all(config_text.as_bytes()).ok()?;

  Some(())
}

fn load_config() -> Option<HashMap<String, String>> {
  let config_file_path = get_config_file_path()?;

  let file = File::open(config_file_path).ok()?;

  let mut reader = BufReader::new(file);
  let mut text = String::new();
  reader.read_to_string(&mut text).ok()?;

  serde_json::from_str::<HashMap<String, String>>(&text).ok()
}

fn get_config_file_path() -> Option<PathBuf> {
  if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANISATION, APPLICATION) {
    let cache_dir = proj_dirs.config_dir();

    Some(cache_dir.join("data_store.json"))
  } else {
    None
  }
}
