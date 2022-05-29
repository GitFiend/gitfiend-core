use crate::git::git_types::{
  Commit, GitConfig, LocalRefCommitDiff, RefCommitDiff, RefInfo, RefLocation,
};
use crate::git::queries::commit_calcs::count_commits_between_commit_ids;
use crate::git::store::{load_commits_from_store, load_config_from_store, RwStore};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RefDiffOptions {
  pub repo_path: String,
  pub head_commit_id: String,
}

pub fn calc_ref_diffs(
  options: &RefDiffOptions,
  store: RwStore,
) -> Option<(
  HashMap<String, LocalRefCommitDiff>,
  HashMap<String, RefCommitDiff>,
)> {
  let RefDiffOptions {
    repo_path,
    head_commit_id,
    ..
  } = options;

  let commits = load_commits_from_store(&repo_path, &store)?;
  let config = load_config_from_store(&store)?;

  let now = Instant::now();

  let res = Some(calc_ref_diffs_inner(&commits, &config, head_commit_id));

  println!("Took {}ms for calc_ref_diffs", now.elapsed().as_millis(),);

  res
}

// We need to pass in head as it may not be found in provided commits in some cases.
pub fn calc_ref_diffs_inner(
  commits: &Vec<Commit>,
  config: &GitConfig,
  head_commit_id: &String,
) -> (
  HashMap<String, LocalRefCommitDiff>,
  HashMap<String, RefCommitDiff>,
) {
  let refs = get_ref_info_map_from_commits(commits);
  let pairs = get_ref_pairs(&refs, config);

  let commit_map: HashMap<String, Commit> = commits
    .clone()
    .into_iter()
    .map(|c| (c.id.clone(), c))
    .collect();

  let now = Instant::now();

  let local_ref_diffs = calc_local_ref_diffs(head_commit_id, pairs, &commit_map);
  println!(
    "Took {}ms to calc_local_ref_diffs",
    now.elapsed().as_millis(),
  );

  let now = Instant::now();
  let remote_ref_diffs = calc_remote_ref_diffs(head_commit_id, &refs, &commit_map);
  println!(
    "Took {}ms to calc_remote_ref_diffs",
    now.elapsed().as_millis(),
  );
  (local_ref_diffs, remote_ref_diffs)
}

fn calc_remote_ref_diffs(
  head_commit_id: &String,
  refs: &HashMap<String, RefInfo>,
  commits: &HashMap<String, Commit>,
) -> HashMap<String, RefCommitDiff> {
  // let mut diffs = HashMap::<String, RefCommitDiff>::new();
  //
  // for (_, info) in refs {
  //   diffs.insert(
  //     info.id.clone(),
  //     calc_remote_ref_diff(head_commit_id, info, commits),
  //   );
  // }

  refs
    .into_par_iter()
    .map(|(_, info)| {
      (
        info.id.clone(),
        calc_remote_ref_diff(head_commit_id, info, commits),
      )
    })
    .collect()

  // diffs
}

fn calc_remote_ref_diff(
  head_commit_id: &String,
  info: &RefInfo,
  commits: &HashMap<String, Commit>,
) -> RefCommitDiff {
  let ref ref_commit_id = info.commit_id;

  let ahead_of_head = count_commits_between_commit_ids(ref_commit_id, head_commit_id, commits);
  let behind_head = count_commits_between_commit_ids(head_commit_id, ref_commit_id, commits);

  RefCommitDiff {
    ahead_of_head,
    behind_head,
  }
}

fn calc_local_ref_diffs(
  head_commit_id: &String,
  pairs: Vec<(RefInfo, Option<RefInfo>)>,
  commits: &HashMap<String, Commit>,
) -> HashMap<String, LocalRefCommitDiff> {
  // let mut diffs = HashMap::<String, LocalRefCommitDiff>::new();

  // for (local, remote) in pairs {
  //   diffs.insert(
  //     local.id.clone(),
  //     calc_local_ref_diff(head_commit_id, local, remote, commits),
  //   );
  // }

  let diffs = pairs
    .into_par_iter()
    .map(|(local, remote)| {
      (
        local.id.clone(),
        calc_local_ref_diff(head_commit_id, local, remote, commits),
      )
    })
    .collect();

  diffs
}

fn calc_local_ref_diff(
  head_commit_id: &String,
  local: RefInfo,
  remote: Option<RefInfo>,
  commits: &HashMap<String, Commit>,
) -> LocalRefCommitDiff {
  let ref local_id = local.commit_id;

  let ahead_of_head = count_commits_between_commit_ids(local_id, head_commit_id, commits);
  let behind_head = count_commits_between_commit_ids(head_commit_id, local_id, commits);

  if let Some(remote) = remote {
    let ref remote_id = remote.commit_id;

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
