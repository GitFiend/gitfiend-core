use std::fs;

use crate::dprintln;
use crate::git::git_request::ReqOptions;
use crate::git::run_git_action::{run_git_action, RunGitActionOptions};

pub fn create_repo(options: &ReqOptions) -> u32 {
  let ReqOptions { repo_path } = options;

  if let Err(e) = fs::create_dir_all(repo_path) {
    dprintln!("{:?}", e);
    return 0;
  }

  run_git_action(RunGitActionOptions {
    repo_path,
    commands: [vec!["init"]],
  })
}
