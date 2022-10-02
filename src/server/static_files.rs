use crate::dprintln;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use tiny_http::{Request, Response};

pub fn handle_resource_request(request: Request) {
  let url = request.url();

  if let Some(dir) = get_server_dir() {
    let file_path = dir.join(&url[3..]);

    dprintln!("file_path {:?}", file_path);

    if let Ok(file) = File::open(&file_path) {
      let response = Response::from_file(file);

      let _ = request.respond(response);
    }
  }
}

fn get_server_dir() -> Option<PathBuf> {
  #[cfg(debug_assertions)]
  return Some(env::current_dir().ok()?.parent()?.join("git-fiend"));

  // TODO: Sort this out. May need to unpack all from asar.
  #[cfg(not(debug_assertions))]
  Some(env::current_exe().ok()?.parent()?.parent()?)
}
