use crate::git::git_types::{Commit, GitConfig, RefInfo, RefLocation};
use std::collections::HashMap;

// We need to pass in head as it may not be found in provided commits in some cases.
pub fn calc_ref_diffs(commits: &Vec<Commit>, config: &GitConfig, head_commit_id: &String) {
  let refs = get_ref_info_map_from_commits(commits);
  let pairs = get_ref_pairs(&refs, config);

  //
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
