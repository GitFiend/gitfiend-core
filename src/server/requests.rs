use crate::{async_requests, dprintln};
use tiny_http::{Response, Server};

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
use crate::git::run_git_action::{clear_action_logs, get_action_logs, poll_action};
use crate::git::store::{clear_cache, override_git_home};

#[cfg(debug_assertions)]
const PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
const PORT: u16 = 0;

const ADDRESS: fn() -> String = || format!("127.0.0.1:{}", PORT);

pub fn start_async_server() {
  let server = Server::http(ADDRESS()).unwrap();

  print_port(server.server_addr().port());

  for mut request in server.incoming_requests() {
    async_requests! {
      request,

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
    };
  }
}

fn print_port(port: u16) {
  // This is parsed by the renderer. Expected to be formatted like:
  // PORT:12345
  println!("PORT:{}", port);
}
