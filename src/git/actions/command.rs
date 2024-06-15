use crate::git::repo_watcher::mark_changed;
use serde::Deserialize;
use ts_rs::TS;

use crate::git::run_git_action::{run_git_action, RunGitActionOptions};

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CommandOptions {
  pub repo_path: String,
  pub args: Vec<String>,
}

pub fn command(options: &CommandOptions) -> u32 {
  mark_changed(&options.repo_path);

  run_git_action(RunGitActionOptions {
    repo_path: &options.repo_path,
    commands: [options
      .args
      .iter()
      .map(|a| a.as_str())
      .collect::<Vec<&str>>()],
  })
}

// #[derive(Debug, Deserialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export)]
// pub struct CommandsOptions {
//   pub repo_path: String,
//   pub commands: Vec<Vec<String>>,
// }
