use serde::Deserialize;
use ts_rs::TS;

use crate::git::git_version::GitVersion;
use crate::git::run_git_action::{run_git_action, RunGitActionOptions};
use crate::git::store::GIT_VERSION;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CommandOptions {
  pub repo_path: String,
  pub args: Vec<String>,
}

pub fn command(options: &CommandOptions) -> u32 {
  run_git_action(RunGitActionOptions {
    repo_path: &options.repo_path,
    git_version: GIT_VERSION.get().unwrap_or_else(GitVersion::new),
    commands: [options
      .args
      .iter()
      .map(|a| a.as_str())
      .collect::<Vec<&str>>()],
  })
}
