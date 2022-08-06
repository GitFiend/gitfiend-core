use crate::git::git_version::GitVersion;
use crate::git::run_git_action::{run_git_action, ActionError, ActionOutput, RunGitActionOptions};
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

pub fn clone_repo(options: &CloneOptions) -> Result<ActionOutput, ActionError> {
  let out = run_git_action(RunGitActionOptions {
    git_version: GIT_VERSION.get().unwrap_or_else(GitVersion::new),
    repo_path: &options.repo_path,
    args: ["clone", "--progress", &options.url],
  });

  // print_action_result(out);

  println!("{:?}", out);

  out
}
