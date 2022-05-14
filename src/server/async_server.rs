use crate::git::queries::commits::{
  commit_ids_between_commits, get_un_pushed_commits, load_commits_and_stashes, load_head_commit,
  load_top_commit_for_branch,
};
use crate::git::queries::config::load_full_config;
use crate::git::queries::hunks::hunks::load_hunks;
use crate::git::queries::patches::patches::load_patches;
use crate::git::queries::wip::is_merge_in_progress;
use crate::git::queries::wip::wip_patches::load_wip_patches;
use crate::git::store::Store;
use serde::Deserialize;
use serde_json::from_str;
use std::error::Error;
use std::sync::{Arc, RwLock};
use tiny_http::{Request, Response, Server};

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

fn get_body(mut request: Request) -> Result<String, Box<dyn Error>> {
  let mut content = String::new();

  if let Err(e) = request.as_reader().read_to_string(&mut content) {
    println!("{}", e);
  }

  Ok(content)
}

fn parse_json<'a, O: Deserialize<'a>>(body: &'a String) -> Option<O> {
  match from_str::<O>(&body) {
    Ok(options) => Some(options),
    Err(e) => {
      println!("{}", e);

      None
    }
  }
}
//
// fn handle_request<'a, O: Deserialize<'a>, R: Serialize>(
//   mut request: Request,
//   handler: fn(&O) -> R,
// ) -> Option<()> {
//   let body = get_body(request)?;
//
//   // handle_request_inner(&body, handler);
//
//   None
// }

// fn handle_request<'a, O: Deserialize<'a>, R: Serialize>(
//   mut request: Request,
//   handler: fn(&O) -> R,
// ) -> Option<()> {
//   let body = get_body(request).ok()?.as_str();
//
//   handle_request_inner(body, handler);
//
//   None
// }
//
// fn handle_request_inner<'a, O: Deserialize<'a>, R: Serialize>(
//   body: &'a str,
//   handler: fn(&O) -> R,
// ) -> Result<(), Box<dyn Error>> {
//   // let body = get_body(request)?;
//   let options = from_str(body)?;
//
//   let handler_result = handler(&options);
//   let serialized = serde_json::to_string(&handler_result)?;
//
//   let response = format!(
//     "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//     serialized.len(),
//     serialized
//   );
//
//   // stream.write(response.as_bytes())?;
//   // stream.flush()?;
//
//   Ok(())
// }
//
// fn handle_request_inner<'a, O: Deserialize<'a>, R: Serialize>(
//   body: &'a String,
//   handler: fn(&O) -> R,
// ) -> Option<()> {
//   let options = parse_json::<O>(&body);
//
//   None
// }

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
  ($request:expr, $store:expr, $handler: ident) => {{
    match parse_json!($request) {
      Some(options) => {
        send_response!($request, $handler(&options, $store));
      }
      None => {}
    };
  }};
}

#[macro_export]
macro_rules! async_requests {
  ($request:expr, $store:expr, $($handler:ident),*) => {{
    match $request.url() {
      $(
      concat!("/", stringify!($handler)) => {
        handle_request!($request, $store, $handler);
      },
      )*
      unknown_url => {
        println!("Unknown url {}", unknown_url);
      }
    }
  }};
}

pub fn start_async_server() {
  let server = Server::http(ADDRESS()).unwrap();

  let port = server.server_addr().port();

  let store = Store::new_lock();

  println!("Address: {}:{}", server.server_addr().ip(), port);

  for mut request in server.incoming_requests() {
    async_requests!(
      request,
      store.clone(),
      load_commits_and_stashes,
      load_full_config,
      load_head_commit,
      load_top_commit_for_branch,
      commit_ids_between_commits,
      load_patches,
      load_hunks,
      is_merge_in_progress,
      load_wip_patches,
      get_un_pushed_commits
    );
  }
}
