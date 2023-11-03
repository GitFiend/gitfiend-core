use crate::git::git_types::Patch;
use crate::git::queries::patches::patches::load_patches;
use crate::git::store::STORE;
use crate::server::request_util::{ES, R};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqPatchesForCommitOpts {
  pub repo_path: String,
  pub commit_id: String,
}

pub fn load_patches_for_commit(options: &ReqPatchesForCommitOpts) -> R<Vec<Patch>> {
  let ReqPatchesForCommitOpts {
    repo_path,
    commit_id,
  } = options;

  let (commits, _) = STORE
    .get_commits_and_refs(repo_path)
    .ok_or(ES::from("load_patches_for_commit: Couldn't get commits."))?;

  let all_patches = load_patches(repo_path, &commits)?;

  Ok(
    all_patches
      .get(commit_id)
      .ok_or(ES::from(
        "load_patches_for_commit: Missing patches for commit.",
      ))?
      .clone(),
  )
}
