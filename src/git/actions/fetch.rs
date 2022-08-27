use crate::git::run_git_action::{run_git_action, RunGitActionOptions};
use crate::server::git_request::ReqOptions;

pub fn fetch_all(options: &ReqOptions) -> u32 {
  run_git_action(RunGitActionOptions {
    repo_path: &options.repo_path,
    commands: [vec!["fetch", "--all", "--prune"]],
  })
}
