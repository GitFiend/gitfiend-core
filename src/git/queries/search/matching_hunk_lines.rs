use crate::git::git_types::{Commit, HunkLine, HunkRange, Patch};
use crate::global2;
use crate::util::global2::Global2;
use ahash::AHashMap;
use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct LinesReqOpts {
  pub repo_path: String,
  pub commit: Commit,
  pub patch: Patch,
  pub search_text: String,
  pub num_results: usize,
}

const SHORT_PATCH_CACHE: Global2<AHashMap<String, Patch>> = global2!(AHashMap::new());

// This should match "gSearchResultDiff", accidentally started wrong one.
pub fn get_matching_hunk_lines(options: &LinesReqOpts) {
  let LinesReqOpts {
    repo_path,
    commit,
    patch,
    search_text,
    num_results,
  } = options;

  let cache_id = format!("{}{}{}", search_text, repo_path, num_results);

  if let Some(cached) = SHORT_PATCH_CACHE.get_by_key(&cache_id) {
    //
  }
}
