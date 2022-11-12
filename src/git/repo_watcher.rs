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

pub fn clear_changed_status() {
  println!("clear_changed_status");

  if let Some(dirs) = WATCH_DIRS.get() {
    WATCH_DIRS.set(dirs.into_iter().map(|(dir, _)| (dir, false)).collect());
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
      dprintln!("{:?}", event.paths);
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

fn get_root_repo(repo_paths: &Vec<String>) -> Option<String> {
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
  if let Some(watch_dirs) = WATCH_DIRS.get() {
    let new_dirs = watch_dirs
      .into_iter()
      .map(|(watched_dir, changed)| {
        if !changed {
          (
            watched_dir.clone(),
            changed_paths.iter().any(|path| {
              if let Some(path_str) = path.to_str() {
                if path_str.contains(&watched_dir) {
                  return true;
                }
              }
              false
            }),
          )
        } else {
          (watched_dir, true)
        }
      })
      .collect();

    WATCH_DIRS.set(new_dirs);
  }
}

pub fn stop_watching() {
  WATCH_DIRS.set(HashMap::new());
}
