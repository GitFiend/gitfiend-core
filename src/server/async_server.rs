use tiny_http::{Response, Server};

use crate::git::git_version;
use crate::git::queries::commits::{
  commit_ids_between_commits, get_un_pushed_commits, load_commits_and_stashes, load_head_commit,
  load_top_commit_for_branch,
};
use crate::git::queries::config::load_full_config;
use crate::git::queries::hunks::hunks::load_hunks;
use crate::git::queries::patches::patches::load_patches;
use crate::git::queries::patches::patches_for_commit::load_patches_for_commit;
use crate::git::queries::refs::ref_diffs::calc_ref_diffs;
use crate::git::queries::scan_workspace::scan_workspace;
use crate::git::queries::search::search::search_commits;
use crate::git::queries::search::search_request::{poll_diff_search, start_diff_search};
use crate::git::queries::wip::is_merge_in_progress;
use crate::git::queries::wip::wip_diff::{load_wip_hunk_lines, load_wip_hunks};
use crate::git::queries::wip::wip_patches::load_wip_patches;
use crate::git::store::clear_cache;
use crate::server::graph_instructions::api::graph_instructions;

#[cfg(debug_assertions)]
const PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
const PORT: u16 = 0;

const ADDRESS: fn() -> String = || format!("127.0.0.1:{}", PORT);

/*
TODO: Stop passing options into each request as reference.

Stop using unwrap in these macros.

Convert macros where possible to functions.
 */

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
macro_rules! async_requests {
  ($request:expr, $($handler:ident),*) => {{
    match $request.url() {
      $(
      concat!("/", stringify!($handler)) => {
        handle_request!($request, $handler);
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

  print_port(server.server_addr().port());

  for mut request in server.incoming_requests() {
    async_requests! {
      request,

      load_commits_and_stashes,
      load_full_config,
      load_head_commit,
      load_top_commit_for_branch,
      commit_ids_between_commits,
      load_patches,
      load_hunks,
      is_merge_in_progress,
      load_wip_patches,
      get_un_pushed_commits,
      clear_cache,
      load_wip_hunks,
      load_wip_hunk_lines,
      git_version,
      scan_workspace,
      calc_ref_diffs,
      graph_instructions,
      start_diff_search,
      poll_diff_search,
      load_patches_for_commit,
      search_commits
    };
  }
}

fn print_port(port: u16) {
  // This is parsed by the renderer. Expected to be formatted like:
  // PORT:12345
  println!("PORT:{}", port);
}
