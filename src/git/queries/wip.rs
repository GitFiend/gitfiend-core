mod wip_patch_parsers;
mod wip_patches;

use crate::git::{run_git, RunGitOptions};
use crate::server::git_request::ReqOptions;

pub fn is_merge_in_progress(options: &ReqOptions) -> bool {
  let ReqOptions { repo_path } = options;

  let out = run_git(RunGitOptions {
    repo_path: &repo_path,
    args: ["merge", "HEAD"],
  });

  if out.is_none() {
    let out = run_git(RunGitOptions {
      repo_path: &repo_path,
      args: ["log", "-n", "1"],
    });

    if out.is_none() {
      return false;
    }
    return true;
  }

  false
}
