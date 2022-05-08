use crate::git::git_types::{Commit, GitConfig};
use lazy_static::lazy_static; // 1.4.0
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
  // TODO: Clear out repos that aren't currently open.
  static ref COMMITS_CACHE: Mutex<HashMap<String, Vec<Commit>>> = Mutex::new(HashMap::new());
}

static CONFIG_CACHE: OnceCell<GitConfig> = OnceCell::new();

pub fn store_commits(repo_path: &String, commits: &Vec<Commit>) -> () {
  let run = || {
    COMMITS_CACHE
      .lock()
      .ok()?
      .insert(repo_path.clone(), commits.clone())
  };

  if let None = run() {
    println!("Failed to store commits in cache.")
  }

  #[cfg(debug_assertions)]
  assert!(load_commits_from_store(&repo_path).is_some());
}

pub fn load_commits_from_store(repo_path: &String) -> Option<Vec<Commit>> {
  let cached = COMMITS_CACHE.lock().ok()?;

  let commits = cached.get(repo_path.clone().as_str())?;

  Some(commits.clone())
}

pub fn store_config(config: &GitConfig) {
  if let Err(_) = CONFIG_CACHE.set(config.clone()) {
    println!("Failed to insert git config into cache.");
  }
}

pub fn load_config_from_store() -> Option<GitConfig> {
  Some(CONFIG_CACHE.get()?.clone())
}
