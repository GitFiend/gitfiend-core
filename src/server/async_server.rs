use crate::git::queries::commits::{
  commit_ids_between_commits, load_commits_and_stashes, load_head_commit,
  load_top_commit_for_branch,
};
use crate::git::queries::config::load_full_config;
use tiny_http::{Response, Server};

#[cfg(debug_assertions)]
const PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
// const PORT: u16 = 0;
const PORT: u16 = 29997;

const ADDRESS: fn() -> String = || format!("127.0.0.1:{}", PORT);

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

pub fn start_server() {
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
