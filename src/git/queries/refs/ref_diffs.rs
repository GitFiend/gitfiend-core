use crate::git::git_types::{Commit, GitConfig, LocalRefCommitDiff, RefInfo, RefLocation};
use crate::git::queries::commit_calcs::count_commits_between_commit_ids;
use std::collections::HashMap;

// We need to pass in head as it may not be found in provided commits in some cases.
pub fn calc_ref_diffs(commits: &Vec<Commit>, config: &GitConfig, head_commit_id: &String) {
  let refs = get_ref_info_map_from_commits(commits);
  let pairs = get_ref_pairs(&refs, config);

  let local_ref_diffs = calc_local_ref_diffs(head_commit_id, pairs, commits);
}

fn calc_local_ref_diffs(
  head_commit_id: &String,
  pairs: Vec<(RefInfo, Option<RefInfo>)>,
  commits: &Vec<Commit>,
) -> HashMap<String, LocalRefCommitDiff> {
  let mut diffs = HashMap::<String, LocalRefCommitDiff>::new();

  for (local, remote) in pairs {
    diffs.insert(
      local.id.clone(),
      calc_local_ref_diff(head_commit_id, local, remote, commits),
    );
  }

  diffs
}

fn calc_local_ref_diff(
  head_commit_id: &String,
  local: RefInfo,
  remote: Option<RefInfo>,
  commits: &Vec<Commit>,
) -> LocalRefCommitDiff {
  let ref local_id = local.commit_id;

  let ahead_of_head = count_commits_between_commit_ids(local_id, head_commit_id, commits) as u32;
  let behind_head = count_commits_between_commit_ids(head_commit_id, local_id, commits) as u32;

  if let Some(remote) = remote {
    let ref remote_id = remote.commit_id;

    let ahead_of_remote = count_commits_between_commit_ids(local_id, remote_id, commits) as u32;
    let behind_remote = count_commits_between_commit_ids(remote_id, local_id, commits) as u32;

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
  refs: &HashMap<String, RefInfo>,
  config: &GitConfig,
) -> Vec<(RefInfo, Option<RefInfo>)> {
  refs
    .iter()
    .map(|(_, r)| r)
    .filter(|r| r.location == RefLocation::Local)
    .map(|r| (r.clone(), get_sibling(&r, config, refs)))
    .collect()
}

fn get_sibling(
  ref_info: &RefInfo,
  config: &GitConfig,
  refs: &HashMap<String, RefInfo>,
) -> Option<RefInfo> {
  let RefInfo {
    sibling_id,
    short_name,
    ..
  } = ref_info;

  if let Some(id) = sibling_id {
    if let Some(sibling) = refs.get(id) {
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

pub fn get_ref_info_map_from_commits(commits: &Vec<Commit>) -> HashMap<String, RefInfo> {
  let mut refs: HashMap<String, RefInfo> = HashMap::new();

  for c in commits.iter() {
    for r in c.refs.iter() {
      if !r.full_name.contains("HEAD") {
        refs.insert(r.id.clone(), r.clone());
      }
    }
  }

  refs
}
