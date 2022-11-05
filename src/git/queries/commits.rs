use std::cmp::Ordering;
use std::thread;
use std::time::Instant;

use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::dprintln;
use crate::git::git_types::{Commit, RefInfo};
use crate::git::queries::commit_calcs::{
  find_commit_ancestors, get_commit_ids_between_commits2, get_commit_map_cloned,
};
use crate::git::queries::commit_filters::{apply_commit_filters, CommitFilter};
use crate::git::queries::commits_parsers::{PRETTY_FORMATTED, P_COMMITS, P_COMMIT_ROW, P_ID_LIST};
use crate::git::queries::patches::patches::load_patches;
use crate::git::queries::refs::finish_initialising_refs_on_commits;
use crate::git::queries::stashes::load_stashes;
use crate::git::run_git::{run_git, RunGitOptions};
use crate::git::store;
use crate::parser::parse_all;
use crate::server::git_request::ReqOptions;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TopCommitOptions {
  pub repo_path: String,
  pub branch_name: String,
}

pub fn load_top_commit_for_branch(options: &TopCommitOptions) -> Option<Commit> {
  let _now = Instant::now();

  let out = run_git(RunGitOptions {
    args: [
      "log",
      &options.branch_name,
      "--decorate=full",
      PRETTY_FORMATTED,
      "-n1",
      "--date=raw",
    ],
    repo_path: &options.repo_path,
  });

  dprintln!(
    "Took {}ms to request top commit from Git",
    _now.elapsed().as_millis()
  );

  parse_all(P_COMMIT_ROW, out?.as_str())
}

pub fn load_head_commit(options: &ReqOptions) -> Option<Commit> {
  let out = run_git(RunGitOptions {
    args: [
      "log",
      "--decorate=full",
      PRETTY_FORMATTED,
      "-n1",
      "--date=raw",
    ],
    repo_path: &options.repo_path,
  });

  parse_all(P_COMMIT_ROW, out?.as_str())
}

#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqCommitsOptions2 {
  pub repo_path: String,
  pub num_commits: u32,
  pub filters: Vec<CommitFilter>,
  pub fast: bool, // Fast means to use the cache only, don't run git command.
}

pub fn load_commits_and_stashes(options: &ReqCommitsOptions2) -> Option<Vec<Commit>> {
  let ReqCommitsOptions2 {
    repo_path,
    num_commits,
    filters,
    fast,
  } = options;

  if *fast {
    if let Some(commits) = store::get_commits(repo_path) {
      return Some(apply_commit_filters(repo_path, commits, filters));
    }
  }

  let now = Instant::now();

  let p1 = repo_path.clone();
  let p2 = repo_path.clone();
  let num = *num_commits;

  let stashes_thread = thread::spawn(move || load_stashes(&p1));
  let commits_thread = thread::spawn(move || load_commits(&p2, num));

  let stashes = stashes_thread.join().ok()?;
  let mut commits = commits_thread.join().ok()??;

  dprintln!(
    "Took {}ms to request stashes and commits from Git",
    now.elapsed().as_millis()
  );

  if let Some(mut stashes) = stashes {
    commits.append(&mut stashes);
  }

  commits.sort_by(|a, b| {
    if b.stash_id.is_some() || a.stash_id.is_some() {
      b.date.ms.partial_cmp(&a.date.ms).unwrap_or(Ordering::Equal)
    } else {
      Ordering::Equal
    }
  });

  for (i, c) in commits.iter_mut().enumerate() {
    c.index = i;
  }

  let now = Instant::now();

  let commits = finish_initialising_refs_on_commits(commits);

  dprintln!(
    "Took {}ms to get refs from commits *****",
    now.elapsed().as_millis()
  );

  store::insert_commits(repo_path, &commits);

  // let new_options = ReqCommitsOptions {
  //   repo_path: repo_path.clone(),
  //   num_commits: *num_commits,
  // };
  // thread::spawn(move || load_patches(&new_options));

  if !filters.is_empty() {
    load_patches(repo_path, &commits);
  }

  Some(apply_commit_filters(repo_path, commits, filters))
}

pub fn load_commits(repo_path: &String, num: u32) -> Option<Vec<Commit>> {
  let now = Instant::now();

  let out = run_git(RunGitOptions {
    args: [
      "log",
      "--branches",
      "--tags",
      "--remotes",
      "--decorate=full",
      PRETTY_FORMATTED,
      format!("-n{}", num).as_str(),
      "--date=raw",
    ],
    repo_path,
  })?;

  dprintln!(
    "Took {}ms to request {} commits from Git",
    now.elapsed().as_millis(),
    num
  );

  let now = Instant::now();
  let result = parse_all(P_COMMITS, &out);

  dprintln!(
    "Took {}ms to parse {} commits. Length {}",
    now.elapsed().as_millis(),
    num,
    out.len()
  );

  result
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CommitDiffOpts {
  pub repo_path: String,
  pub commit_id1: String,
  pub commit_id2: String,
}

pub fn commit_ids_between_commits(options: &CommitDiffOpts) -> Option<Vec<String>> {
  let CommitDiffOpts {
    repo_path,
    commit_id1,
    commit_id2,
  } = options;

  if let Some(commits) = store::get_commits(repo_path) {
    let commit_map: AHashMap<String, Commit> =
      commits.into_iter().map(|c| (c.id.clone(), c)).collect();

    if let Some(result) = get_commit_ids_between_commits2(commit_id2, commit_id1, &commit_map) {
      return Some(result);
    }
  }

  commit_ids_between_commits_fallback(repo_path, commit_id1, commit_id2)
}

// We use this when commit ids are outside our loaded range (not in COMMITS).
pub fn commit_ids_between_commits_fallback(
  repo_path: &str,
  commit_id1: &str,
  commit_id2: &str,
) -> Option<Vec<String>> {
  let now = Instant::now();

  let out = run_git(RunGitOptions {
    args: ["rev-list", &format!("{}..{}", commit_id1, commit_id2)],
    repo_path,
  })?;

  dprintln!("Took {}ms to request ids", now.elapsed().as_millis());

  parse_all(P_ID_LIST, &out)
}

// Use this as a fallback when calculation fails.
pub fn get_un_pushed_commits(options: &ReqOptions) -> Vec<String> {
  if let Some(ids) = get_un_pushed_commits_computed(options) {
    // println!("Computed ids: {:?}", ids);
    return ids;
  } else {
    dprintln!("get_un_pushed_commits: Refs not found in commits, fall back to git request.");
  }

  if let Some(out) = run_git(RunGitOptions {
    repo_path: &options.repo_path,
    args: ["log", "HEAD", "--not", "--remotes", "--pretty=format:%H"],
  }) {
    if let Some(ids) = parse_all(P_ID_LIST, &out) {
      return ids;
    }
  }

  Vec::new()
}

// This will return none if head ref or remote ref can't be found in provided commits.
fn get_un_pushed_commits_computed(options: &ReqOptions) -> Option<Vec<String>> {
  let now = Instant::now();

  let commits = store::get_commits(&options.repo_path)?;
  // let commits = load_commits_from_store(&options.repo_path, &store)?;

  let commit_map: AHashMap<String, Commit> = commits
    .clone()
    .into_iter()
    .map(|c| (c.id.clone(), c))
    .collect();

  // let commit = commits.iter().find(|c| c.refs.iter().any(|r| r.head));

  // println!("{:?}", commit.unwrap());

  let head_ref = get_head_ref(&commits)?;
  let remote = find_sibling_ref(head_ref, &commits)?;

  let result = get_commit_ids_between_commits2(&head_ref.commit_id, &remote.commit_id, &commit_map);

  dprintln!(
    "get_un_pushed_commits_computed Took {}ms",
    now.elapsed().as_millis()
  );

  result
}

fn get_head_ref(commits: &[Commit]) -> Option<&RefInfo> {
  commits
    .iter()
    .find(|c| c.refs.iter().any(|r| r.head))?
    .refs
    .iter()
    .find(|r| r.head)
}

pub fn find_sibling_ref<'a>(ri: &RefInfo, commits: &'a [Commit]) -> Option<&'a RefInfo> {
  if let Some(sibling_id) = &ri.sibling_id {
    return commits
      .iter()
      .find(|c| c.refs.iter().any(|r| &r.id == sibling_id))?
      .refs
      .iter()
      .find(|r| &r.id == sibling_id);
  }
  None
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CommitAncestorOpts {
  pub repo_path: String,
  pub commit_id: String,
  pub ancestor_candidate_id: String,
}

pub fn commit_is_ancestor(options: &CommitAncestorOpts) -> bool {
  let CommitAncestorOpts {
    repo_path,
    commit_id,
    ancestor_candidate_id,
  } = options;

  if let Some(commits) = store::get_commits(repo_path) {
    let commits = get_commit_map_cloned(&commits);

    if let Some(commit) = commits.get(commit_id) {
      let ancestors = find_commit_ancestors(commit, &commits);

      return ancestors.contains(ancestor_candidate_id.as_str());
    }
  }

  false
}
