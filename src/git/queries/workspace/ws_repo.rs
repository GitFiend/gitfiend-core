use crate::git::git_types::GitConfig;
use crate::git::queries::commit_calcs::count_commits_between_fallback;
use crate::git::queries::config::load_full_config;
use crate::git::queries::wip::wip_patches::{load_wip_patches, WipPatches};
use crate::git::queries::workspace::load_current_branch::{load_current_branch, read_refs, Refs};
use crate::git::repo_watcher::clear_repo_changed_status;
use crate::server::git_request::ReqOptions;
use crate::server::request_util::R;
use serde::Serialize;
use std::collections::HashSet;
use ts_rs::TS;

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct WsRepoState2 {
  patches: WipPatches,
  config: GitConfig,
  branches: HashSet<String>,
  branch_name: String,
  remote_ahead: u32,
  remote_behind: u32,
  state: BranchState,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub enum BranchState {
  Local,
  Remote,
  Both,
}

pub fn load_ws_repo2(options: &ReqOptions) -> R<WsRepoState2> {
  let ReqOptions { repo_path } = options;

  let patches = load_wip_patches(options)?;
  let config = load_full_config(options)?;

  let current_branch = load_current_branch(repo_path)?;

  let Refs {
    local_id,
    remote_id,
    others,
  } = read_refs(repo_path, &current_branch)?;

  if let Some(local_id) = local_id.clone() {
    if let Some(remote_id) = remote_id {
      let remote_ahead = count_commits_between_fallback(repo_path, &local_id, &remote_id);
      let remote_behind = count_commits_between_fallback(repo_path, &remote_id, &local_id);

      clear_repo_changed_status(options);

      return Ok(WsRepoState2 {
        patches,
        config,
        branches: others,
        branch_name: current_branch,
        remote_ahead,
        remote_behind,
        state: BranchState::Both,
      });
    }
  }

  let state = match (local_id.is_some(), remote_id.is_some()) {
    (true, true) => BranchState::Both,
    (true, false) => {
      if others.contains("HEAD") {
        BranchState::Remote
      } else {
        BranchState::Local
      }
    }
    (false, true) => BranchState::Remote,
    (false, false) => BranchState::Local,
  };

  clear_repo_changed_status(options);

  Ok(WsRepoState2 {
    patches,
    config,
    branches: others,
    branch_name: current_branch,
    remote_ahead: 0,
    remote_behind: 0,
    state,
  })
}
