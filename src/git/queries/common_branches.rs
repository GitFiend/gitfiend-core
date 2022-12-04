use crate::git::git_types::{Commit, RefInfo, RefLocation, RefType};
use crate::git::queries::refs::get_ref_info_from_commits;
use crate::git::store::{get_all_workspace_commits, RepoPath};
use crate::server::git_request::ReqOptions;
use ahash::{AHashMap, AHashSet};

type RefShortName = String;

pub fn get_common_branches(_: &ReqOptions) -> Option<Vec<RefShortName>> {
  let repos: AHashMap<RepoPath, Vec<Commit>> = get_all_workspace_commits()?;

  let mut counts: AHashMap<RefShortName, AHashSet<RepoPath>> = AHashMap::new();

  let expected_num = repos.len();
  let data: Vec<(String, Vec<RefInfo>)> = repos
    .into_iter()
    .map(|(repo_name, commits)| (repo_name, get_ref_info_from_commits(&commits)))
    .collect();

  for (repo_path, refs) in &data {
    for RefInfo {
      short_name,
      ref_type,
      ..
    } in refs
    {
      if *ref_type == RefType::Branch {
        if let Some(count) = counts.get_mut(short_name) {
          count.insert(repo_path.clone());
        } else {
          let mut count = AHashSet::new();
          count.insert(repo_path.clone());
          counts.insert(short_name.clone(), count);
        }
      }
    }
  }

  let shared = counts
    .into_iter()
    .filter(|(_, repo_paths)| repo_paths.len() == expected_num)
    .map(|(short_name, _)| short_name)
    .collect::<Vec<RefShortName>>();

  calc_diffs(&shared, &data);

  Some(shared)
}

fn calc_diffs(
  ref_names: &[RefShortName],
  all_repo_refs: &[(RepoPath, Vec<RefInfo>)],
) -> Option<()> {
  let head_names = all_repo_refs
    .iter()
    .flat_map(|(_, refs)| refs.iter().find(|r| r.head))
    .map(|head| &head.short_name)
    .collect::<AHashSet<_>>();

  if head_names.len() != 1 {
    return None;
  }

  let head_name = head_names.iter().next()?.to_string();

  println!("all_on_same_branch \"{head_name}\"");

  // all_repo_refs
  //   .iter()
  //   .map(|(repo_path, refs)| ref_names.iter().map(|name| refs.iter()));

  Some(())
}

// TODO: What about other remotes?
fn calc_diffs_for_repo(ref_name: &RefShortName, head_name: &RefShortName, refs: &Vec<RefInfo>) {
  let mut head_ref: Option<&RefInfo> = None;
  let mut local_ref: Option<&RefInfo> = None;
  let mut remote_ref: Option<&RefInfo> = None;

  for r in refs {
    if head_ref.is_none() && r.short_name == *head_name {
      head_ref = Some(r);
    }
    if r.short_name == *ref_name {
      if (r.location == RefLocation::Local) {
        //
      }
    }
  }
}
