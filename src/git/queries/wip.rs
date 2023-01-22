mod create_hunks;
pub(crate) mod wip_diff;
mod wip_patch_parsers;
pub(crate) mod wip_patches;
use crate::server::git_request::ReqOptions;
use std::fs;
use std::path::Path;

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
