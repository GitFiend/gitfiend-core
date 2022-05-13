use crate::git::queries::config::load_full_config;
use crate::git::queries::patches::patches::load_patches;
use crate::server::http::{parse_http_request, HttpRequest};

use crate::git::queries::commits::{
  commit_ids_between_commits, load_commits_and_stashes, load_head_commit,
  load_top_commit_for_branch,
};
use crate::git::queries::hunks::hunks::load_hunks;
use crate::git::queries::wip::is_merge_in_progress;
use crate::git::queries::wip::wip_patches::load_wip_patches;
use crate::requests;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

#[cfg(debug_assertions)]
const PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
// const PORT: u16 = 0;
const PORT: u16 = 29997;

const ADDRESS: fn() -> String = || format!("127.0.0.1:{}", PORT);

pub fn start_sync_server() {
  let listener = TcpListener::bind(ADDRESS()).unwrap();

  if let Ok(r) = listener.local_addr() {
    println!("Port: {}", r.port())
  };

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(stream: TcpStream) {
  let result = get_request_from_stream(&stream);

  if result.is_some() {
    let request = result.unwrap();
    println!("Body: {}", &request.body);

    // requests!(
    //   request,
    //   stream,
    //   load_commits_and_stashes,
    //   load_full_config,
    //   load_head_commit,
    //   load_top_commit_for_branch,
    //   commit_ids_between_commits,
    //   load_patches,
    //   load_hunks,
    //   is_merge_in_progress,
    //   load_wip_patches
    // );
  }
}

fn get_request_from_stream(mut stream: &TcpStream) -> Option<HttpRequest> {
  let mut buffer = [0; 20048];

  if let Err(e) = stream.read(&mut buffer) {
    println!("Failed to read tcp stream: {}", e);

    return None;
  }

  let request_text = String::from_utf8_lossy(&buffer[..]).to_string();

  println!(
    "request_text: {}, length: {}",
    request_text,
    request_text.len()
  );

  parse_http_request(request_text)
}
