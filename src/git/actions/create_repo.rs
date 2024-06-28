use std::fs;

use crate::dprintln;
use crate::git::run_git_action::{run_git_action, RunGitActionOptions};
use crate::server::git_request::ReqOptions;

pub fn create_repo(options: &ReqOptions) -> u32 {
  let ReqOptions { repo_path } = options;

  if let Err(_e) = fs::create_dir_all(repo_path) {
    dprintln!("{:?}", _e);
    return 0;
  }

  run_git_action(RunGitActionOptions {
    repo_path,
    commands: [vec!["init"]],
  })
}
