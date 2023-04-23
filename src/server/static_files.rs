use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tiny_http::{Header, Request, Response};
use ts_rs::TS;

pub fn handle_resource_request(request: Request) -> Option<()> {
  let dir = get_server_dir()?;

  // Remove any extra query part.
  let url = request.url().split('?').next()?;
  let file_path = dir.join(&url[3..]);

  #[cfg(debug_assertions)]
  println!("file_path {:?}, exists: {}", file_path, file_path.exists());

  let file = File::open(&file_path).ok()?;
  let mut response = Response::from_file(file);

  let content_type = get_content_type(&file_path.to_string_lossy())?;

  let header = Header::from_str(&content_type).ok()?;
  response.add_header(header);

  let _ = request.respond(response);

  Some(())
}

fn get_content_type(file_path: &str) -> Option<String> {
  let guess = mime_guess::from_path(file_path);

  Some(format!("Content-Type: {}", guess.first()?))
}

fn get_server_dir() -> Option<PathBuf> {
  #[cfg(debug_assertions)]
  return Some(env::current_dir().ok()?.parent()?.join("git-fiend"));

  // TODO: Sort this out. May need to unpack all from asar.
  #[cfg(not(debug_assertions))]
  Some(env::current_exe().ok()?.parent()?.parent()?.to_path_buf())
}

pub fn path_exists(file_path: &String) -> bool {
  Path::new(file_path).exists()
}

pub fn temp_dir(_: &String) -> Option<String> {
  Some(String::from(env::temp_dir().to_str()?))
}

pub fn file_size(file_path: &String) -> Option<u64> {
  Some(Path::new(file_path).metadata().ok()?.len())
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct WriteFileOpts {
  pub file_path: String,
  pub content: String,
}

pub fn write_file(options: &WriteFileOpts) -> Option<bool> {
  let WriteFileOpts { file_path, content } = options;

  let mut file = File::create(file_path).ok()?;
  file.write_all(content.as_ref()).ok()?;

  Some(true)
}
