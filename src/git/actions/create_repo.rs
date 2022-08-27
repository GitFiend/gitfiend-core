use std::fs;

use crate::dprintln;
use crate::git::git_version::GitVersion;
use crate::git::run_git_action::{run_git_action, RunGitActionOptions};
use crate::git::store::GIT_VERSION;
use crate::server::git_request::ReqOptions;

pub fn create_repo(options: &ReqOptions) -> u32 {
  let ReqOptions { repo_path } = options;

  if let Err(e) = fs::create_dir_all(&repo_path) {
    dprintln!("{:?}", e);
    return 0;
  }

  run_git_action(RunGitActionOptions {
    git_version: GIT_VERSION.get().unwrap_or_else(GitVersion::new),
    repo_path,
    commands: [vec!["init"]],
  })
}
