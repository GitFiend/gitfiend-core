use serde::{Deserialize, Serialize};
use std::fs;
use ts_rs::TS;

use crate::git::run_git::{run_git_buffer, RunGitOptions};

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqImageOptions {
  pub repo_path: String,
  pub commit_id: String,
  pub original_image_path: String,
  pub temp_image_path: String,
}

pub fn load_commit_image(options: &ReqImageOptions) -> bool {
  let ReqImageOptions {
    repo_path,
    commit_id,
    original_image_path,
    temp_image_path,
  } = options;

  if let Some(buffer) = run_git_buffer(RunGitOptions {
    repo_path,
    args: ["show", &format!("{}:{}", commit_id, original_image_path)],
  }) {
    return fs::write(temp_image_path, buffer).is_ok();
  }
  false
}
