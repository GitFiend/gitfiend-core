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
