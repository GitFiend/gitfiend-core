use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::git::request_util::{ES, R};
use serde::Deserialize;
use ts_rs::TS;

pub fn get_content_type(file_path: &str) -> Option<String> {
  let guess = mime_guess::from_path(file_path);

  Some(format!("Content-Type: {}", guess.first()?))
}

pub fn path_exists(file_path: &String) -> bool {
  Path::new(file_path).exists()
}

pub fn temp_dir(_: &String) -> R<String> {
  Ok(String::from(
    env::temp_dir()
      .to_str()
      .ok_or(ES::from("temp_dir: Couldn't convert to str."))?,
  ))
}

pub fn file_size(file_path: &String) -> R<u64> {
  Ok(Path::new(file_path).metadata()?.len())
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct WriteFileOpts {
  pub file_path: String,
  pub content: String,
}

pub fn write_file(options: &WriteFileOpts) -> R<bool> {
  let WriteFileOpts { file_path, content } = options;

  let mut file = File::create(file_path)?;
  file.write_all(content.as_ref())?;

  Ok(true)
}
