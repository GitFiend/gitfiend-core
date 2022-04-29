use crate::git::queries::config::load_full_config;
use crate::server::server::start_sync_server;
use git::queries::commits::{
  commit_ids_between_commits, load_commits_and_stashes, load_head_commit,
  load_top_commit_for_branch,
};
use parser::input::Input;
use tiny_http::{Response, Server};

mod git;
mod parser;
mod server;

#[cfg(debug_assertions)]
const PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
// const PORT: u16 = 0;
const PORT: u16 = 29997;

const ADDRESS: fn() -> String = || format!("127.0.0.1:{}", PORT);

fn main() {
  // start_server();
  start_sync_server();
}

fn start_server() {
  let server = Server::http(ADDRESS()).unwrap();

  let port = server.server_addr().port();

  println!("Address: {}:{}", server.server_addr().ip(), port);

  for mut request in server.incoming_requests() {
    println!("received url: {:?}", request.url());

    match request.url() {
      "/load_commits_and_stashes" => handle_request!(request, load_commits_and_stashes),
      "/load_full_config" => handle_request!(request, load_full_config),
      "/load_head_commit" => handle_request!(request, load_head_commit),
      "/load_top_commit_for_branch" => handle_request!(request, load_top_commit_for_branch),
      "/commit_ids_between_commits" => handle_request!(request, commit_ids_between_commits),
      unknown_url => print_request_error!(unknown_url, request),
    }
  }
}
