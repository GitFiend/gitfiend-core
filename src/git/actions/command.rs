use crate::git::git_version::GitVersion;
use crate::git::run_git_action::{run_git_action3, RunGitActionOptions2};
use crate::git::store::GIT_VERSION;
use crate::server::git_request::ActionOptions;

pub fn command(options: &ActionOptions) -> u32 {
  run_git_action3(RunGitActionOptions2 {
    repo_path: &options.repo_path,
    git_version: GIT_VERSION.get().unwrap_or_else(GitVersion::new),
    commands: [options
      .args
      .iter()
      .map(|a| a.as_str())
      .collect::<Vec<&str>>()],
  })
}
