use crate::git::git_types::{Commit, RefInfo, RefType};
use crate::git::queries::refs::get_ref_info_from_commits;
use crate::git::store::get_all_workspace_commits;
use crate::server::git_request::ReqOptions;
use ahash::{AHashMap, AHashSet};

pub fn get_common_branches(_: &ReqOptions) -> Option<Vec<String>> {
  let repos: AHashMap<String, Vec<Commit>> = get_all_workspace_commits()?;

  println!(
    "repos.len(): {:?}, num commits in each: {:?}",
    repos.len(),
    repos
      .iter()
      .map(|(_, c)| { c.len() })
      .collect::<Vec<usize>>()
  );

  let mut counts: AHashMap</* short_name */ String, AHashSet</* repo_path */ String>> =
    AHashMap::new();

  let expected_num = repos.len();

  for (repo_path, commits) in repos {
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

  println!("counts: {:?}", counts);

  let shared = counts
    .into_iter()
    .filter(|(_, repo_paths)| repo_paths.len() == expected_num)
    .map(|(short_name, _)| short_name)
    .collect::<Vec<String>>();

  Some(shared)
}
