use crate::git::git_request::ReqOptions;
use crate::git::queries::commit_calcs::count_commits_between_fallback;
use crate::git::queries::config::load_full_config;
use crate::git::queries::config::GitConfig;
use crate::git::queries::wip::wip_patches::{load_wip_patches, WipPatches};
use crate::git::queries::workspace::load_current_branch::{
  load_current_branch, read_refs, Refs,
};
use crate::git::queries::workspace::load_packed_refs::{load_packed_refs, PackedRef};
use crate::git::repo_watcher::clear_repo_changed_status;
use crate::git::request_util::R;
use serde::Serialize;
use std::collections::HashSet;
use ts_rs::TS;

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RepoStatus {
  patches: WipPatches,
  config: GitConfig,
  // These are just short names. Don't include remote name or whether local.
  branches: HashSet<String>,
  branch_name: String,
  head_ref_id: String,
  local_commit_id: Option<String>,
  remote_commit_id: Option<String>,
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

pub fn load_repo_status(options: &ReqOptions) -> R<RepoStatus> {
  let ReqOptions { repo_path } = options;

  let patches = load_wip_patches(options)?;
  let config = load_full_config(options)?;

  let (head_id, current_branch) = load_current_branch(repo_path)?;

  let Refs {
    local_id,
    mut remote_id,
    mut others,
  } = read_refs(repo_path, &current_branch)?;

  let packed_refs = load_packed_refs(repo_path).unwrap_or_else(|_| Vec::new());

  if remote_id.is_none() {
    for r in packed_refs.iter() {
      if let PackedRef::Remote(remote) = r {
        if remote.name == current_branch {
          remote_id = Some(remote.commit_id.clone());
          break;
        }
      }
    }
  }

  others.extend(packed_refs.iter().flat_map(|r| match r {
    PackedRef::Local(l) => Some(l.name.clone()),
    PackedRef::Remote(r) => Some(r.name.clone()),
    PackedRef::Unknown => None,
  }));

  if let Some(local_id) = local_id.clone() {
    if let Some(remote_id) = remote_id {
      let remote_ahead = count_commits_between_fallback(repo_path, &local_id, &remote_id);
      let remote_behind =
        count_commits_between_fallback(repo_path, &remote_id, &local_id);

      clear_repo_changed_status(options);

      return Ok(RepoStatus {
        patches,
        config,
        branches: others,
        branch_name: current_branch,
        head_ref_id: head_id,
        local_commit_id: Some(local_id),
        remote_commit_id: Some(remote_id),
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

  Ok(RepoStatus {
    patches,
    config,
    branches: others,
    branch_name: current_branch,
    head_ref_id: head_id,
    local_commit_id: local_id,
    remote_commit_id: remote_id,
    remote_ahead: 0,
    remote_behind: 0,
    state,
  })
}
