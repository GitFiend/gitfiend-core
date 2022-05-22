use crate::server::http::{parse_http_request, HttpRequest};

use std::io::Read;
use std::net::{TcpListener, TcpStream};

#[cfg(debug_assertions)]
const _PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
// const PORT: u16 = 0;
const _PORT: u16 = 29997;

const _ADDRESS: fn() -> String = || format!("127.0.0.1:{}", _PORT);

pub fn _start_sync_server() {
  let listener = TcpListener::bind(_ADDRESS()).unwrap();

  if let Ok(r) = listener.local_addr() {
    println!("Port: {}", r.port())
  };

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    _handle_connection(stream);
  }
}

fn _handle_connection(stream: TcpStream) {
  let result = _get_request_from_stream(&stream);

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

fn _get_request_from_stream(mut stream: &TcpStream) -> Option<HttpRequest> {
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
