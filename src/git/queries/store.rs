use crate::git::git_types::Commit;
use lazy_static::lazy_static; // 1.4.0
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
  static ref COMMITS_CACHE: Mutex<HashMap<String, Vec<Commit>>> = Mutex::new(HashMap::new());
}

pub fn store_commits(repo_path: &String, commits: &Vec<Commit>) -> () {
  COMMITS_CACHE
    .lock()
    .unwrap()
    .insert(repo_path.clone(), commits.clone());
}

pub fn load_commits_from_store(repo_path: &String) -> Option<Vec<Commit>> {
  let cached = COMMITS_CACHE.lock().unwrap();

  let commits = cached.get(repo_path.clone().as_str())?;

  Some(commits.clone())
}
