use crate::git::run_git_action::{print_action_result, run_git_action, RunGitActionOptions};
use crate::git::store::GIT_VERSION;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CloneOptions {
  // Dir to clone into.
  pub repo_path: String,
  pub url: String,
}

pub fn clone_repo(options: &CloneOptions) -> Option<()> {
  let out = run_git_action(RunGitActionOptions {
    git_version: GIT_VERSION.get()?,
    repo_path: &options.repo_path,
    args: ["clone", "--progress", &options.url],
  });

  print_action_result(out);

  Some(())
}
