use crate::git::actions::clone::clone_repo;
use crate::git::actions::command::{command, commands};
use crate::git::actions::create_repo::create_repo;
use crate::git::actions::credentials::set_credentials;
use crate::git::actions::fetch::fetch_all;
use crate::git::actions::stash::{stash_changes, stash_staged};
use crate::git::conflicts::api::load_conflicted_file;
use crate::git::git_version::git_version;
use crate::git::queries::commits::{
  commit_ids_between_commits, commit_is_ancestor, commit_is_on_branch,
  get_all_commits_on_current_branch, load_commits_and_refs, load_head_commit,
  load_top_commit_for_branch,
};
use crate::git::queries::config::load_full_config;
use crate::git::queries::hunks::images::load_commit_image;
use crate::git::queries::hunks::load_hunks::load_hunks;
use crate::git::queries::patches::patches_for_commit::load_patches_for_commit;
use crate::git::queries::refs::head_info::calc_head_info;
use crate::git::queries::refs::ref_diffs::calc_ref_diffs;
use crate::git::queries::run::run;
use crate::git::queries::scan_workspace::scan_workspace;
use crate::git::queries::search::search_commits::search_commits;
use crate::git::queries::search::search_request::{poll_diff_search, start_diff_search};
use crate::git::queries::unpushed_commits::get_un_pushed_commits;
use crate::git::queries::wip::wip_diff::{load_wip_hunk_lines, load_wip_hunks};
use crate::git::queries::wip::wip_patches::load_wip_patches;
use crate::git::queries::wip::{is_merge_in_progress, is_rebase_in_progress};
use crate::git::repo_watcher::{
  clear_repo_changed_status, repo_has_changed, stop_watching_repo, watch_repo,
};
use crate::git::run_git_action::poll_action2;
use crate::git::store::{clear_all_caches, clear_cache, override_git_home};
use crate::index::auto_complete::auto_complete;
use crate::server::static_files::{file_size, path_exists, temp_dir, write_file};
use crate::util::data_store::{get_data_store, set_data_store};

macro_rules! handler {
    ($name:expr, $options:expr, $($handler:ident),*) => {{
    match $name {
      $(
      stringify!($handler) => {
        let options = serde_json::from_str($options)?;

        let result = $handler(&options);

        serde_json::to_string(&result)
      },
      )*
      unknown_function => {
        panic!("Unknown function {}", unknown_function);
      }
    }
  }};
}

#[tauri::command]
pub fn handle_rpc_request(name: &str, options: &str) -> Option<String> {
  if let Ok(result) = run_request(name, options) {
    return Some(result);
  }
  None
}

fn run_request(name: &str, options: &str) -> serde_json::Result<String> {
  println!("Received request: {}, {}", name, options);

  handler! {
    name,
    options,

    // Queries
    run,
    scan_workspace,
    load_commits_and_refs,
    load_full_config,
    load_head_commit,
    load_top_commit_for_branch,
    commit_ids_between_commits,
    load_hunks,
    is_merge_in_progress,
    is_rebase_in_progress,
    load_wip_patches,
    get_un_pushed_commits,
    load_wip_hunks,
    load_wip_hunk_lines,
    git_version,
    calc_ref_diffs,
    start_diff_search,
    poll_diff_search,
    load_patches_for_commit,
    search_commits,
    commit_is_ancestor,
    commit_is_on_branch,
    get_all_commits_on_current_branch,
    load_conflicted_file,
    load_commit_image,
    calc_head_info,
    repo_has_changed,
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
    stop_watching_repo,
    get_data_store,
    set_data_store,

    // Actions
    command,
    commands,
    stash_changes,
    fetch_all,
    clone_repo,
    create_repo,
    stash_staged
  }
}
