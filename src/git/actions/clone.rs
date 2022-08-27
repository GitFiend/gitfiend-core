use serde::Deserialize;
use ts_rs::TS;

use crate::dprintln;
use crate::git::run_git_action::{run_git_action, RunGitActionOptions};

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CloneOptions {
  // Dir to clone into.
  pub repo_path: String,
  pub url: String,
}

pub fn clone_repo(options: &CloneOptions) -> u32 {
  let out = run_git_action(RunGitActionOptions {
    repo_path: &options.repo_path,
    commands: [vec!["clone", "--progress", &options.url]],
  });

  // print_action_result(out);

  dprintln!("{:?}", out);

  out
}
