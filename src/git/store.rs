use crate::git::git_types::{Commit, GitConfig, Patch, RefInfo};
use crate::git::git_version::GitVersion;
use crate::git::queries::patches::cache::clear_patch_cache;
use crate::git::queries::search::search_request::clear_completed_searches;
use crate::git::repo_watcher::{clear_repo_changed_status, get_watched_repos};
use crate::server::git_request::ReqOptions;
use crate::util::global::{Glo, Global};
use crate::{dprintln, glo, global, time_block};
use ahash::AHashMap;
use std::collections::HashMap;
use std::env;

pub type RepoPath = String;
type PatchPath = String;

// static COMMITS: Global<AHashMap<RepoPath, Vec<CommitInfo>>> = global!(AHashMap::new());

static COMMITS_AND_REFS: Global<AHashMap<RepoPath, (Vec<Commit>, Vec<RefInfo>)>> =
  global!(AHashMap::new());

static PATCHES: Glo<(RepoPath, HashMap<PatchPath, Vec<Patch>>)> =
  glo!((RepoPath::new(), HashMap::new()));

// static PATCHES2: Glo<Arc<(RepoPath, HashMap<PatchPath, Vec<Patch>>)>> =
//   glo!(Arc::new((RepoPath::new(), HashMap::new())));

// Key is 2 commit ids joined.
pub static REF_DIFFS: Glo<AHashMap<String, u32>> = glo!(AHashMap::new());

// This probably needs to be per repo. We could then watch for changes?
pub static CONFIG: Global<AHashMap<RepoPath, GitConfig>> = global!(AHashMap::new());

pub static GIT_VERSION: Global<GitVersion> = global!(GitVersion::new());

// Assumes git is installed.
pub fn get_git_version() -> GitVersion {
  GIT_VERSION.get().unwrap_or_else(GitVersion::new)
}

pub fn insert_commits(repo_path: &RepoPath, commits: &Vec<Commit>, refs: &Vec<RefInfo>) {
  COMMITS_AND_REFS.insert(repo_path.to_owned(), (commits.to_owned(), refs.to_owned()));

  clear_repo_changed_status(&ReqOptions {
    repo_path: repo_path.to_string(),
  });
}

pub fn get_commits_and_refs(repo_path: &RepoPath) -> Option<(Vec<Commit>, Vec<RefInfo>)> {
  COMMITS_AND_REFS.get_by_key(repo_path)
}

pub fn get_all_workspace_commits() -> Option<AHashMap<RepoPath, (Vec<Commit>, Vec<RefInfo>)>> {
  let commits = COMMITS_AND_REFS.get()?;
  let watched_repos: HashMap<RepoPath, bool> = get_watched_repos()?;

  Some(
    commits
      .into_iter()
      .filter(|(repo_path, _)| watched_repos.contains_key(repo_path))
      .collect(),
  )
}

pub fn clear_unwatched_repos_from_commits(watched_repos: &HashMap<String, bool>) -> Option<()> {
  let commits = COMMITS_AND_REFS
    .get()?
    .into_iter()
    .filter(|(repo_path, _)| watched_repos.contains_key(repo_path))
    .collect();

  COMMITS_AND_REFS.set(commits);

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

    // PATCHES.set((repo_path.to_string(), patches.to_owned()));
  });
}

pub fn get_patches(repo_path: &str) -> Option<HashMap<String, Vec<Patch>>> {
  println!("TRYING TO GET PATCHES FOR: {}", repo_path);

  if let Ok(stored) = PATCHES.read() {
    // let (path, patches) = &stored;

    if stored.0 == repo_path && !stored.1.is_empty() {
      println!("Found {} patches for repo: {}", stored.1.len(), repo_path);
      return Some(stored.1.clone());
    }
  }

  None
}

// pub fn get_patches_ref(repo_path: &str) -> Option<Arc<(RepoPath, HashMap<PatchPath, Vec<Patch>>)>> {
//   println!("TRYING TO GET PATCHES FOR: {}", repo_path);
//
//   if let Ok(stored) = PATCHES2.read() {
//     // let (path, patches) = &stored;
//
//     if stored.0 == repo_path && !stored.1.is_empty() {
//       println!("Found {} patches for repo: {}", stored.1.len(), repo_path);
//       return Some(stored.clone());
//     }
//   }
//
//   None
// }

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
