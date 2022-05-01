use crate::git::git_types::{Commit, Patch};
use crate::git::queries::COMMIT_0_ID;
use crate::git::{run_git, RunGitOptions};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqHunkOptions {
  pub repo_path: String,
  pub commit: Commit,
  pub patch: Patch,
}

pub fn load_hunk_lines(options: ReqHunkOptions) {
  let out = run_git(RunGitOptions {
    repo_path: &options.repo_path,
    args: load_hunks_args(&options),
  });

  //
}

pub fn load_hunks_args(options: &ReqHunkOptions) -> [String; 4] {
  let diff = "diff".to_string();
  let dashes = "--".to_string();

  let ReqHunkOptions { commit, patch, .. } = options;
  let old_file = patch.old_file.clone();

  let Commit {
    id,
    parent_ids,
    is_merge,
    ..
  } = commit;

  if *is_merge {
    [diff, format!("{}^@", id), dashes, old_file]
  } else if commit.parent_ids.len() > 0 {
    [diff, format!("{}..{}", parent_ids[0], id), dashes, old_file]
  } else {
    [diff, format!("{}..{}", COMMIT_0_ID, id), dashes, old_file]
  }
}
