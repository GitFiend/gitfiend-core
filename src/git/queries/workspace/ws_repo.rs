use crate::f;
use crate::git::git_types::GitConfig;
use crate::git::queries::commits::convert_commit;
use crate::git::queries::config::load_full_config;
use crate::git::queries::refs::head_info::{calc_head_fallback, calc_remote_fallback, HeadInfo};
use crate::git::queries::wip::wip_patches::{load_wip_patches, WipPatches};
use crate::git::queries::workspace::load_current_branch::{load_current_branch, read_refs};
use crate::git::repo_watcher::clear_repo_changed_status;
use crate::server::git_request::ReqOptions;
use crate::server::request_util::R;
use serde::Serialize;
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

  read_refs(repo_path, &current_branch);

  // let other_branches =
  //   read_refs(repo_path, &current_branch).map_err(|e| f!("Failed to read refs: {}", e))?;
  // println!("{} {:?}", current_branch, other_branches);

  let patches = load_wip_patches(options)?;

  let config = load_full_config(options).ok_or("Failed to load config")?;

  let (mut head_commit, i) = calc_head_fallback(repo_path)?;
  let head_ref = &mut head_commit.refs[i];

  if let Some((remote_ahead, remote_commit, remote_behind, remote_ref)) =
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
