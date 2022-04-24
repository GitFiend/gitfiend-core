use crate::git::git_types::Patch;
use crate::git::queries::patches::patch_parsers::P_MANY_PATCHES_WITH_COMMIT_IDS;
use crate::git::{run_git, RunGitOptions};
use crate::parser::parse_all;
use crate::server::git_request::ReqCommitsOptions;
use std::collections::HashMap;

pub fn load_all_commit_patches(options: &ReqCommitsOptions) -> Option<HashMap<String, Vec<Patch>>> {
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
