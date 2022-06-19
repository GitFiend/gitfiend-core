use crate::git::git_types::{Commit, GitConfig};
use crate::git::queries::commits::COMMITS;
use crate::git::queries::search::search_request::clear_completed_searches;
use crate::server::git_request::ReqOptions;
use ahash::AHashMap;
use std::sync::{Arc, RwLock};

pub type RwStore = Arc<RwLock<Store>>;

pub struct Store {
  // pub commits: AHashMap<String, Vec<Commit>>,
// pub config: GitConfig,
}

impl Store {
  pub fn new() -> Store {
    Store {
      // commits: AHashMap::new(),
      // config: GitConfig::new(),
    }
  }

  pub fn new_lock() -> RwStore {
    Arc::new(RwLock::new(Store::new()))
  }
}

// pub fn load_commits_from_store(repo_path: &String, store_lock: &RwStore) -> Option<Vec<Commit>> {
//   if let Ok(store) = store_lock.read() {
//     return Some((*store).commits.get(repo_path)?.clone());
//   }
//
//   None
// }

// pub fn load_config_from_store(store_lock: &RwStore) -> Option<GitConfig> {
//   Some((*store_lock).read().ok()?.config.clone())
// }

pub fn clear_cache(_: &ReqOptions, store_lock: RwStore) {
  // if let Ok(mut store) = store_lock.write() {
  //   (*store).commits = AHashMap::new();
  //
  //   println!("Cleared commits cache.");
  // }

  COMMITS.clear();
  clear_completed_searches();

  println!("Cleared cache.");
}
