use crate::git::git_types::{Commit, GitConfig};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Store {
  pub commits: HashMap<String, Vec<Commit>>,
  pub config: GitConfig,
}

impl Store {
  pub fn new() -> Store {
    Store {
      commits: HashMap::new(),
      config: GitConfig::new(),
    }
  }

  pub fn new_lock() -> Arc<RwLock<Store>> {
    Arc::new(RwLock::new(Store::new()))
  }
}

pub fn load_commits_from_store(
  repo_path: &String,
  store_lock: &Arc<RwLock<Store>>,
) -> Option<Vec<Commit>> {
  if let Ok(store) = store_lock.read() {
    return Some((*store).commits.get(repo_path)?.clone());
  }

  None
}

pub fn load_config_from_store(store_lock: &Arc<RwLock<Store>>) -> Option<GitConfig> {
  Some((*store_lock).read().ok()?.config.clone())
}
