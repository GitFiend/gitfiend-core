use crate::git::git_types::{Commit, GitConfig};
use crate::server::git_request::ReqOptions;
use ahash::AHashMap;
use std::sync::{Arc, RwLock};

pub type RwStore = Arc<RwLock<Store>>;

pub struct Store {
  pub commits: AHashMap<String, Vec<Commit>>,
  pub config: GitConfig,
  // pub ref_diffs: AHashMap<String, u32>,
  // pub current_search: u32,
}

impl Store {
  pub fn new() -> Store {
    Store {
      commits: AHashMap::new(),
      config: GitConfig::new(),
      // ref_diffs: AHashMap::new(),
      // current_search: 0,
    }
  }

  pub fn new_lock() -> RwStore {
    Arc::new(RwLock::new(Store::new()))
  }
}

pub fn load_commits_from_store(repo_path: &String, store_lock: &RwStore) -> Option<Vec<Commit>> {
  if let Ok(store) = store_lock.read() {
    return Some((*store).commits.get(repo_path)?.clone());
  }

  None
}

pub fn load_config_from_store(store_lock: &RwStore) -> Option<GitConfig> {
  Some((*store_lock).read().ok()?.config.clone())
}

// pub fn get_ref_diff_from_store(store_lock: &RwStore, key: &str) -> Option<u32> {
//   let read = (*store_lock).read();
//
//   if read.is_ok() {
//     let ref diffs = read.ok()?.ref_diffs;
//     return Some(diffs.get(key)?.clone());
//   } else {
//     println!("Failed to read from store");
//   }
//
//   None
// }
//
// pub fn store_ref_diff(store_lock: &RwStore, key: &str, value: u32) {
//   if let Ok(mut store) = store_lock.write() {
//     println!("storing {key}");
//     (*store).ref_diffs.insert(key.to_string(), value);
//   } else {
//     println!("Failed to get write lock");
//   }
// }

pub fn clear_cache(_: &ReqOptions, store_lock: RwStore) {
  if let Ok(mut store) = store_lock.write() {
    (*store).commits = AHashMap::new();
    // (*store).ref_diffs = AHashMap::new();

    println!("Cleared commits cache.");
  }
}

// pub fn get_current_search_num(store_lock: &RwStore) -> u32 {
//   if let Ok(store) = store_lock.read() {
//     (*store).current_search
//   } else {
//     0
//   }
// }
//
// pub fn get_next_search_num(store_lock: &RwStore) -> u32 {
//   if let Ok(mut store) = store_lock.write() {
//     (*store).current_search += 1;
//
//     (*store).current_search
//   } else {
//     0
//   }
// }
