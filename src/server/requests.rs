use std::process::exit;

use tiny_http::{Response, Server};

use crate::git::actions::add::git_add_files;
use crate::git::actions::clone::clone_repo;
use crate::git::actions::command::command;
use crate::git::actions::create_repo::create_repo;
use crate::git::actions::credentials::set_credentials;
use crate::git::actions::fetch::fetch_all;
use crate::git::actions::stash::{stash_changes, stash_staged};
use crate::git::conflicts::api::load_conflicted_file;
use crate::git::git_version::git_version;
use crate::git::queries::commits::{
  commit_ids_between_commits, commit_is_ancestor, commit_is_on_branch,
  get_all_commits_on_current_branch, load_commits_and_refs,
};
use crate::git::queries::hunks::html_code::get_patch_as_html;
use crate::git::queries::hunks::images::load_commit_image;
use crate::git::queries::hunks::load_hunks::{load_hunks, load_hunks_split};
use crate::git::queries::patches::patches_for_commit::load_patches_for_commit;
use crate::git::queries::refs::ref_diffs::calc_ref_diffs;
use crate::git::queries::run::run;
use crate::git::queries::scan_workspace::scan_workspace;
use crate::git::queries::search::search_commits::search_commits;
use crate::git::queries::search::search_request::{poll_diff_search, start_diff_search};
use crate::git::queries::unpushed_commits::get_un_pushed_commits;
use crate::git::queries::wip::is_rebase_in_progress;
use crate::git::queries::wip::wip_diff::{
  load_wip_hunk_lines, load_wip_hunks, load_wip_hunks_split,
};
use crate::git::queries::wip::wip_patches::load_wip_patches;
use crate::git::queries::workspace::ws_repo::load_repo_status;
use crate::git::repo_watcher::{clear_repo_changed_status, repo_has_changed, watch_repo};
use crate::git::run_git_action::poll_action2;
use crate::git::store::{clear_all_caches, clear_cache, override_git_home};
use crate::index::auto_complete::auto_complete;
use crate::server::static_files::{
  file_size, handle_resource_request, path_exists, temp_dir, write_file,
};
use crate::util::data_store::{get_data_store, set_data_store};
use crate::{dprintln, handle_function_request};

#[cfg(debug_assertions)]
const PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
const PORT: u16 = 0;

const ADDRESS: fn() -> String = || format!("127.0.0.1:{}", PORT);

pub fn start_async_server() {
  let server = Server::http(ADDRESS()).expect("Started server");

  print_port(
    server
      .server_addr()
      .to_ip()
      .expect("Get port for printing")
      .port(),
  );

  for mut request in server.incoming_requests() {
    match &request.url()[..3] {
      "/r/" => {
        handle_resource_request(request);
      }
      "/pi" => {
        let _ = request.respond(Response::from_string("gitfiend"));
      }
      "/ex" => {
        let _ = request.respond(Response::from_string("GitFiend core exiting..."));
        exit(0);
      }
      "/f/" => {
        handle_function_request! {
          request,

          // Queries
          git_version,
          run,

          scan_workspace,
          repo_has_changed,
          load_repo_status,

          is_rebase_in_progress,
          load_commits_and_refs,

          load_hunks,
          load_hunks_split,
          load_wip_hunks,
          load_wip_hunk_lines,
          load_wip_hunks_split,
          load_conflicted_file,
          get_patch_as_html,

          load_wip_patches,
          load_patches_for_commit,
          load_commit_image,

          commit_ids_between_commits,
          get_un_pushed_commits,
          calc_ref_diffs,
          commit_is_ancestor,
          commit_is_on_branch,
          get_all_commits_on_current_branch,

          search_commits,
          start_diff_search,
          poll_diff_search,
          auto_complete,

          // TODO: Will this work in a sand-boxed mac app?
          path_exists,
          temp_dir,
          file_size,
          write_file,

          // Core messages
          clear_cache,
          clear_all_caches,
          clear_repo_changed_status,
          set_credentials,
          poll_action2,
          override_git_home,
          watch_repo,
          get_data_store,
          set_data_store,

          // Actions
          command,
          git_add_files,
          stash_changes,
          fetch_all,
          clone_repo,
          create_repo,
          stash_staged
        }
      }
      _ => {
        dprintln!("Unhandled url {}", request.url());
      }
    }
  }
}

fn print_port(port: u16) {
  // This is required by the renderer. Expected to be formatted like:
  // PORT:12345
  // We pad the width so we can read a specific number of chars from the stream.
  println!("PORT:{:<12}", port);
}
