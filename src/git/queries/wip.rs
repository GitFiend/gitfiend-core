mod create_hunks;
pub(crate) mod wip_diff;
mod wip_patch_parsers;
pub(crate) mod wip_patches;
use crate::server::git_request::ReqOptions;
use std::fs;
use std::path::Path;

pub fn is_rebase_in_progress(options: &ReqOptions) -> bool {
  Path::new(&options.repo_path)
    .join(".git")
    .join("rebase-merge")
    .exists()
}

// Returns the commit id of the branch we tried to merge
// into our current if we have a conflict.
pub fn is_merge_in_progress(options: &ReqOptions) -> Option<String> {
  let ReqOptions { repo_path } = options;

  read_merge_head(repo_path)

  // let out = run_git::run_git(RunGitOptions {
  //   repo_path,
  //   args: ["merge", "HEAD"],
  // });
  //
  // // An empty repo will fail so try another thing?
  // if out.is_none() {
  //   let out = run_git::run_git(RunGitOptions {
  //     repo_path,
  //     args: ["log", "-n", "1"],
  //   });
  //
  //   if out.is_none() {
  //     return false;
  //   }
  //   return true;
  // }
  //
  // false
}

fn read_merge_head(repo_path: &str) -> Option<String> {
  let text = fs::read_to_string(Path::new(repo_path).join(".git").join("MERGE_HEAD")).ok()?;

  Some(text.trim().to_string())
}
