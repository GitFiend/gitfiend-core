use crate::git::git_version::GitVersion;
use crate::git::run_git_action::{run_git_action, RunGitActionOptions};
use crate::git::store::GIT_VERSION;
use crate::server::git_request::ReqOptions;

pub fn stash_changes(options: &ReqOptions) -> u32 {
  run_git_action(RunGitActionOptions {
    git_version: GIT_VERSION.get().unwrap_or_else(GitVersion::new),
    repo_path: &options.repo_path,
    commands: [vec!["add", "--all"], vec!["stash", "push"]],
  })
}
