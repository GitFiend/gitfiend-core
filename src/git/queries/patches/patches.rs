use crate::git::git_types::{Commit, Patch};
use crate::git::queries::patches::cache::{load_patches_cache, write_patches_cache};
use crate::git::queries::patches::patch_parsers::P_MANY_PATCHES_WITH_COMMIT_IDS;
use crate::git::queries::store::load_commits_from_store;
use crate::git::{run_git, RunGitOptions};
use crate::load_commits_and_stashes;
use crate::parser::parse_all;
use crate::server::git_request::ReqCommitsOptions;
use std::collections::HashMap;

pub fn load_patches(options: &ReqCommitsOptions) -> Option<HashMap<String, Vec<Patch>>> {
  let ReqCommitsOptions { repo_path, .. } = options;

  let commits =
    load_commits_from_store(&repo_path).or_else(|| load_commits_and_stashes(options))?;

  let mut commits_without_patches: Vec<&Commit> = Vec::new();
  let mut stashes_or_merges_without_patches: Vec<&Commit> = Vec::new();

  let mut new_patches: HashMap<String, Vec<Patch>> = HashMap::new();

  if let Some(patches) = load_patches_cache(&repo_path) {
    for c in commits.iter() {
      if let Some(patch) = patches.get(&c.id) {
        new_patches.insert(c.id.clone(), patch.clone());
      } else {
        if c.stash_id.is_none() && !c.is_merge {
          commits_without_patches.push(c);
        } else {
          stashes_or_merges_without_patches.push(c);
        }
      }
    }
  }

  if commits_without_patches.len() > 0 {
    if let Some(patches) = load_normal_patches(&commits_without_patches, &options) {
      new_patches.extend(patches);
    }
  }

  write_patches_cache(&repo_path, &new_patches);

  Some(new_patches)
}

// TODO: This isn't tested.
fn load_normal_patches(
  commits_without_patches: &Vec<&Commit>,
  options: &ReqCommitsOptions,
) -> Option<HashMap<String, Vec<Patch>>> {
  if commits_without_patches.len() > 20 {
    // Assume we now have all the plain commits.
    load_all_patches_for_normal_commits(&options)
  } else {
    // We can't handle many commit ids with this command.
    let mut ids: Vec<&str> = commits_without_patches
      .iter()
      .map(|c| c.id.as_str())
      .collect();

    ids.insert(0, "show");
    ids.extend(["--name-status", "--pretty=format:%H,", "-z"]);

    let out = run_git(RunGitOptions {
      repo_path: &options.repo_path,
      args: ids,
    })?;

    let commit_patches = parse_all(P_MANY_PATCHES_WITH_COMMIT_IDS, &out)?;

    // let mut map = HashMap::new();

    // for (id, patches) in commit_patches.into_iter() {
    //   map.insert(id, patches);
    // }

    // map.extend(commit_patches);
    // commit_patches.into_iter().collect();

    // I think we can do this to convert the vec to hashmap?
    Some(commit_patches.into_iter().collect())
  }
}

/// This doesn't include stashes and merges.
fn load_all_patches_for_normal_commits(
  options: &ReqCommitsOptions,
) -> Option<HashMap<String, Vec<Patch>>> {
  let out = run_git(RunGitOptions {
    args: [
      "log",
      "--remotes",
      "--name-status",
      "--pretty=format:%H,",
      // Can't get correct patch info for merges with this command.
      "--no-merges",
      "-z",
      &format!("-n{}", options.num_commits),
    ],
    repo_path: &options.repo_path,
  })?;

  let commit_patches = parse_all(P_MANY_PATCHES_WITH_COMMIT_IDS, &out)?;

  // let mut map = HashMap::new();
  //
  // for (id, patches) in commit_patches.into_iter() {
  //   map.insert(id, patches);
  // }
  //
  // Some(map)

  // I think we can do this to convert the vec to hashmap?
  Some(commit_patches.into_iter().collect())
}
