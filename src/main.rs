mod git;
mod parser;

use crate::git::queries::commits::load_commits;
use parser::input::Input;
use std::time::Instant;
use tiny_http::{Response, Server};

#[cfg(debug_assertions)]
const PORT: u16 = 29998;
#[cfg(not(debug_assertions))]
const PORT: u16 = 0;

fn main() {
  let now = Instant::now();

  load_commits(5000);

  println!("load commits took {}ms", now.elapsed().as_millis());

  start_server();
}

fn start_server() {
  let addr = format!("127.0.0.1:{}", PORT);

  let server = Server::http(addr).unwrap();

  let port = server.server_addr().port();

  println!("Address: {}:{}", server.server_addr().ip(), port);

  for request in server.incoming_requests() {
    println!(
      "received request! method: {:?}, url: {:?}, headers: {:?}",
      request.method(),
      request.url(),
      request.headers()
    );

    let response = Response::from_string("hello world");
    request.respond(response);
  }
}
