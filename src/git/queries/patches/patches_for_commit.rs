use crate::git::git_types::Patch;
use crate::git::queries::patches::cache::load_patches_cache;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqPatchesForCommitOpts {
  pub repo_path: String,
  pub commit_id: String,
}

pub fn load_patches_for_commit(options: &ReqPatchesForCommitOpts) -> Option<Vec<Patch>> {
  let ReqPatchesForCommitOpts {
    repo_path,
    commit_id,
  } = options;

  Some(load_patches_cache(repo_path)?.get(commit_id)?.clone())
}
