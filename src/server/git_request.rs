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

#[macro_export]
macro_rules! parse_json {
  ($request: expr) => {{
    let mut content = String::new();
    $request.as_reader().read_to_string(&mut content).unwrap();

    match serde_json::from_str(&content) {
      Ok(options) => options,
      Err(e) => {
        println!("{}", e);
        None
      }
    }
  }};
}

#[macro_export]
macro_rules! send_response {
  ($request: expr, $result: expr) => {{
    let serialized = serde_json::to_string(&$result).unwrap();

    match $request.respond(Response::from_string(serialized)) {
      Ok(_) => {}
      Err(e) => {
        println!("{}", e);
      }
    };
  }};
}

#[macro_export]
macro_rules! handle_request {
  ($request:expr, $handler: ident) => {{
    match parse_json!($request) {
      Some(options) => {
        send_response!($request, $handler(&options));
      }
      None => {}
    };
  }};
}

#[macro_export]
macro_rules! print_request_error {
  ($url:expr, $request:expr) => {{
    let response = Response::from_string(format!("Unknown request: '{}'", $url));
    let send_result = $request.respond(response);

    println!("{:?}", send_result);
  }};
}

// #[macro_export]
// macro_rules! handle_sync_request {
//   ($request:expr, $stream:expr, $handler: ident) => {{
//     match serde_json::from_str($request.body.as_str()) {
//       Ok(options) => {
//         let commits = $handler(&options);
//
//         let serialized = serde_json::to_string(&commits).unwrap();
//
//         let response = format!(
//           "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//           serialized.len(),
//           serialized
//         );
//
//         $stream.write(response.as_bytes()).unwrap();
//         $stream.flush().unwrap();
//       }
//       Err(e) => {
//         println!("{}", e);
//       }
//     };
//   }};
// }

pub fn handle_sync_request<'a, O: Deserialize<'a>, R: Serialize>(
  body: &'a str,
  mut stream: TcpStream,
  handler: fn(&O) -> Option<R>,
) {
  match from_str(body) {
    Ok(options) => {
      let commits = handler(&options);

      let serialized = serde_json::to_string(&commits).unwrap();

      let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        serialized.len(),
        serialized
      );

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
