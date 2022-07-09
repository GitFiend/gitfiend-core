use crate::server::async_server::start_async_server;

pub(crate) mod git;
mod parser;
mod server;
mod util;

fn main() {
  start_async_server();
}
