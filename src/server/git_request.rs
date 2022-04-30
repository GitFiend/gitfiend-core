use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::io::prelude::*;
use std::net::TcpStream;
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqOptions {
  pub repo_path: String,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqCommitsOptions {
  pub repo_path: String,
  pub num_commits: u32,
}

pub fn handle_sync_request<'a, O: Deserialize<'a>, R: Serialize>(
  body: &'a str,
  mut stream: TcpStream,
  handler: fn(&O) -> Option<R>,
) {
  match from_str(body) {
    Ok(options) => {
      let commits = handler(&options);

      // TODO: Unchecked unwrap.
      let serialized = serde_json::to_string(&commits).unwrap();

      let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        serialized.len(),
        serialized
      );

      // TODO: Unchecked unwrap.
      stream.write(response.as_bytes()).unwrap();
      stream.flush().unwrap();
    }
    Err(e) => {
      println!("{}", e);
    }
  };
}

#[macro_export]
macro_rules! requests {
  ($request:expr, $stream:expr, $($handler:ident),*) => {{
    let url = $request.url.as_str();
    let body = $request.body.as_str();

    match url {
      $(
      concat!("/", stringify!($handler)) => {
        crate::server::git_request::handle_sync_request(body, $stream, $handler)
      },
      )*
      unknown_url => {
        println!("Unknown url {}", unknown_url);
      }
    }
  }};
}
