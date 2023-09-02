use crate::f;
use crate::git::git_types::GitConfig;
use crate::git::queries::commit_calcs::count_commits_between_fallback;
use crate::git::queries::commits::convert_commit;
use crate::git::queries::config::load_full_config;
use crate::git::queries::refs::head_info::{calc_head_fallback, calc_remote_fallback, HeadInfo};
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
pub struct WsRepoState {
  patches: WipPatches,
  config: GitConfig,
  head_info: HeadInfo,
}

pub fn load_ws_repo(options: &ReqOptions) -> R<WsRepoState> {
  let ReqOptions { repo_path } = options;

  let current_branch = load_current_branch(repo_path)?;
  println!("Current branch: {}", current_branch);

  // read_refs(repo_path, &current_branch);

  // let other_branches =
  //   read_refs(repo_path, &current_branch).map_err(|e| f!("Failed to read refs: {}", e))?;
  // println!("{} {:?}", current_branch, other_branches);

  let patches = load_wip_patches(options)?;

  let config = load_full_config(options)?;

  let (mut head_commit, i) = calc_head_fallback(repo_path)?;
  let head_ref = &mut head_commit.refs[i];

  if let Ok((remote_ahead, remote_commit, remote_behind, remote_ref)) =
    calc_remote_fallback(repo_path, head_ref)
  {
    let info = HeadInfo {
      ref_info: head_ref.clone(),
      commit: convert_commit(head_commit),
      remote_ref: Some(remote_ref),
      remote_commit: Some(remote_commit),
      remote_ahead,
      remote_behind,
    };

    clear_repo_changed_status(options);

    return Ok(WsRepoState {
      patches,
      config,
      head_info: info,
    });
  }

  Err(f!("ruh roh"))
}

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

  let remote_ahead = count_commits_between_fallback(repo_path, &local_id, &remote_id);
  let remote_behind = count_commits_between_fallback(repo_path, &remote_id, &local_id);

  clear_repo_changed_status(options);

  Ok(WsRepoState2 {
    patches,
    config,
    branches: others,
    branch_name: current_branch,
    remote_ahead,
    remote_behind,
  })
}
