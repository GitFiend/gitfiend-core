use serde::{Deserialize, Serialize};
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

#[macro_export]
macro_rules! handle_request2 {
  ($request:expr, $stream:expr, $handler: ident) => {{
    match serde_json::from_str($request.body.as_str()) {
      Ok(options) => {
        let commits = $handler(&options);

        let serialized = serde_json::to_string(&commits).unwrap();

        let response = format!(
          "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
          serialized.len(),
          serialized
        );

        $stream.write(response.as_bytes()).unwrap();
        $stream.flush().unwrap();
      }
      Err(e) => {
        println!("{}", e);
      }
    };
  }};
}
