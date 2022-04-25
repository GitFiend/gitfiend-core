use crate::git::queries::config::load_full_config;
use crate::git::queries::patches::patches::load_all_commit_patches;
use crate::server::http::parse_http_request;
use crate::{
  commit_ids_between_commits, handle_request2, load_commits_and_stashes, load_head_commit,
  load_top_commit_for_branch,
};
use std::fmt::format;
use std::io::prelude::*;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::time::Instant;

#[cfg(debug_assertions)]
const PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
// const PORT: u16 = 0;
const PORT: u16 = 29997;

const ADDRESS: fn() -> String = || format!("127.0.0.1:{}", PORT);

pub fn start_server2() {
  let listener = TcpListener::bind(ADDRESS()).unwrap();

  match listener.local_addr() {
    Ok(r) => {
      println!("Port: {}", r.port())
    }
    Err(_) => {}
  }

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 2048];

  stream.read(&mut buffer).unwrap();

  let request_text = String::from_utf8_lossy(&buffer[..]).to_string();
  let result = parse_http_request(request_text);

  println!("{:?}", result);
  if result.is_some() {
    let request = result.unwrap();

    match request.url.as_str() {
      "/load-commits" => handle_request2!(request, stream, load_commits_and_stashes),
      "/load-config" => handle_request2!(request, stream, load_full_config),
      "/head-commit" => handle_request2!(request, stream, load_head_commit),
      "/top-commit" => handle_request2!(request, stream, load_top_commit_for_branch),
      "/ids-between-commits" => handle_request2!(request, stream, commit_ids_between_commits),
      "/load-all-commit-patches" => handle_request2!(request, stream, load_all_commit_patches),
      // "/load-commits" => match serde_json::from_str(request.body.as_str()) {
      //   Ok(options) => {
      //     let commits = load_commits_and_stashes(&options);
      //
      //     let serialized = serde_json::to_string(&commits).unwrap();
      //
      //     let response = format!(
      //       "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
      //       serialized.len(),
      //       serialized
      //     );
      //
      //     stream.write(response.as_bytes()).unwrap();
      //     stream.flush().unwrap();
      //     return;
      //   }
      //   Err(e) => {
      //     println!("{}", e);
      //   }
      // },
      unknown_url => {}
    }
  }

  // let message = "hello";
  //
  // let response = format!(
  //   "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
  //   message.len(),
  //   message
  // );
  //
  // stream.write(response.as_bytes()).unwrap();
  // stream.flush().unwrap();
}

#[cfg(test)]
mod tests {
  use crate::server::server::start_server2;

  #[test]
  fn test_start_server() {
    start_server2();
  }
}
