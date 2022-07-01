use std::process::{Output, Stdio};
use std::{thread, time};

use async_process::Command;
use futures::executor;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::git::git_types::{HunkLine, Patch};
use crate::git::queries::commits::COMMITS;
use crate::git::queries::patches::patch_parsers::P_MANY_PATCHES_WITH_COMMIT_IDS;
use crate::git::queries::search::matching_hunk_lines::get_matching_hunk_lines;
use crate::global;
use crate::parser::parse_all;
use crate::util::global::Global;

mod commit_search;
pub(crate) mod matching_hunk_lines;
pub(crate) mod search;
mod search_refs;
pub(crate) mod search_request;

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
    println!("current: {id}, this: {search_id}");
    search_id != id
  } else {
    false
  }
}

// TODO: Deprecate this.
pub fn _search_diffs(options: &SearchOptions) -> Option<Vec<(String, Vec<Patch>)>> {
  let search_id = get_next_search_id();
  let result = search_diffs_inner(options, search_id)?;

  parse_all(P_MANY_PATCHES_WITH_COMMIT_IDS, &result)
}

// None result means either no results or cancelled.
pub fn search_diffs_with_id(
  options: &SearchOptions,
  search_id: u32,
) -> Option<Vec<(String, Vec<Patch>)>> {
  let result = search_diffs_inner(options, search_id)?;

  parse_all(P_MANY_PATCHES_WITH_COMMIT_IDS, &result)
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct FileMatch {
  patch: Patch,
  lines: Vec<HunkLine>,
}

// None result means either no results or cancelled.
pub fn search_diffs_with_id2(
  options: &SearchOptions,
  search_id: u32,
) -> Option<Vec<(String, Vec<FileMatch>)>> {
  let result = search_diffs_inner(options, search_id)?;

  let commit_patches = parse_all(P_MANY_PATCHES_WITH_COMMIT_IDS, &result)?;

  let SearchOptions {
    repo_path,
    search_text,
    ..
  } = options;

  let commits = COMMITS.get_by_key(repo_path)?;

  // TODO: Should we check for cancelled search while we do this?
  Some(
    commit_patches
      .into_iter()
      .flat_map(|(id, patches)| {
        let commit = commits.iter().find(|c| c.id == id)?;

        let matches = patches
          .into_iter()
          .flat_map(|patch| {
            Some(FileMatch {
              lines: get_matching_hunk_lines(repo_path, commit, &patch, search_text)?,
              patch,
            })
          })
          .collect::<Vec<FileMatch>>();

        Some((commit.id.clone(), matches))
      })
      .collect::<Vec<(String, Vec<FileMatch>)>>(),
  )
}

// TODO: Rename this.
pub fn search_diffs_inner(options: &SearchOptions, search_id: u32) -> Option<String> {
  println!(
    "Search for text: {}, id: {}, num: {}",
    options.search_text, search_id, options.num_results
  );

  let SearchOptions {
    repo_path,
    search_text,
    num_results,
    ..
  } = options;

  let mut child = Command::new("git")
    .args([
      "log",
      "-S",
      search_text,
      "--name-status",
      "--pretty=format:%H,",
      &format!("-n{}", num_results),
      "-z",
    ])
    .stdout(Stdio::piped())
    .current_dir(repo_path)
    .spawn()
    .ok()?;

  loop {
    thread::sleep(time::Duration::from_millis(50));

    if search_cancelled(search_id) {
      println!("Killing search {search_id} \"{search_text}\"");

      if let Err(e) = child.kill() {
        eprintln!("{}", e);
      }
      break;
    }

    if let Ok(exit_status) = child.try_status() {
      if exit_status.is_some() {
        if let Ok(result) = executor::block_on(child.output()) {
          let Output { stdout, stderr, .. } = &result;

          if !stdout.is_empty() {
            return Some(String::from_utf8_lossy(stdout).to_string());
          } else {
            println!(
              "Git Command stderr: {:?}",
              String::from_utf8_lossy(stderr).to_string()
            );
          }
        }

        break;
      }
    } else {
      break;
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use std::time::{Duration, Instant};
  use std::{assert_eq, println, thread};

  use crate::git::queries::search::{get_next_search_id, SearchOptions, _search_diffs};

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
  #[test]
  fn test_thing() {
    let t1 = thread::spawn(move || {
      _search_diffs(&SearchOptions {
        num_results: 500,
        search_text: "this".to_string(),
        repo_path: ".".to_string(),
      })
    });

    thread::sleep(Duration::from_millis(10));

    let t2 = thread::spawn(move || {
      _search_diffs(&SearchOptions {
        num_results: 500,
        search_text: "this".to_string(),
        repo_path: ".".to_string(),
      })
    });

    thread::sleep(Duration::from_millis(10));

    let t3 = thread::spawn(move || {
      _search_diffs(&SearchOptions {
        num_results: 5,
        search_text: "this".to_string(),
        repo_path: ".".to_string(),
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
}
