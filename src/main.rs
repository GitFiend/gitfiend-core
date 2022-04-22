mod git;
mod parser;
mod server;

use crate::git::queries::config::req_config;
use crate::server::git_request::req_commits;
use parser::input::Input;
use tiny_http::{Response, Server};

#[cfg(debug_assertions)]
const PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
// const PORT: u16 = 0;
const PORT: u16 = 29997;

const ADDRESS: fn() -> String = || format!("127.0.0.1:{}", PORT);

fn main() {
  start_server();
}

fn start_server() {
  let server = Server::http(ADDRESS()).unwrap();

  let port = server.server_addr().port();

  println!("Address: {}:{}", server.server_addr().ip(), port);

  for request in server.incoming_requests() {
    println!(
      "received request! method: {:?}, url: {:?}, headers: {:?}",
      request.method(),
      request.url(),
      request.headers()
    );

    match request.url() {
      "/load-commits" => req_commits(request),
      "/load-config" => req_config(request),
      unknown_request => {
        let response = Response::from_string(format!("Unknown request: '{}'", unknown_request));
        let send_result = request.respond(response);

        println!("{:?}", send_result);
      }
    }
  }
}
