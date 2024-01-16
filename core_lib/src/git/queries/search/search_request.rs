use std::thread;
use std::time::Instant;

use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::git::queries::search::get_next_search_id;
use crate::git::queries::search::search_code::{
  search_commits_for_code, CodeSearchOpts, FileMatch,
};
use crate::global;
use crate::util::global::Global;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DiffSearch {
  pub repo_path: String,
  pub search_text: String,
  pub search_id: u32,
  pub search_result: Option<Vec<(String, Vec<FileMatch>)>>,
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

static DIFF_SEARCHES: Global<AHashMap<u32, DiffSearch>> = global!(AHashMap::new());

/*
This begins a search and returns the search_id. We return before completing so
we don't block the server, and new searches can cancel the stale ones.
 */
pub fn start_diff_search(options: &CodeSearchOpts) -> u32 {
  let CodeSearchOpts {
    repo_path,
    search_text,
    ..
  } = options.clone();

  let search = DiffSearch::new(repo_path, search_text, get_next_search_id());

  DIFF_SEARCHES.insert(search.search_id, search.clone());

  let o = options.clone();

  thread::spawn(move || {
    let result = search_commits_for_code(&o, search.search_id);

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
  pub results: Option<Vec<(/*commit_id*/ String, Vec<FileMatch>)>>,
}

pub fn poll_diff_search(options: &PollSearchOpts) -> PollSearchResult {
  if let Some(result) = poll_diff_search_inner(options) {
    return result;
  }

  // Search not found. We should return complete?
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

// Be careful with this as all searches my be completed but client has polled and gotten the
// last result yet.
pub fn clear_completed_searches() {
  if let Ok(mut searches) = DIFF_SEARCHES.data.write() {
    (*searches) = (*searches)
      .clone()
      .into_iter()
      .filter(|search| !search.1.completed)
      .collect();
  }
}
