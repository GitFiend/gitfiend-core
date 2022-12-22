use serde::Deserialize;
use std::fs::create_dir_all;
use ts_rs::TS;

use crate::dprintln;
use crate::git::run_git_action::{run_git_action, RunGitActionOptions};
use crate::git::store::get_git_version;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CloneOptions {
  // Dir to clone into.
  pub repo_path: String,
  pub url: String,
}

pub fn clone_repo(options: &CloneOptions) -> u32 {
  if create_dir_all(&options.repo_path).is_err() {
    return 0;
  }

  let version = get_git_version();

  let command = if version.major > 1 && version.minor > 12 {
    vec!["clone", "--recurse-submodules", "--progress", &options.url]
  } else {
    vec!["clone", "--recursive", "--progress", &options.url]
  };

  let out = run_git_action(RunGitActionOptions {
    repo_path: &options.repo_path,
    commands: [command],
  });

  dprintln!("{:?}", out);

  out
}
