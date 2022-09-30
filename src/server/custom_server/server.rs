use crate::git::actions::clone::clone_repo;
use crate::git::actions::command::command;
use crate::git::actions::create_repo::create_repo;
use crate::git::actions::credentials::set_credentials;
use crate::git::actions::fetch::fetch_all;
use crate::git::actions::stash::{stash_changes, stash_staged};
use crate::git::conflicts::api::load_conflicted_file;
use crate::git::git_version::git_version;
use crate::git::queries::commits::{
  commit_ids_between_commits, commit_is_ancestor, get_un_pushed_commits, load_commits_and_stashes,
  load_head_commit, load_top_commit_for_branch,
};
use crate::git::queries::config::load_full_config;
use crate::git::queries::hunks::images::load_commit_image;
use crate::git::queries::hunks::load_hunks::load_hunks;
use crate::git::queries::patches::patches::load_patches;
use crate::git::queries::patches::patches_for_commit::load_patches_for_commit;
use crate::git::queries::refs::head_info::calc_head_info;
use crate::git::queries::refs::ref_diffs::calc_ref_diffs;
use crate::git::queries::run::run;
use crate::git::queries::scan_workspace::scan_workspace;
use crate::git::queries::search::search_commits::search_commits;
use crate::git::queries::search::search_request::{poll_diff_search, start_diff_search};
use crate::git::queries::wip::is_merge_in_progress;
use crate::git::queries::wip::wip_diff::{load_wip_hunk_lines, load_wip_hunks};
use crate::git::queries::wip::wip_patches::load_wip_patches;
use crate::git::run_git_action::{
  clear_action_logs, get_action_logs, poll_action, read_available_string_data,
};
use crate::git::store::{clear_cache, override_git_home};
use crate::requests;
use crate::server::custom_server::http::{parse_http_request, HttpRequest};
use std::io::Read;
use std::net::{TcpListener, TcpStream};

#[cfg(debug_assertions)]
const _PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
const _PORT: u16 = 29997;
// const PORT: u16 = 0;

const _ADDRESS: fn() -> String = || format!("127.0.0.1:{}", _PORT);

pub fn _start_sync_server() {
  let listener = TcpListener::bind(_ADDRESS()).unwrap();

  if let Ok(r) = listener.local_addr() {
    println!("Port: {}", r.port())
  };

  for stream in listener.incoming() {
    let mut stream = stream.unwrap();

    _handle_connection(&mut stream);
  }
}

fn _handle_connection(stream: &mut TcpStream) {
  if let Some(request) = _get_request_from_stream(stream) {
    println!("Body: {}", &request.body);

    requests!(
      request,
      stream,
      // Queries
      run,
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
      load_wip_hunks,
      load_wip_hunk_lines,
      git_version,
      scan_workspace,
      calc_ref_diffs,
      start_diff_search,
      poll_diff_search,
      load_patches_for_commit,
      search_commits,
      commit_is_ancestor,
      load_conflicted_file,
      load_commit_image,
      calc_head_info,
      // Core messages
      clear_cache,
      get_action_logs,
      clear_action_logs,
      set_credentials,
      poll_action,
      override_git_home,
      // Actions
      command,
      stash_changes,
      fetch_all,
      clone_repo,
      create_repo,
      stash_staged
    );
  }
}

fn _get_request_from_stream(stream: &mut TcpStream) -> Option<HttpRequest> {
  // let mut buffer = [0; 20048];

  let request_text = read_available_string_data(stream);

  // if let Err(e) = stream.read(&mut buffer) {
  //   println!("Failed to read tcp stream: {}", e);
  //
  //   return None;
  // }
  //
  // let request_text = String::from_utf8_lossy(&buffer[..]).to_string();

  // println!(
  //   "request_text: {}, length: {}",
  //   request_text,
  //   request_text.len()
  // );

  parse_http_request(request_text)
}
