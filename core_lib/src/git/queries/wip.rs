mod create_hunks;
pub mod wip_diff;
mod wip_patch_parsers;
pub mod wip_patches;
use crate::git::git_request::ReqOptions;
use crate::git::store::STORE;
use std::fs::read_to_string;

pub fn is_rebase_in_progress(options: &ReqOptions) -> bool {
  match STORE.get_repo_path(&options.repo_path) {
    Ok(path) => path.git_path.join("rebase-merge").exists(),
    Err(..) => false,
  }
}

// // Returns the commit id of the branch we tried to merge
// // into our current if we have a conflict.
// pub fn is_merge_in_progress(options: &ReqOptions) -> Option<String> {
//   let ReqOptions { repo_path } = options;
//
//   read_merge_head(repo_path)
// }

pub fn read_merge_head(repo_path: &str) -> Option<String> {
  let path = STORE.get_repo_path(repo_path).ok()?;

  if let Ok(text) = read_to_string(path.git_path.join("MERGE_HEAD")) {
    return Some(text.trim().to_string());
  }

  // This seems to happen when there's a conflict from un-stashing. Returns "special ref".
  if let Ok(text) = read_to_string(path.git_path.join("AUTO_MERGE")) {
    return Some(text.trim().to_string());
  }

  None
}
