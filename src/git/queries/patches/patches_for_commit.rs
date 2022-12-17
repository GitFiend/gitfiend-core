use crate::git::git_types::Patch;
use crate::git::queries::patches::patches::load_patches;
use crate::git::store;
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

  let (commits, _) = store::get_commits_and_refs(repo_path)?;
  let all_patches = load_patches(repo_path, &commits)?;

  Some(all_patches.get(commit_id)?.clone())
}
