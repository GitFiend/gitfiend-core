use crate::git::run_git::{run_git, RunGitOptions};
use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RunOptions {
  pub repo_path: String,
  pub args: Vec<String>,
}

pub fn run(options: &RunOptions) -> Option<String> {
  run_git(RunGitOptions {
    repo_path: &options.repo_path,
    args: &options.args,
  })
}
