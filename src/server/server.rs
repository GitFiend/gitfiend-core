use crate::load_commits_and_stashes;
use crate::server::http::parse_http_request;
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

const ADDRESS: fn() -> String = || format!("127.0.0.1:{}", PORT - 1);

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
      "/load-commits" => match serde_json::from_str(request.body.as_str()) {
        Ok(options) => {
          let commits = load_commits_and_stashes(&options);

          let serialized = serde_json::to_string(&commits).unwrap();

          let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            serialized.len(),
            serialized
          );

          stream.write(response.as_bytes()).unwrap();
          stream.flush().unwrap();
          return;
        }
        Err(e) => {
          println!("{}", e);
        }
      },
      unknown_url => {}
    }
  }

  let message = "hello";

  let response = format!(
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    message.len(),
    message
  );

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

#[cfg(test)]
mod tests {
  use crate::server::server::start_server2;

  #[test]
  fn test_start_server() {
    start_server2();
  }
}
