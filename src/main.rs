use crate::git::git_settings::set_git_env;
use crate::git::git_version::load_git_version;
use crate::server::requests::start_async_server;
use crate::ui::window::make_application_window;
use std::thread;

mod config;
pub mod git;
mod index;
mod parser;
mod server;
mod ui;
mod util;

fn main() -> iced::Result {
  set_git_env();
  load_git_version();

  let no_server = true;

  thread::spawn(move || {
    if !no_server {
      start_async_server();
    }
  });

  make_application_window()
}
