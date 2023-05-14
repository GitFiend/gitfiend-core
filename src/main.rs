// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::git::git_settings::set_git_env;
use crate::git::git_version::load_git_version;
use crate::handle_request::handle_rpc_request;
use crate::server::requests::start_async_server;

mod config;
pub(crate) mod git;
mod handle_request;
mod index;
mod parser;
mod server;
mod util;

const USE_SERVER: bool = false;

fn main() {
  if USE_SERVER {
    set_git_env();
    load_git_version();
    start_async_server();
  } else {
    tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![handle_rpc_request])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  }
}
