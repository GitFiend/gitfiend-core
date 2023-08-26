use crate::git::git_types::{Commit, CommitInfo, RefInfo};
use crate::git::queries::commit_calcs::{
  find_commit_ancestors, get_commit_ids_between_commit_ids, get_commit_map_cloned,
};
use crate::git::queries::commit_filters::{apply_commit_filters, CommitFilter};
use crate::git::queries::commits_parsers::{PRETTY_FORMATTED, P_COMMITS, P_COMMIT_ROW, P_ID_LIST};
use crate::git::queries::refs::head_info::{calc_head_info, HeadInfo};
use crate::git::queries::refs::{finish_properties_on_refs, get_ref_info_from_commits};
use crate::git::queries::stashes::load_stashes;
use crate::git::run_git::{run_git, RunGitOptions};
use crate::git::store;
use crate::git::store::RepoPath;
use crate::parser::parse_all;
use crate::server::git_request::ReqOptions;
use crate::{dprintln, time_result};
use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::thread;
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TopCommitOptions {
  pub repo_path: String,
  pub branch_name: String,
}

pub fn load_top_commit_for_branch(options: &TopCommitOptions) -> Option<CommitInfo> {
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

  parse_all(P_COMMIT_ROW, out?.as_str())
}

pub fn load_head_commit(options: &ReqOptions) -> Option<CommitInfo> {
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
  pub repo_path: RepoPath,
  pub num_commits: u32,
  pub filters: Vec<CommitFilter>,
  pub fast: bool, // Fast means to use the cache only, don't run git command.
  pub skip_stashes: bool,
  // pub head_only: bool
}

pub fn load_commits_and_refs(options: &ReqCommitsOptions2) -> Option<(Vec<Commit>, Vec<RefInfo>)> {
  let ReqCommitsOptions2 {
    repo_path,
    num_commits,
    filters,
    fast,
    skip_stashes,
  } = options;

  let (commits, refs) = load_commits_unfiltered(repo_path, *num_commits, *fast, *skip_stashes)?;

  Some((
    apply_commit_filters(repo_path, commits, &refs, filters),
    refs,
  ))
}

fn get_commits_from_info(commit_info: Vec<CommitInfo>) -> Vec<Commit> {
  commit_info.into_iter().map(convert_commit).collect()
}

pub fn convert_commit(commit_info: CommitInfo) -> Commit {
  Commit {
    author: commit_info.author,
    email: commit_info.email,
    date: commit_info.date,
    id: commit_info.id,
    index: commit_info.index,
    parent_ids: commit_info.parent_ids,
    is_merge: commit_info.is_merge,
    message: commit_info.message,
    stash_id: commit_info.stash_id,
    refs: commit_info.refs.into_iter().map(|r| r.id).collect(),
    filtered: commit_info.filtered,
    num_skipped: commit_info.num_skipped,
  }
}

fn load_commits_unfiltered(
  repo_path: &RepoPath,
  num_commits: u32,
  cache_only: bool,
  skip_stashes: bool,
) -> Option<(Vec<Commit>, Vec<RefInfo>)> {
  if cache_only {
    if let Some(commits) = store::get_commits_and_refs(repo_path) {
      return Some(commits);
    }
  }

  let mut commits = if skip_stashes {
    load_commits(repo_path, num_commits)?
  } else {
    let p1 = repo_path.clone();
    let p2 = repo_path.clone();
    let num = num_commits;

    let stashes_thread = thread::spawn(move || load_stashes(&p1));
    let commits_thread = thread::spawn(move || load_commits(&p2, num));

    let stashes = stashes_thread.join().ok()?;
    let mut commits = commits_thread.join().ok()??;

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

    commits
  };

  for (i, c) in commits.iter_mut().enumerate() {
    c.index = i;
  }

  let refs = finish_properties_on_refs(get_ref_info_from_commits(&commits), repo_path);
  let commits = get_commits_from_info(commits);

  store::insert_commits(repo_path, &commits, &refs);

  Some((commits, refs))
}

pub fn load_commits(repo_path: &RepoPath, num: u32) -> Option<Vec<CommitInfo>> {
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

  time_result!(format!("parse commits. Length {}", out.len()), {
    parse_all(P_COMMITS, &out)
  })
}

#[derive(Debug, Deserialize, TS)]
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

  if let Some((commits, _)) = store::get_commits_and_refs(repo_path) {
    let commit_map: AHashMap<String, Commit> =
      commits.into_iter().map(|c| (c.id.clone(), c)).collect();

    if let Some(result) = get_commit_ids_between_commit_ids(commit_id2, commit_id1, &commit_map) {
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
  let out = time_result!("commit_ids_between_commits_fallback", {
    run_git(RunGitOptions {
      args: ["rev-list", &format!("{}..{}", commit_id1, commit_id2)],
      repo_path,
    })?
  });

  parse_all(P_ID_LIST, &out)
}

#[derive(Debug, Deserialize, TS)]
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

  if let Some((commits, _)) = store::get_commits_and_refs(repo_path) {
    let commits = get_commit_map_cloned(&commits);

    if let Some(commit) = commits.get(commit_id) {
      let ancestors = find_commit_ancestors(commit, &commits);

      return ancestors.contains(ancestor_candidate_id.as_str());
    }
  }

  false
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CommitOnBranchOpts {
  pub repo_path: String,
  pub commit_id: String,
}

pub fn commit_is_on_branch(options: &CommitOnBranchOpts) -> Option<bool> {
  let CommitOnBranchOpts {
    repo_path,
    commit_id,
  } = options;

  Some(
    get_all_commits_on_current_branch(&ReqOptions {
      repo_path: repo_path.clone(),
    })?
    .contains(commit_id),
  )
}

pub fn get_all_commits_on_current_branch(options: &ReqOptions) -> Option<HashSet<String>> {
  let ReqOptions { repo_path } = options;

  let HeadInfo {
    remote_commit,
    commit,
    ..
  } = calc_head_info(&ReqOptions {
    repo_path: repo_path.to_string(),
  })?;

  let (commits, _) = store::get_commits_and_refs(repo_path)?;
  let commits = get_commit_map_cloned(&commits);

  let mut ancestors: HashSet<String> = HashSet::new();

  if let Some(c) = remote_commit {
    ancestors.extend(
      find_commit_ancestors(&c, &commits)
        .iter()
        .map(|id| id.to_string()),
    );
    ancestors.insert(c.id);
  }

  ancestors.extend(
    find_commit_ancestors(&commit, &commits)
      .iter()
      .map(|id| id.to_string()),
  );
  ancestors.insert(commit.id);

  Some(ancestors)
}
