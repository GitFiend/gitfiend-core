use crate::server::git_request::ReqOptions;
use crate::util::global::Global;
use crate::{dprintln, global};
use loggers::elapsed;
use notify::{Event, RecursiveMode, Result, Watcher};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use ts_rs::TS;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct WatchRepoOptions {
  pub repo_paths: Vec<String>,
}

pub fn watch_repo(options: &WatchRepoOptions) {
  let repo_paths = options.repo_paths.clone();

  thread::spawn(move || watch(repo_paths));
}

pub fn stop_watching_repo(_: &ReqOptions) {
  stop_watching();
}

pub fn get_changed_repos(_: &ReqOptions) -> Option<HashMap<String, bool>> {
  WATCH_DIRS.get()
}

pub fn clear_changed_status(repo_path: &str) {
  dprintln!("clear_changed_status {}", repo_path);

  if let Some(mut dirs) = WATCH_DIRS.get() {
    if dirs.contains_key(repo_path) {
      dirs.insert(repo_path.to_string(), false);

      WATCH_DIRS.set(dirs);
    } else {
      dprintln!("clear_changed_status: {} isn't being watched", repo_path);
    }
    // WATCH_DIRS.set(dirs.into_iter().map(|(dir, _)| (dir, false)).collect());
  }
}

static WATCH_DIRS: Global<HashMap<String, bool>> = global!(HashMap::new());

fn watch(repo_paths: Vec<String>) -> Result<()> {
  if already_watching(&repo_paths) {
    return Ok(());
  }

  let repo_path =
    get_root_repo(&repo_paths).ok_or_else(|| notify::Error::generic("Empty repo list"))?;

  let mut watcher = notify::recommended_watcher(|res: Result<Event>| match res {
    Ok(event) => {
      update_changed(event.paths);
    }
    Err(e) => {
      dprintln!("watch error: {:?}", e);
    }
  })?;

  WATCH_DIRS.set(
    repo_paths
      .iter()
      .map(|path| (path.to_string(), false))
      .collect(),
  );

  dprintln!("Start watching dir {}", repo_path);

  watcher.watch(Path::new(&repo_path), RecursiveMode::Recursive)?;

  loop {
    thread::sleep(Duration::from_millis(500));

    if !already_watching(&repo_paths) {
      break;
    }
  }

  dprintln!("Stop watching dir {}", repo_path);

  Ok(())
}

fn get_root_repo(repo_paths: &[String]) -> Option<String> {
  repo_paths
    .iter()
    .min_by(|a, b| a.len().cmp(&b.len()))
    .cloned()
}

fn already_watching(repo_paths: &Vec<String>) -> bool {
  if let Some(dirs) = WATCH_DIRS.get() {
    if dirs.len() != repo_paths.len() {
      return false;
    }

    return repo_paths
      .iter()
      .all(|repo_path| dirs.contains_key(repo_path));
  }
  false
}

#[elapsed]
fn update_changed(changed_paths: Vec<PathBuf>) {
  let changed_paths: Vec<String> = changed_paths
    .iter()
    .flat_map(|path| path.to_str())
    .map(|path| path.to_string())
    .filter(|path| !(path.contains(".git") && path.ends_with(".lock")))
    .collect();

  if !changed_paths.is_empty() {
    dprintln!("{:?}", changed_paths);
  }

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

fn closest_match(changed_path: &str, watch_dirs: &HashMap<String, bool>) -> Option<String> {
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

pub fn stop_watching() {
  WATCH_DIRS.set(HashMap::new());
}
