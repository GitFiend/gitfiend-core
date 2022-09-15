use crate::dprintln;
use crate::git::git_settings::GIT_PATH;
use crate::git::queries::search::{search_cancelled, SearchOptions};
use crate::git::store::COMMITS;
use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

pub fn search_diffs_inner2(options: &SearchOptions, search_id: u32) -> Option<String> {
  dprintln!(
    "Search for text: {}, id: {}, num: {}",
    options.search_text,
    search_id,
    options.num_results
  );

  let SearchOptions {
    repo_path,
    search_text,
    num_results,
    ..
  } = options;

  let commits = COMMITS.get_by_key(repo_path)?;
  let first_commit_id = &commits.first()?.id;
  let last_commit_id = &commits.last()?.id;

  let mut cmd = Command::new(GIT_PATH.as_path())
    .args([
      "log",
      &format!("{}..{}", last_commit_id, first_commit_id),
      "-S",
      search_text,
      "--name-status",
      "--branches",
      "--remotes",
      "--pretty=format:%H,",
      &format!("-n{}", num_results),
      "-z",
    ])
    .stdout(Stdio::piped())
    .current_dir(repo_path)
    .spawn()
    .ok()?;

  while let Ok(None) = cmd.try_wait() {
    if search_cancelled(search_id) {
      dprintln!("Killing search {search_id} \"{search_text}\"");

      if let Err(e) = cmd.kill() {
        dprintln!("{}", e);
      }
      return None;
    }

    thread::sleep(Duration::from_millis(50));
  }

  let status = cmd.wait().ok()?;

  if status.success() {
    let mut text = String::new();

    if let Some(mut out) = cmd.stdout {
      if let Ok(len) = out.read_to_string(&mut text) {
        if len > 0 {
          return Some(text);
        }
      }
    }
  }

  None
}
