use crate::git::git_types::{Commit, RefInfo};
use crate::git::queries::refs::ref_diffs::{calc_remote_ref_diffs, get_ref_info_map_from_commits};
use crate::git::store::COMMITS;
use crate::server::git_request::ReqOptions;
use ahash::AHashMap;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct HeadInfo {
  pub ref_info: RefInfo,
  pub commit: Commit,
  pub remote_ref: Option<RefInfo>,
  pub remote_commit: Option<Commit>,
  pub remote_ahead: u32,
  pub remote_behind: u32,
}

pub fn calc_head_info(options: &ReqOptions) -> Option<HeadInfo> {
  let ReqOptions { repo_path } = options;

  calc_head_info_from_commits(repo_path.to_string());

  None
}

// Note: This depends on COMMITS and REF_DIFFS already being loaded.
// Change this to just take commits? Better to have calls to COMMITS at api level.
fn calc_head_info_from_commits(repo_path: String) -> Option<HeadInfo> {
  let commits = COMMITS.get_by_key(&repo_path)?;
  if commits.is_empty() {
    return None;
  }

  let all_refs = get_ref_info_map_from_commits(&commits);

  let commit_map = commits
    .into_iter()
    .map(|c| (c.id.clone(), c))
    .collect::<AHashMap<String, Commit>>();

  let mut remote_ahead = 0;
  let mut remote_behind = 0;

  for (_, commit) in &commit_map {
    for info in &commit.refs {
      if info.head {
        let mut remote_ref: Option<&RefInfo> = None;
        let mut remote_commit: Option<&Commit> = None;

        if let Some(sibling_id) = &info.sibling_id {
          remote_ref = all_refs.get(sibling_id);
          if let Some(remote_ref) = remote_ref {
            remote_commit = commit_map.get(&remote_ref.commit_id);
          } else {
            // TODO: It may still exist, but not be part of our commit batch.
            // Maybe this is too unlikely? A user would need to commit 1000 times without pushing
            // for this to happen?
            // Unless they set a small number of commits to show in the future?
          }
        }

        if let Some(remote_ref) = remote_ref {
          let diffs_map = calc_remote_ref_diffs(&commit.id, &all_refs, &commit_map);

          if let Some(diffs) = diffs_map.get(&remote_ref.id) {
            remote_ahead = diffs.ahead_of_head; //
            remote_behind = diffs.behind_head;
          }
        }

        return Some(HeadInfo {
          ref_info: info.clone(),
          commit: commit.clone(),
          remote_ref: remote_ref.cloned(),
          remote_commit: remote_commit.cloned(),
          remote_ahead,
          remote_behind,
        });
      }
    }
  }

  None
}
