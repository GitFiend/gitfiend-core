use std::time::Duration;

use serde::Deserialize;
use ts_rs::TS;

use crate::git::git_types::{Commit, Hunk, HunkLine, HunkLineStatus, Patch};
use crate::git::queries::hunks::hunk_parsers::P_HUNKS;
use crate::git::queries::hunks::hunks::{flatten_hunks, load_hunks_args};
use crate::git::{run_git, RunGitOptions};
use crate::global;
use crate::parser::parse_all;
use crate::util::global::Global;
use crate::util::short_cache::ShortCache;

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

static SHORT_HUNK_CACHE: Global<ShortCache<Vec<Hunk>>> = global!(ShortCache::new(
  "Hunk Cache".to_string(),
  Duration::from_secs(10)
));

// This should match "gSearchResultDiff"
pub fn get_matching_hunk_lines(options: &LinesReqOpts) -> Option<Vec<HunkLine>> {
  let LinesReqOpts {
    repo_path,
    commit,
    patch,
    search_text,
    ..
  } = options;

  let cache_id = format!("{}{}", commit.id, patch.id);

  if let Some(hunks) = get_hunks_from_cache(&cache_id) {
    return Some(get_matching_lines_in_hunks(hunks, search_text));
  }

  if let Some(out) = run_git(RunGitOptions {
    repo_path,
    args: load_hunks_args(commit, patch),
  }) {
    let hunks = parse_all(P_HUNKS, &out)?;
    store_hunk_in_cache(&cache_id, hunks.clone());

    let hunk_lines = flatten_hunks(hunks);

    return Some(hunk_lines);
  }

  None
}

fn get_hunks_from_cache(key: &str) -> Option<Vec<Hunk>> {
  if let Some(mut cached) = SHORT_HUNK_CACHE.get() {
    return Some(cached.get(key)?.clone());
  }
  None
}

fn store_hunk_in_cache(key: &str, hunks: Vec<Hunk>) {
  if let Some(mut cache) = SHORT_HUNK_CACHE.get() {
    cache.insert(key, hunks);
  }
}

fn get_matching_lines_in_hunks(hunks: Vec<Hunk>, search_text: &str) -> Vec<HunkLine> {
  let mut hunk_lines: Vec<HunkLine> = Vec::new();

  for hunk in hunks {
    for line in hunk.lines {
      let HunkLine { status, text, .. } = &line;

      if *status == HunkLineStatus::Added
        || *status == HunkLineStatus::Removed && text.contains(search_text)
      {
        hunk_lines.push(line);
      }
    }
  }

  hunk_lines
}
