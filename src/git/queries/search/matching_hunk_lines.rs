use std::time::Duration;

use crate::git::git_types::{Commit, Hunk, HunkLine, HunkLineStatus, Patch};
use crate::git::queries::hunks::hunk_parsers::P_HUNKS;
use crate::git::queries::hunks::load_hunks::load_hunks_args;
use crate::git::run_git::{run_git_err, RunGitOptions};
use crate::global;
use crate::parser::parse_all_err;
use crate::server::request_util::R;
use crate::util::global::Global;
use crate::util::short_cache::ShortCache;

static SHORT_HUNK_CACHE: Global<ShortCache<Vec<Hunk>>> = global!(ShortCache::new(
  "Hunk Cache".to_string(),
  Duration::from_secs(10)
));

// This should match "gSearchResultDiff"
pub fn get_matching_hunk_lines(
  repo_path: &String,
  commit: &Commit,
  patch: &Patch,
  search_text: &str,
) -> R<Vec<HunkLine>> {
  let cache_id = format!("{}{}", commit.id, patch.id);

  if let Some(hunks) = get_hunks_from_cache(&cache_id) {
    return Ok(get_matching_lines_in_hunks(hunks, search_text));
  }

  let out = run_git_err(RunGitOptions {
    repo_path,
    args: load_hunks_args(commit, patch),
  })?;

  let hunks = parse_all_err(P_HUNKS, &out.stdout)?;
  store_hunk_in_cache(&cache_id, hunks.clone());

  let hunk_lines = get_matching_lines_in_hunks(hunks, search_text);

  Ok(hunk_lines)
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

      if (*status == HunkLineStatus::Added || *status == HunkLineStatus::Removed)
        && text.contains(search_text)
      {
        hunk_lines.push(line);
      }
    }
  }

  hunk_lines
}
