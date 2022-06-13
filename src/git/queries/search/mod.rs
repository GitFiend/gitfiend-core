use std::process::Output;
use std::{thread, time};

use async_process::Command;
use futures::executor;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::git::store::RwStore;
use crate::global;
use crate::util::global::Global;

mod search;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SearchOptions {
  pub repo_path: String,
  pub search_text: String,
  pub num_results: u32,
}

static CURRENT_SEARCH: Global<u32> = global!();

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
    search_id != id
  } else {
    false
  }
}

/*
TODO: Create and run async version of run_git

use async_process library for Command.
call try_status on child process every x milliseconds

if done, return Some(result), if not, check get_current_search_num() == search_num.
if we are still the current search, continue polling.
If we aren't, return None.
 */
pub fn search_diffs(options: &SearchOptions, store: RwStore) -> Option<String> {
  let search_num = get_next_search_id();

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
    .current_dir(repo_path)
    .spawn()
    .ok()?;

  loop {
    thread::sleep(time::Duration::from_millis(60));

    if search_cancelled(search_num) {
      if let Err(e) = child.kill() {
        eprintln!("{}", e);
      }
      break;
    } else if let Ok(exit_status) = child.try_status() {
      if exit_status.is_some() {
        if let Ok(result) = executor::block_on(child.output()) {
          let Output { stdout, stderr, .. } = &result;

          println!("stdout.len(): {}", stdout.len());
          // TODO: Is stderr sometimes valid and useful git output?
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
  use std::time::Instant;
  use std::{assert_eq, println};

  use crate::git::queries::search::get_next_search_id;

  #[test]
  fn test_get_next_search_id() {
    assert_eq!(get_next_search_id(), 0);
    assert_eq!(get_next_search_id(), 1);
    assert_eq!(get_next_search_id(), 2);

    let now = Instant::now();

    while get_next_search_id() < 1_000 {}

    println!("Took {}us", now.elapsed().as_micros());
  }
}
