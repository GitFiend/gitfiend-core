use crate::git::git_version::GitVersion;
use crate::git::run_git_action::{run_git_action3, RunGitActionOptions2};
use crate::git::store::GIT_VERSION;
use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CloneOptions {
  // Dir to clone into.
  pub repo_path: String,
  pub url: String,
}

pub fn clone_repo(options: &CloneOptions) -> u32 {
  let out = run_git_action3(RunGitActionOptions2 {
    git_version: GIT_VERSION.get().unwrap_or_else(GitVersion::new),
    repo_path: &options.repo_path,
    commands: [vec!["clone", "--progress", &options.url]],
  });

  // print_action_result(out);

  println!("{:?}", out);

  out
}
