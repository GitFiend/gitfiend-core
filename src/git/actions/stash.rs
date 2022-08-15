use crate::git::git_version::GitVersion;
use crate::git::run_git_action::{run_git_action, RunGitActionOptions};
use crate::git::store::GIT_VERSION;
use crate::server::git_request::ReqOptions;
use serde::Deserialize;
use ts_rs::TS;

pub fn stash_changes(options: &ReqOptions) -> u32 {
  run_git_action(RunGitActionOptions {
    git_version: GIT_VERSION.get().unwrap_or_else(GitVersion::new),
    repo_path: &options.repo_path,
    commands: [vec!["add", "--all"], vec!["stash", "push"]],
  })
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct StashStagedOptions {
  pub repo_path: String,
  pub head_commit_id: String,
}

pub fn stash_staged(options: &StashStagedOptions) -> u32 {
  let StashStagedOptions {
    repo_path,
    head_commit_id,
  } = options;

  run_git_action(RunGitActionOptions {
    git_version: GIT_VERSION.get().unwrap_or_else(GitVersion::new),
    repo_path,
    commands: [
      vec![
        "commit",
        "-m",
        "TEMP_COMMIT: If you are seeing this commit there has been an error while stashing.",
      ],
      vec!["add", "--all"],
      vec!["stash", "push"],
      vec!["reset", "--soft", head_commit_id],
      vec!["add", "--all"],
      vec!["stash", "push"],
      vec!["stash", "pop", "stash@{1}"],
      vec!["reset"],
    ],
  })
}
