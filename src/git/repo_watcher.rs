use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use loggers::elapsed;
use notify::{Event, RecursiveMode, Result, Watcher};
use serde::Deserialize;
use ts_rs::TS;

use crate::git::store::STORE;
use crate::server::git_request::ReqOptions;
use crate::util::global::Global;
use crate::{dprintln, global};

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct WatchRepoOptions {
  pub repo_paths: Vec<String>,
  pub root_repo: String,
  pub start_changed: bool,
}

// The root dir?
static CURRENT_DIR: Global<String> = global!(String::new());

static WATCH_DIRS: Global<HashMap<String, bool>> = global!(HashMap::new());

// Ignore anything in .git/logs
// What about ".git/index.lock" ?
const PATH_FILTER2: fn(&&PathBuf) -> bool = |path: &&PathBuf| {
  let mut ignore = false;

  let parts: Vec<&str> = path.iter().flat_map(|part| part.to_str()).collect();

  if let Some(i) = parts.iter().position(|p| *p == ".git") {
    if let Some(next) = parts.get(i + 1) {
      if *next == "logs" {
        ignore = true;
      }
    }
  }

  if parts.last().unwrap_or(&"").contains(".lock") {
    ignore = true
  }

  !ignore
};

pub fn watch_repo(options: &WatchRepoOptions) {
  let watched: HashMap<String, bool> = options
    .repo_paths
    .iter()
    .map(|path| (path.to_string(), options.start_changed))
    .collect();

  STORE.clear_unwatched_repos_from_commits(&watched);

  WATCH_DIRS.set(watched);

  let root_repo = options.root_repo.clone();

  thread::spawn(move || watch(root_repo));
}

pub fn repo_has_changed(options: &ReqOptions) -> Option<bool> {
  let dirs = WATCH_DIRS.get()?;
  let changed = dirs.get(&options.repo_path)?;

  Some(*changed)
}

pub fn clear_repo_changed_status(options: &ReqOptions) {
  let ReqOptions { repo_path } = options;

  dprintln!("clear_changed_status {}", repo_path);

  if let Some(mut dirs) = WATCH_DIRS.get() {
    if dirs.contains_key(repo_path) {
      dirs.insert(repo_path.to_string(), false);

      WATCH_DIRS.set(dirs);
    } else {
      dprintln!("clear_changed_status: {} isn't being watched", repo_path);
    }
  }
}

pub fn mark_changed(repo_path: &str) {
  if let Some(mut dirs) = WATCH_DIRS.get() {
    if dirs.contains_key(repo_path) {
      dirs.insert(repo_path.to_string(), true);

      WATCH_DIRS.set(dirs);
    } else {
      dprintln!("mark_changed: {} isn't being watched", repo_path);
    }
  }
}

fn watch(root_dir: String) -> Result<()> {
  if already_watching(&root_dir) {
    dprintln!("Already watching {}", root_dir);

    return Ok(());
  }

  let mut watcher = notify::recommended_watcher(|res: Result<Event>| match res {
    Ok(event) => {
      update_changed(event.paths);
    }
    Err(_e) => {
      dprintln!("watch error: {:?}", _e);
    }
  })?;

  dprintln!("Start watching dir {}", root_dir);
  CURRENT_DIR.set(root_dir.clone());

  watcher.watch(Path::new(&root_dir), RecursiveMode::Recursive)?;

  dprintln!("Watcher ready for dir {}", root_dir);

  loop {
    thread::sleep(Duration::from_millis(500));

    if !already_watching(&root_dir) {
      break;
    }
  }

  dprintln!("Stop watching dir {}", root_dir);

  Ok(())
}

fn already_watching(repo_path: &str) -> bool {
  if let Some(dir) = CURRENT_DIR.get() {
    if dir == repo_path {
      return true;
    }
  }
  false
}

#[elapsed]
fn update_changed(changed_paths: Vec<PathBuf>) {
  let changed_paths: Vec<String> = changed_paths
    .iter()
    .filter(PATH_FILTER2)
    .flat_map(|path| path.to_str())
    .map(|path| path.to_string())
    .collect();

  // if !changed_paths.is_empty() {
  //   dprintln!("{:?}", changed_paths);
  // }

  if let Some(mut watch_dirs) = WATCH_DIRS.get() {
    for changed in changed_paths {
      if let Some(matching_dir) = closest_match(&changed, &watch_dirs) {
        // println!("{:?} -> {}", changed, matching_dir);
        watch_dirs.insert(matching_dir, true);
      }
    }

    WATCH_DIRS.set(watch_dirs);
  }
}

fn closest_match(
  changed_path: &str,
  watch_dirs: &HashMap<String, bool>,
) -> Option<String> {
  let mut matches = Vec::<&String>::new();

  for dir in watch_dirs.keys() {
    if changed_path.starts_with(dir) {
      matches.push(dir);
    }
  }

  matches
    .iter()
    .max_by(|a, b| a.len().cmp(&b.len()))
    .cloned()
    .cloned()
}

#[cfg(test)]
mod tests {
  use crate::git::repo_watcher::PATH_FILTER2;
  use std::path::Path;

  #[test]
  fn test_path_filter2() {
    let is_ignored = |path| !PATH_FILTER2(path);

    let p = &Path::new("/repo/.git").to_path_buf();
    assert!(!is_ignored(&p));

    let p = &Path::new("/repo/.git/logs/HEAD").to_path_buf();
    assert!(is_ignored(&p));

    let p = &Path::new("/repo/.git/logs").to_path_buf();
    assert!(is_ignored(&p));

    let p = &Path::new("/repo/.git/HEAD").to_path_buf();
    assert!(!is_ignored(&p));

    let p = &Path::new("/repo/.git/ORIG_HEAD").to_path_buf();
    assert!(!is_ignored(&p));

    let p =
      &Path::new("/Users/tobysuggate/Repos/game-dist/.git/index.lock").to_path_buf();
    assert!(is_ignored(&p));
  }
}
