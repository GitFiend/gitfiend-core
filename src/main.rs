use crate::server::async_server::start_async_server;
// use crate::server::server::start_sync_server;

mod git;
mod parser;
mod server;

fn main() {
  // start_sync_server();
  start_async_server();
}
