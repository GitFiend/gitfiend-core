use crate::util::global::Global;
use crate::{dprintln, global};
use serde::Deserialize;
use ts_rs::TS;

pub mod matching_hunk_lines;
pub mod search_code;
pub mod search_commits;
pub mod search_request;

#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SearchOptions {
  pub repo_path: String,
  pub search_text: String,
  pub num_results: usize,
}

static CURRENT_SEARCH: Global<u32> = global!(0);

pub fn get_next_search_id() -> u32 {
  if let Some(id) = CURRENT_SEARCH.get() {
    let new_id = id + 1;
    CURRENT_SEARCH.set(new_id);
    new_id
  } else {
    CURRENT_SEARCH.set(0);
    0
  }
}

fn search_cancelled(search_id: u32) -> bool {
  if let Some(id) = CURRENT_SEARCH.get() {
    dprintln!("current: {id}, this: {search_id}");
    search_id != id
  } else {
    false
  }
}

#[cfg(test)]
mod tests {
  use crate::git::git_types::Patch;
  use crate::git::queries::patches::patch_parsers::P_MANY_PATCHES_WITH_COMMIT_IDS;
  use std::time::{Duration, Instant};
  use std::{assert_eq, println, thread};

  use crate::git::queries::search::get_next_search_id;
  use crate::git::queries::search::search_code::{search_code_command, CodeSearchOpts};
  use crate::parser::parse_all;

  #[test]
  fn test_get_next_search_id() {
    assert_eq!(get_next_search_id(), 1);
    assert_eq!(get_next_search_id(), 2);
    assert_eq!(get_next_search_id(), 3);

    let now = Instant::now();

    while get_next_search_id() < 1_000 {}

    println!("Took {}us", now.elapsed().as_micros());
  }

  // We can't run tests in parallel as they will be killed.
  /*
  TODO: Write better tests and re-enable.

  These tests where written to get our search working, but depend on the repo being in a certain
  state so need to be disabled.

   */
  #[ignore]
  #[test]
  fn test_thing() {
    let t1 = thread::spawn(move || {
      search_diffs(&CodeSearchOpts {
        num_results: 500,
        search_text: "this".to_string(),
        repo_path: ".".to_string(),
        start_commit_index: 0,
      })
    });

    thread::sleep(Duration::from_millis(10));

    let t2 = thread::spawn(move || {
      search_diffs(&CodeSearchOpts {
        num_results: 500,
        search_text: "this".to_string(),
        repo_path: ".".to_string(),
        start_commit_index: 0,
      })
    });

    thread::sleep(Duration::from_millis(10));

    let t3 = thread::spawn(move || {
      search_diffs(&CodeSearchOpts {
        num_results: 5,
        search_text: "this".to_string(),
        repo_path: ".".to_string(),
        start_commit_index: 0,
      })
    });

    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();
    let r3 = t3.join().unwrap();

    println!("{:?}, {:?}, {:?}", r1, r2, r3);

    assert!(r1.is_none());
    assert!(r2.is_none());
    assert!(r3.is_some());
  }

  pub fn search_diffs(options: &CodeSearchOpts) -> Option<Vec<(String, Vec<Patch>)>> {
    let search_id = get_next_search_id();
    let result = search_code_command(options, search_id)?;

    parse_all(P_MANY_PATCHES_WITH_COMMIT_IDS, &result)
  }
}
