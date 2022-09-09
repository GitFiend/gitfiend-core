use crate::git::git_settings::set_git_env;
use crate::git::git_version::load_git_version;
use crate::server::requests::start_async_server;

pub(crate) mod git;
mod parser;
mod server;
mod util;

fn main() {
  set_git_env();
  load_git_version();
  start_async_server();
}
