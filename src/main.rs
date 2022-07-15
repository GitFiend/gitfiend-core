use crate::git::git_settings::set_git_env;
use crate::server::async_server::start_async_server;

pub(crate) mod git;
mod parser;
mod server;
mod util;

fn main() {
  set_git_env();
  start_async_server();
}
