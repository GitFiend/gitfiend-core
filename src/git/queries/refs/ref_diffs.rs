use ahash::AHashMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

use crate::git::git_types::{
  Commit, LocalRefCommitDiff, RefCommitDiff, RefInfo, RefLocation,
};
use crate::git::queries::commit_calcs::count_commits_between_commit_ids;
use crate::git::queries::config::GitConfig;
use crate::git::store::{CONFIG, STORE};

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RefDiffOptions {
  pub repo_path: String,
  pub head_commit_id: String,
}

pub fn calc_ref_diffs(
  options: &RefDiffOptions,
) -> Option<(
  HashMap<String, LocalRefCommitDiff>,
  HashMap<String, RefCommitDiff>,
)> {
  let RefDiffOptions {
    repo_path,
    head_commit_id,
    ..
  } = options;

  let (commits, refs) = STORE.get_commits_and_refs(repo_path)?;
  let config = CONFIG.get_by_key(repo_path).unwrap_or_else(GitConfig::new);

  Some(calc_ref_diffs_inner(
    &commits,
    &refs,
    &config,
    head_commit_id,
  ))
}

// We need to pass in head as it may not be found in provided commits in some cases.
pub fn calc_ref_diffs_inner(
  commits: &[Commit],
  refs: &[RefInfo],
  config: &GitConfig,
  head_commit_id: &String,
) -> (
  HashMap<String, LocalRefCommitDiff>,
  HashMap<String, RefCommitDiff>,
) {
  let ref_map = refs.iter().map(|r| (r.id.clone(), r.clone())).collect();
  // let refs = get_ref_info_map_from_commits(commits);
  let pairs = get_ref_pairs(&ref_map, config);

  let commit_map: AHashMap<String, Commit> =
    commits.iter().map(|c| (c.id.clone(), c.clone())).collect();

  let local_ref_diffs = calc_local_ref_diffs(head_commit_id, pairs, &commit_map);
  let remote_ref_diffs = calc_remote_ref_diffs(head_commit_id, &ref_map, &commit_map);

  (local_ref_diffs, remote_ref_diffs)
}

pub fn calc_remote_ref_diffs(
  head_commit_id: &String,
  refs: &AHashMap<String, RefInfo>,
  commits: &AHashMap<String, Commit>,
) -> HashMap<String, RefCommitDiff> {
  refs
    .par_iter()
    .map(|(_, info)| {
      (
        info.id.clone(),
        calc_remote_ref_diff(head_commit_id, info, commits),
      )
    })
    .collect()
}

fn calc_remote_ref_diff(
  head_commit_id: &String,
  info: &RefInfo,
  commits: &AHashMap<String, Commit>,
) -> RefCommitDiff {
  let ref_commit_id = &info.commit_id;

  let ahead_of_head =
    count_commits_between_commit_ids(ref_commit_id, head_commit_id, commits);
  let behind_head =
    count_commits_between_commit_ids(head_commit_id, ref_commit_id, commits);

  RefCommitDiff {
    ahead_of_head,
    behind_head,
  }
}

fn calc_local_ref_diffs(
  head_commit_id: &String,
  pairs: Vec<(RefInfo, Option<RefInfo>)>,
  commits: &AHashMap<String, Commit>,
) -> HashMap<String, LocalRefCommitDiff> {
  pairs
    .into_par_iter()
    .map(|(local, remote)| {
      (
        local.id.clone(),
        calc_local_ref_diff(head_commit_id, local, remote, commits),
      )
    })
    .collect()
}

fn calc_local_ref_diff(
  head_commit_id: &String,
  local: RefInfo,
  remote: Option<RefInfo>,
  commits: &AHashMap<String, Commit>,
) -> LocalRefCommitDiff {
  let local_id = &local.commit_id;

  let ahead_of_head = count_commits_between_commit_ids(local_id, head_commit_id, commits);
  let behind_head = count_commits_between_commit_ids(head_commit_id, local_id, commits);

  if let Some(remote) = remote {
    let remote_id = &remote.commit_id;

    let ahead_of_remote = count_commits_between_commit_ids(local_id, remote_id, commits);
    let behind_remote = count_commits_between_commit_ids(remote_id, local_id, commits);

    LocalRefCommitDiff {
      ahead_of_remote,
      behind_remote,
      ahead_of_head,
      behind_head,
    }
  } else {
    LocalRefCommitDiff {
      ahead_of_remote: 0,
      behind_remote: 0,
      ahead_of_head,
      behind_head,
    }
  }
}

fn get_ref_pairs(
  refs: &AHashMap<String, RefInfo>,
  config: &GitConfig,
) -> Vec<(RefInfo, Option<RefInfo>)> {
  refs
    .iter()
    .map(|(_, r)| r)
    .filter(|r| r.location == RefLocation::Local)
    .map(|r| (r.clone(), get_sibling(r, config, refs)))
    .collect()
}

fn get_sibling(
  ref_info: &RefInfo,
  config: &GitConfig,
  refs: &AHashMap<String, RefInfo>,
) -> Option<RefInfo> {
  let RefInfo {
    sibling_id,
    short_name,
    ..
  } = ref_info;

  if !sibling_id.is_empty() {
    if let Some(sibling) = refs.get(sibling_id) {
      let remote = config.get_remote_for_branch(short_name);

      if let Some(name) = &sibling.remote_name {
        if remote == *name {
          return Some(sibling.clone());
        }
      }
    }
  }

  None
}

// pub fn get_ref_info_map_from_commits(commits: &[Commit]) -> AHashMap<String, RefInfo> {
//   let mut refs: AHashMap<String, RefInfo> = AHashMap::new();
//
//   for c in commits.iter() {
//     for r in c.refs.iter() {
//       if !r.full_name.contains("HEAD") {
//         refs.insert(r.id.clone(), r.clone());
//       }
//     }
//   }
//
//   refs
// }
