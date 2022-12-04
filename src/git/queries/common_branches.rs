use crate::git::git_types::{Commit, RefInfo, RefType};
use crate::git::queries::refs::get_ref_info_from_commits;
use crate::git::repo_watcher::get_watched_repos;
use crate::git::store::{get_all_workspace_commits, RepoPath};
use crate::server::git_request::ReqOptions;
use ahash::{AHashMap, AHashSet};
use std::collections::HashMap;

pub fn get_common_branches(_: &ReqOptions) -> Option<Vec<String>> {
  let repos: AHashMap<RepoPath, Vec<Commit>> = get_all_workspace_commits()?;
  let watched_repos: HashMap<RepoPath, bool> = get_watched_repos()?;

  let mut counts: AHashMap</* short_name */ String, AHashSet</* repo_path */ String>> =
    AHashMap::new();

  let expected_num = watched_repos.len();

  for (repo_path, commits) in repos {
    if !watched_repos.contains_key(&repo_path) {
      continue;
    }

    let refs = get_ref_info_from_commits(&commits);

    for RefInfo {
      short_name,
      ref_type,
      ..
    } in refs
    {
      if ref_type == RefType::Branch {
        if let Some(count) = counts.get_mut(&short_name) {
          count.insert(repo_path.clone());
        } else {
          let mut count = AHashSet::new();
          count.insert(repo_path.clone());
          counts.insert(short_name, count);
        }
      }
    }
  }

  let shared = counts
    .into_iter()
    .filter(|(_, repo_paths)| repo_paths.len() == expected_num)
    .map(|(short_name, _)| short_name)
    .collect::<Vec<String>>();

  if shared.is_empty() {
    println!("get_common_branches: empty result.");
    println!(
      "repos.len(): {:?}, num commits in each: {:?}",
      expected_num,
      get_all_workspace_commits()?
        .iter()
        .map(|(_, c)| { c.len() })
        .collect::<Vec<usize>>()
    );
  }

  Some(shared)
}
