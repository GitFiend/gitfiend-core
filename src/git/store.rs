use crate::git::git_types::{Commit, GitConfig, Patch};
use crate::git::git_version::GitVersion;
use crate::git::queries::search::search_request::clear_completed_searches;
use crate::git::run_git_action::ActionProgress;
use crate::global;
use crate::server::git_request::ReqOptions;
use crate::util::global::Global;
use ahash::AHashMap;
use std::collections::HashMap;

pub static COMMITS: Global<AHashMap<String, Vec<Commit>>> = global!(AHashMap::new());

pub static PATCHES: Global<(String, HashMap<String, Vec<Patch>>)> =
  global!((String::new(), HashMap::new()));

pub static REF_DIFFS: Global<AHashMap<String, u32>> = global!(AHashMap::new());

pub static CONFIG: Global<GitConfig> = global!(GitConfig::new());

pub static ACTION_LOGS: Global<Vec<ActionProgress>> = global!(Vec::new());

pub static GIT_VERSION: Global<GitVersion> = global!(GitVersion::new());

pub fn clear_cache(_: &ReqOptions) {
  COMMITS.clear();
  clear_completed_searches();

  println!("Cleared cache.");
}
