use crate::git::git_types::{Commit, Patch};
use crate::git::queries::patches::cache::{load_patches_cache, write_patches_cache};
use crate::git::queries::patches::patch_parsers::P_MANY_PATCHES_WITH_COMMIT_IDS;
use crate::git::queries::store::load_commits_from_store;
use crate::git::{run_git, RunGitOptions};
use crate::parser::parse_all;
use crate::server::git_request::ReqCommitsOptions;
use std::collections::HashMap;

fn load_patches_for_commits(options: &ReqCommitsOptions) -> Option<HashMap<String, Vec<Patch>>> {
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

  let mut map = HashMap::new();

  for (id, patches) in commit_patches.into_iter() {
    map.insert(id, patches);
  }

  Some(map)
}

pub fn load_patches(options: &ReqCommitsOptions) -> Option<HashMap<String, Vec<Patch>>> {
  let ReqCommitsOptions { repo_path, .. } = options;

  // What if we have no commits? Should always be ready at this point.
  let commits = load_commits_from_store(&repo_path)?;

  let mut commits_without_patches: Vec<&Commit> = Vec::new();

  if let Some(patches) = load_patches_cache(&repo_path) {
    for c in commits.iter() {
      if !patches.contains_key(&c.id) {
        commits_without_patches.push(c);
      }
    }
  }

  if commits_without_patches.len() > 0 {
    //
  }

  let patches = load_patches_for_commits(&options)?;

  write_patches_cache(&repo_path, &patches);

  Some(patches)
}
