use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqOptions {
  pub repo_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqCommitsOptions {
  pub repo_path: String,
  pub num_commits: u32,
}

pub fn _handle_sync_request<'a, O: Deserialize<'a>, R: Serialize>(
  body: &'a str,
  mut stream: TcpStream,
  handler: fn(&O) -> R,
) -> Result<(), Box<dyn Error>> {
  let options = from_str(body)?;

  let handler_result = handler(&options);
  let serialized = serde_json::to_string(&handler_result)?;

  let response = format!(
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    serialized.len(),
    serialized
  );

  stream.write(response.as_bytes())?;
  stream.flush()?;

  Ok(())
}

#[macro_export]
macro_rules! requests {
  ($request:expr, $stream:expr, $($handler:ident),*) => {{
    let url = $request.url.as_str();
    let body = $request.body.as_str();

    match url {
      $(
      concat!("/", stringify!($handler)) => {
        if let Err(e) = crate::server::git_request::handle_sync_request(body, $stream, $handler) {
           println!("{}", e);
        }
      },
      )*
      unknown_url => {
        println!("Unknown url {}", unknown_url);
      }
    }
  }};
}
