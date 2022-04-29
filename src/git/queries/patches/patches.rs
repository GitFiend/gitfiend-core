use crate::git::git_types::{Commit, Patch};
use crate::git::queries::patches::cache::load_patches_cache;
use crate::git::queries::patches::patch_parsers::P_MANY_PATCHES_WITH_COMMIT_IDS;
use crate::git::{run_git, RunGitOptions};
use crate::parser::parse_all;
use crate::server::git_request::ReqCommitsOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

pub fn load_all_commit_patches(options: &ReqCommitsOptions) -> Option<HashMap<String, Vec<Patch>>> {
  load_patches_cache(&options.repo_path);

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

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqPatchesOptions {
  pub repo_path: String,
  pub commits: Vec<Commit>,
}

pub fn load_patches(options: &ReqPatchesOptions) {
  //
}

// pub fn load_missing_patches_for_commits
