use core_lib::git::git_request::ReqOptions;
use core_lib::{dprintln, handler};
use serde::de::Error;

pub fn run_app_request(name: &str, options: &str) -> serde_json::Result<String> {
  handler! {
    name,
    options,

    show_open_folder_window
  }
}

fn show_open_folder_window(_: &ReqOptions) -> Option<()> {
  None
}
