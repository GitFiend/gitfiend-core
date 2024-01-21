use crate::server::requests::start_async_server;
use core_lib::git::git_settings::set_git_env;
use core_lib::git::git_version::load_git_version;

pub mod server;

fn main() {
  set_git_env();
  load_git_version();
  start_async_server();
}
