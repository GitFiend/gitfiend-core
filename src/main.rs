use crate::server::server::start_sync_server;

mod git;
mod parser;
mod server;

fn main() {
  // start_server();
  start_sync_server();
}
