use std::thread;
use std::time::Instant;

use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::git::git_types::Patch;
use crate::git::queries::search::{get_next_search_id, search_diffs_with_id, SearchOptions};
use crate::git::store::RwStore;
use crate::global2;
use crate::util::global2::Global2;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DiffSearch {
  pub repo_path: String,
  pub search_text: String,
  pub search_id: u32,
  pub search_result: Option<Vec<(String, Vec<Patch>)>>,
  pub time: Instant,
  pub completed: bool,
}

impl DiffSearch {
  fn new(repo_path: String, search_text: String, search_id: u32) -> Self {
    Self {
      repo_path,
      search_text,
      search_id,
      search_result: None,
      time: Instant::now(),
      completed: false,
    }
  }
}

static DIFF_SEARCHES: Global2<AHashMap<u32, DiffSearch>> = global2!(AHashMap::new());

/*
This begins a search and returns the search_id. We return before completing so
we don't block the server, and new searches can cancel the stale ones.
 */
pub fn start_diff_search(options: &SearchOptions, _: RwStore) -> u32 {
  let SearchOptions {
    repo_path,
    search_text,
    ..
  } = options.clone();

  let search = DiffSearch::new(repo_path, search_text, get_next_search_id());

  DIFF_SEARCHES.insert(search.search_id, search.clone());

  let o = options.clone();

  thread::spawn(move || {
    let result = search_diffs_with_id(&o, search.search_id.clone());

    if let Some(searches) = DIFF_SEARCHES.get() {
      if let Some(initial_search) = searches.get(&search.search_id) {
        let mut updated_search = initial_search.clone();

        updated_search.search_result = result;
        // This needs to be set regardless of whether we get a result.
        updated_search.completed = true;

        DIFF_SEARCHES.insert(updated_search.search_id, updated_search);
      }
    }
  });

  search.search_id
}

#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PollSearchOpts {
  pub search_id: u32,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PollSearchResult {
  pub search_id: u32,
  pub complete: bool,
  pub results: Option<Vec<(String, Vec<Patch>)>>,
}

// TODO: Clean up DIFF_SEARCHES.
pub fn poll_diff_search(options: &PollSearchOpts, _: RwStore) -> PollSearchResult {
  if let Some(result) = poll_diff_search_inner(options) {
    return result;
  }

  PollSearchResult {
    search_id: options.search_id,
    complete: false,
    results: None,
  }
}

fn poll_diff_search_inner(options: &PollSearchOpts) -> Option<PollSearchResult> {
  let searches = DIFF_SEARCHES.get()?;
  let search = searches.get(&options.search_id)?;

  if search.completed {
    return Some(PollSearchResult {
      search_id: options.search_id,
      complete: true,
      results: search.search_result.clone(),
    });
  }

  None
}
