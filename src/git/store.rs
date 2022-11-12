use crate::git::git_types::{Commit, GitConfig, Patch};
use crate::git::git_version::GitVersion;
use crate::git::queries::patches::cache::clear_patch_cache;
use crate::git::queries::search::search_request::clear_completed_searches;
use crate::git::repo_watcher::clear_changed_status;
use crate::server::git_request::ReqOptions;
use crate::util::global::Global;
use crate::{dprintln, global, time_block};
use ahash::AHashMap;
use std::collections::HashMap;
use std::env;

static COMMITS: Global<AHashMap<String, Vec<Commit>>> = global!(AHashMap::new());

static PATCHES: Global<(String, HashMap<String, Vec<Patch>>)> =
  global!((String::new(), HashMap::new()));

pub static REF_DIFFS: Global<AHashMap<String, u32>> = global!(AHashMap::new());

pub static CONFIG: Global<GitConfig> = global!(GitConfig::new());

pub static GIT_VERSION: Global<GitVersion> = global!(GitVersion::new());

pub fn insert_commits(repo_path: &str, commits: &Vec<Commit>) {
  COMMITS.insert(repo_path.to_string(), commits.to_owned());
  clear_changed_status(repo_path);
}

pub fn get_commits(repo_path: &str) -> Option<Vec<Commit>> {
  COMMITS.get_by_key(&repo_path.to_string())
}

pub fn insert_patches(repo_path: &str, patches: &HashMap<String, Vec<Patch>>) {
  time_block!("insert_patches", {
    PATCHES.set((repo_path.to_string(), patches.to_owned()));
  });
}

pub fn get_patches(repo_path: &str) -> Option<HashMap<String, Vec<Patch>>> {
  if let Some((path, patches)) = PATCHES.get() {
    if path == repo_path && !patches.is_empty() {
      return Some(patches);
    }
  }

  None
}

pub fn clear_cache(_: &ReqOptions) {
  clear_completed_searches();
  // clear_changed_status();

  dprintln!("Cleared cache.");
}

pub fn clear_all_caches(_: &ReqOptions) {
  clear_completed_searches();
  clear_patch_cache();

  dprintln!("Cleared all caches.");
}

pub fn override_git_home(options: &ReqOptions) {
  dprintln!("HOME before override: {:?}", env::var("HOME"));

  env::set_var("HOME", &options.repo_path);
}
