use std::process::{Output, Stdio};
use std::{thread, time};

use async_process::Command;
use futures::executor;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::git::git_types::Patch;
use crate::git::queries::patches::patch_parsers::P_MANY_PATCHES_WITH_COMMIT_IDS;
use crate::git::store::RwStore;
use crate::global2;
use crate::parser::parse_all;
use crate::util::global2::Global2;

mod search;
pub(crate) mod search_request;

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SearchOptions {
  pub repo_path: String,
  pub search_text: String,
  pub num_results: u32,
}

static CURRENT_SEARCH: Global2<u32> = global2!(0);

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
pub fn search_diffs(options: &SearchOptions, _: RwStore) -> Option<Vec<(String, Vec<Patch>)>> {
  let search_id = get_next_search_id();
  let result = search_diffs_inner(&options, search_id)?;

  parse_all(P_MANY_PATCHES_WITH_COMMIT_IDS, &result)
}

pub fn search_diffs_with_id(
  options: &SearchOptions,
  search_id: u32,
) -> Option<Vec<(String, Vec<Patch>)>> {
  let result = search_diffs_inner(&options, search_id)?;

  parse_all(P_MANY_PATCHES_WITH_COMMIT_IDS, &result)
}

// TODO: Rename this.
pub fn search_diffs_inner(options: &SearchOptions, search_id: u32) -> Option<String> {
  println!(
    "Search for text: {}, num: {}",
    options.search_text, search_id
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

          if stdout.len() > 0 {
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

  use crate::git::queries::search::{get_next_search_id, search_diffs, SearchOptions};
  use crate::git::store::Store;

  #[test]
  fn test_get_next_search_id() {
    assert_eq!(get_next_search_id(), 0);
    assert_eq!(get_next_search_id(), 1);
    assert_eq!(get_next_search_id(), 2);

    let now = Instant::now();

    while get_next_search_id() < 1_000 {}

    println!("Took {}us", now.elapsed().as_micros());
  }

  // We can't run tests in parallel as they will be killed.
  #[test]
  fn test_thing() {
    let t1 = thread::spawn(move || {
      search_diffs(
        &SearchOptions {
          num_results: 500,
          search_text: "this".to_string(),
          repo_path: ".".to_string(),
        },
        Store::new_lock(),
      )
    });

    thread::sleep(Duration::from_millis(10));

    let t2 = thread::spawn(move || {
      search_diffs(
        &SearchOptions {
          num_results: 500,
          search_text: "this".to_string(),
          repo_path: ".".to_string(),
        },
        Store::new_lock(),
      )
    });

    thread::sleep(Duration::from_millis(10));

    let t3 = thread::spawn(move || {
      search_diffs(
        &SearchOptions {
          num_results: 5,
          search_text: "this".to_string(),
          repo_path: ".".to_string(),
        },
        Store::new_lock(),
      )
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
