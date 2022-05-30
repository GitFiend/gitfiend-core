use ahash::AHashMap;
use std::cmp::Ordering;
use std::thread;
use std::time::Instant;

use cached::proc_macro::cached;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::git::git_types::{Commit, RefInfo};
use crate::git::queries::commit_calcs::get_commit_ids_between_commits2;
use crate::git::queries::commits_parsers::{PRETTY_FORMATTED, P_COMMITS, P_COMMIT_ROW, P_ID_LIST};
use crate::git::queries::refs::finish_initialising_refs_on_commits;
use crate::git::queries::stashes::load_stashes;
use crate::git::store::{load_commits_from_store, RwStore};
use crate::git::{run_git, RunGitOptions};
use crate::parser::parse_all;
use crate::server::git_request::{ReqCommitsOptions, ReqOptions};

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TopCommitOptions {
  pub repo_path: String,
  pub branch_name: String,
}

pub fn load_top_commit_for_branch(options: &TopCommitOptions, _: RwStore) -> Option<Commit> {
  let now = Instant::now();

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

  println!(
    "Took {}ms to request top commit from Git",
    now.elapsed().as_millis(),
  );

  parse_all(P_COMMIT_ROW, out?.as_str())
}

pub fn load_head_commit(options: &ReqOptions, _: RwStore) -> Option<Commit> {
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

pub fn load_commits_and_stashes(
  options: &ReqCommitsOptions,
  store_lock: RwStore,
) -> Option<Vec<Commit>> {
  let ReqCommitsOptions {
    repo_path,
    num_commits,
  } = options;

  let now = Instant::now();

  let p1 = repo_path.clone();
  let p2 = repo_path.clone();
  let num = num_commits.clone();

  let stashes_thread = thread::spawn(move || load_stashes(&p1));
  let commits_thread = thread::spawn(move || load_commits(&p2, num));

  let stashes = stashes_thread.join().unwrap();
  let mut commits = commits_thread.join().unwrap()?;

  println!(
    "Took {}ms to request stashes and commits from Git",
    now.elapsed().as_millis(),
  );

  if stashes.is_some() {
    commits.append(&mut stashes.unwrap());
  }

  commits.sort_by(|a, b| {
    if b.stash_id.is_some() || a.stash_id.is_some() {
      b.date.ms.partial_cmp(&a.date.ms).unwrap_or(Ordering::Equal)
    } else {
      Ordering::Equal
    }
  });

  for i in 0..commits.len() {
    let mut c = &mut commits[i];
    c.index = i;
  }

  let now = Instant::now();

  let commits = finish_initialising_refs_on_commits(commits, &store_lock);

  println!(
    "Took {}ms to get refs from commits *****",
    now.elapsed().as_millis(),
  );

  if let Ok(mut store) = store_lock.write() {
    (*store).commits.insert(repo_path.clone(), commits.clone());
  }

  // store_commits(&repo_path, &commits);

  Some(commits)
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

  println!(
    "Took {}ms to request {} commits from Git",
    now.elapsed().as_millis(),
    num
  );

  let now = Instant::now();
  let result = parse_all(P_COMMITS, &out);

  println!(
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

pub fn commit_ids_between_commits(
  options: &CommitDiffOpts,
  store_lock: RwStore,
) -> Option<Vec<String>> {
  let CommitDiffOpts {
    repo_path,
    commit_id1,
    commit_id2,
  } = options;

  if let Ok(store) = store_lock.read() {
    if let Some(commits) = (*store).commits.get(repo_path) {
      let commit_map: AHashMap<String, Commit> = commits
        .clone()
        .into_iter()
        .map(|c| (c.id.clone(), c))
        .collect();

      if let Some(result) = get_commit_ids_between_commits2(&commit_id2, &commit_id1, &commit_map) {
        return Some(result);
      }
    }
  }

  commit_ids_between_commits_inner(repo_path.clone(), commit_id1.clone(), commit_id2.clone())
}

#[cached(option = true, time = 1000)]
fn commit_ids_between_commits_inner(
  repo_path: String,
  commit_id1: String,
  commit_id2: String,
) -> Option<Vec<String>> {
  let now = Instant::now();

  let out = run_git(RunGitOptions {
    args: ["rev-list", &format!("{}..{}", commit_id1, commit_id2)],
    repo_path: &repo_path,
  })?;

  println!("Took {}ms to request ids", now.elapsed().as_millis());

  parse_all(P_ID_LIST, &out)
}

// Use this as a fallback when calculation fails.
pub fn get_un_pushed_commits(options: &ReqOptions, store: RwStore) -> Vec<String> {
  if let Some(ids) = get_un_pushed_commits_computed(&options, store) {
    println!("Computed ids: {:?}", ids);
    return ids;
  } else {
    #[cfg(debug_assertions)]
    println!("get_un_pushed_commits: Refs not found in commits, fall back to git request.")
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
fn get_un_pushed_commits_computed(options: &ReqOptions, store: RwStore) -> Option<Vec<String>> {
  let now = Instant::now();

  let commits = load_commits_from_store(&options.repo_path, &store)?;

  let commit_map: AHashMap<String, Commit> = commits
    .clone()
    .into_iter()
    .map(|c| (c.id.clone(), c))
    .collect();

  // let commit = commits.iter().find(|c| c.refs.iter().any(|r| r.head));

  // println!("{:?}", commit.unwrap());

  let head_ref = get_head_ref(&commits)?;
  let remote = find_sibling_ref(&head_ref, &commits)?;

  let result = get_commit_ids_between_commits2(&head_ref.commit_id, &remote.commit_id, &commit_map);

  println!(
    "get_un_pushed_commits_computed Took {}ms",
    now.elapsed().as_millis(),
  );

  result
}

fn get_head_ref(commits: &Vec<Commit>) -> Option<RefInfo> {
  Some(
    commits
      .iter()
      .find(|c| c.refs.iter().any(|r| r.head))?
      .refs
      .iter()
      .find(|r| r.head)?
      .clone(),
  )
}

fn find_sibling_ref(ri: &RefInfo, commits: &Vec<Commit>) -> Option<RefInfo> {
  if let Some(sibling_id) = &ri.sibling_id {
    return Some(
      commits
        .iter()
        .find(|c| c.refs.iter().any(|r| &r.id == sibling_id))?
        .refs
        .iter()
        .find(|r| &r.id == sibling_id)?
        .clone(),
    );
  }
  None
}
