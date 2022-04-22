use crate::git::git_types::Commit;
use crate::git::queries::commits_parsers::{PRETTY_FORMATTED, P_COMMITS, P_COMMIT_ROW};
use crate::git::queries::stashes::load_stashes;
use crate::git::{run_git, RunGitOptions};
use crate::parser::parse_all;
use crate::server::git_request::{ReqCommitsOptions, ReqOptions};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::thread;
use std::time::Instant;
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TopCommitOptions {
  pub repo_path: String,
  pub branch_name: String,
}

pub fn load_top_commit_for_branch(options: &TopCommitOptions) -> Option<Commit> {
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

pub fn load_commits_and_stashes(options: &ReqCommitsOptions) -> Option<Vec<Commit>> {
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
  });

  println!(
    "Took {}ms to request {} commits from Git",
    now.elapsed().as_millis(),
    num
  );

  let now = Instant::now();
  let result = parse_all(P_COMMITS, out?.as_str());

  println!(
    "Took {}ms to parse {} commits",
    now.elapsed().as_millis(),
    num
  );

  result
}
