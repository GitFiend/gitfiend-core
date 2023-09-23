use crate::git::git_types::{Commit, GitConfig, Patch, RefInfo};
use crate::git::git_version::GitVersion;
use crate::git::queries::patches::cache::clear_patch_cache;
use crate::git::queries::search::search_request::clear_completed_searches;
use crate::git::repo_watcher::clear_repo_changed_status;
use crate::server::git_request::ReqOptions;
use crate::util::global::{Glo, Global};
use crate::{dprintln, glo, global, time_block};
use ahash::AHashMap;
use std::collections::HashMap;
use std::env;

pub type RepoPath = String;
type PatchPath = String;

type CommitsAndRefs = (Vec<Commit>, Vec<RefInfo>);

static COMMITS_AND_REFS: Glo<AHashMap<RepoPath, CommitsAndRefs>> = glo!(AHashMap::new());

static PATCHES: Glo<(RepoPath, HashMap<PatchPath, Vec<Patch>>)> =
  glo!((RepoPath::new(), HashMap::new()));

// Key is 2 commit ids joined.
pub static REF_DIFFS: Glo<AHashMap<String, u32>> = glo!(AHashMap::new());

// This probably needs to be per repo. We could then watch for changes?
pub static CONFIG: Global<AHashMap<RepoPath, GitConfig>> = global!(AHashMap::new());

pub static GIT_VERSION: Glo<GitVersion> = glo!(GitVersion::new());

// Assumes git is installed.
pub fn get_git_version() -> GitVersion {
  if let Ok(version) = GIT_VERSION.read() {
    return (*version).to_owned();
  }
  GitVersion::new()
}

pub fn insert_commits(repo_path: &RepoPath, commits: &Vec<Commit>, refs: &Vec<RefInfo>) {
  if let Ok(mut cr) = COMMITS_AND_REFS.write() {
    (*cr).insert(repo_path.to_string(), (commits.to_owned(), refs.to_owned()));
  }

  clear_repo_changed_status(&ReqOptions {
    repo_path: repo_path.to_string(),
  });
}

pub fn get_commits_and_refs(repo_path: &RepoPath) -> Option<CommitsAndRefs> {
  if let Ok(cr) = COMMITS_AND_REFS.read() {
    return Some((*cr).get(repo_path)?.to_owned());
  }

  None
}

fn get_all_commits_and_refs() -> Option<AHashMap<RepoPath, CommitsAndRefs>> {
  let cr = COMMITS_AND_REFS.read().ok()?;

  Some((*cr).to_owned())
}

pub fn clear_unwatched_repos_from_commits(watched_repos: &HashMap<String, bool>) -> Option<()> {
  let commits = get_all_commits_and_refs()?
    .into_iter()
    .filter(|(repo_path, _)| watched_repos.contains_key(repo_path))
    .collect();

  if let Ok(mut cr) = COMMITS_AND_REFS.write() {
    *cr = commits
  }

  let configs = CONFIG
    .get()?
    .into_iter()
    .filter(|(repo_path, _)| watched_repos.contains_key(repo_path))
    .collect();

  CONFIG.set(configs);

  Some(())
}

pub fn insert_patches(repo_path: &str, patches: &HashMap<String, Vec<Patch>>) {
  time_block!("insert_patches", {
    if let Ok(mut saved_patches) = PATCHES.write() {
      *saved_patches = (repo_path.to_owned(), patches.to_owned());
    }
  });
}

pub fn get_patches(repo_path: &str) -> Option<HashMap<String, Vec<Patch>>> {
  if let Ok(stored) = PATCHES.read() {
    if stored.0 == repo_path && !stored.1.is_empty() {
      return Some(stored.1.clone());
    }
  }

  None
}

pub fn clear_cache(_: &ReqOptions) {
  clear_completed_searches();

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
