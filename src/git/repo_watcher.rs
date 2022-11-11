use crate::server::git_request::ReqOptions;
use crate::util::global::Global;
use crate::{dprintln, global};
use loggers::elapsed;
use notify::{Event, RecursiveMode, Result, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

// TODO: Will need to update this for the sub repos we are interested in.
pub fn open_repo(options: &ReqOptions) {
  start_watching(&options.repo_path);
}

pub fn close_repo(_: &ReqOptions) {
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

pub fn start_watching(repo_path: &str) {
  let repo_path = repo_path.to_string();

  thread::spawn(move || watch(&repo_path));
}

//
fn watch(repo_path: &str) -> Result<()> {
  if let Some(dirs) = WATCH_DIRS.get() {
    if dirs.contains_key(repo_path) {
      return Ok(());
    }
  }

  // Automatically select the best implementation for your platform.
  let mut watcher = notify::recommended_watcher(|res: Result<Event>| match res {
    Ok(event) => {
      dprintln!("{:?}", event.paths);
      update_changed(event.paths);
    }
    Err(e) => {
      dprintln!("watch error: {:?}", e);
    }
  })?;

  let mut dirs = HashMap::new();
  dirs.insert(repo_path.to_string(), false);
  WATCH_DIRS.set(dirs);
  // WATCH_DIR.set(repo_path.to_string());

  dprintln!("Start watching dir {}", repo_path);

  // Add a path to be watched. All files and directories at that path and
  // below will be monitored for changes.
  watcher.watch(Path::new(repo_path), RecursiveMode::Recursive)?;

  loop {
    thread::sleep(Duration::from_millis(500));

    if let Some(dirs) = WATCH_DIRS.get() {
      if !dirs.contains_key(repo_path) {
        break;
      }
    } else {
      break;
    }
  }

  dprintln!("Stop watching dir {}", repo_path);

  Ok(())
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
