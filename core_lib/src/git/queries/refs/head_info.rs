use crate::git::git_request::ReqOptions;
use ahash::AHashMap;

use crate::git::git_types::{Commit, CommitInfo, RefInfo, RefLocation};
use crate::git::queries::commit_calcs::count_commits_between_fallback;
use crate::git::queries::commits::{
  convert_commit, load_head_commit, load_top_commit_for_branch, TopCommitOptions,
};
use crate::git::queries::config::GitConfig;
use crate::git::queries::refs::ref_diffs::calc_remote_ref_diffs;
use crate::git::request_util::{ES, R};
use crate::git::store::{PathString, CONFIG, STORE};

#[derive(Debug, Clone)]
pub struct HeadInfo {
  pub ref_info: RefInfo,
  pub commit: Commit,
  pub remote_ref: Option<RefInfo>,
  pub remote_commit: Option<Commit>,
  pub remote_ahead: u32,
  pub remote_behind: u32,
}

pub fn calc_head_info(options: &ReqOptions) -> R<HeadInfo> {
  let ReqOptions { repo_path } = options;

  let (commits, refs) = STORE
    .get_commits_and_refs(repo_path)
    .ok_or(ES::from("calc_head_info: No commits"))?;

  let head_info = calc_head_info_from_commits(commits, refs);

  if let Some(mut head_info) = head_info {
    if head_info.remote_ref.is_none() {
      if let Ok((remote_ahead, remote_commit, remote_behind, remote_ref)) =
        calc_remote_fallback(repo_path, &mut head_info.ref_info)
      {
        return Ok(HeadInfo {
          ref_info: head_info.ref_info.clone(),
          commit: head_info.commit.clone(),
          remote_ref: Some(remote_ref),
          remote_commit: Some(remote_commit),
          remote_ahead,
          remote_behind,
        });
      }
    }
    return Ok(head_info);
  } else if let Ok((mut head_commit, i)) = calc_head_fallback(repo_path) {
    let head_ref = &mut head_commit.refs[i];

    if let Ok((remote_ahead, remote_commit, remote_behind, remote_ref)) =
      calc_remote_fallback(repo_path, head_ref)
    {
      return Ok(HeadInfo {
        ref_info: head_ref.clone(),
        commit: convert_commit(head_commit),
        remote_ref: Some(remote_ref),
        remote_commit: Some(remote_commit),
        remote_ahead,
        remote_behind,
      });
    }
  }

  Err(ES::from("calc_head_info: Failed to get head info"))
}

// Returns Option intentionally.
fn calc_head_info_from_commits(
  commits: Vec<Commit>,
  refs: Vec<RefInfo>,
) -> Option<HeadInfo> {
  let all_refs: AHashMap<String, RefInfo> =
    refs.iter().map(|r| (r.id.clone(), r.clone())).collect();

  let commit_map = commits
    .into_iter()
    .map(|c| (c.id.clone(), c))
    .collect::<AHashMap<String, Commit>>();

  let mut remote_ahead = 0;
  let mut remote_behind = 0;

  for info in refs {
    if info.head {
      let mut remote_ref: Option<&RefInfo> = None;
      let mut remote_commit: Option<&Commit> = None;
      let commit = commit_map.get(&info.commit_id)?;

      if let Some(sibling_id) = &info.sibling_id {
        remote_ref = all_refs.get(sibling_id);
        if let Some(remote_ref) = remote_ref {
          remote_commit = commit_map.get(&remote_ref.commit_id);
        } else {
          // TODO: It may still exist, but not be part of our commit batch.
          // Maybe this is too unlikely? A user would need to commit 1000 times without pushing
          // for this to happen?
          // Unless they set a small number of commits to show in the future?
        }
      }

      if let Some(remote_ref) = remote_ref {
        let diffs_map = calc_remote_ref_diffs(&info.commit_id, &all_refs, &commit_map);

        if let Some(diffs) = diffs_map.get(&remote_ref.id) {
          remote_ahead = diffs.ahead_of_head;
          remote_behind = diffs.behind_head;
        }
      }

      return Some(HeadInfo {
        ref_info: info.clone(),
        commit: commit.clone(),
        remote_ref: remote_ref.cloned(),
        remote_commit: remote_commit.cloned(),
        remote_ahead,
        remote_behind,
      });
    }
  }

  None
}

// We return an index to the ref in the commit. This is so we can
// set sibling id later and not have separate instances of the same RefInfo
pub fn calc_head_fallback(repo_path: &str) -> R<(CommitInfo, usize)> {
  let commit = load_head_commit(&ReqOptions {
    repo_path: repo_path.to_string(),
  })?;

  let i = commit
    .refs
    .iter()
    .position(|r| r.head)
    .ok_or(ES::from("calc_head_fallback: Head index not found"))?;

  Ok((commit, i))
}

pub fn calc_remote_fallback(
  repo_path: &PathString,
  head_ref: &mut RefInfo,
) -> R<(u32, Commit, u32, RefInfo)> {
  let config = CONFIG.get_by_key(repo_path).unwrap_or_else(GitConfig::new);

  let remote_tracking_branch = config.get_tracking_branch_name(&head_ref.short_name);

  let mut remote_commit = load_top_commit_for_branch(&TopCommitOptions {
    repo_path: repo_path.to_string(),
    branch_name: remote_tracking_branch,
  })?;

  if let Some(remote_ref) = remote_commit
    .refs
    .iter_mut()
    .find(|r| r.short_name == head_ref.short_name && r.location == RefLocation::Remote)
  {
    head_ref.sibling_id = Some(remote_ref.id.to_string());
    remote_ref.sibling_id = Some(head_ref.id.to_string());

    let remote_ref = remote_ref.clone();

    let remote_ahead = count_commits_between_fallback(
      repo_path,
      &head_ref.full_name,
      &remote_ref.full_name,
    );

    let remote_behind = count_commits_between_fallback(
      repo_path,
      &remote_ref.full_name,
      &head_ref.full_name,
    );

    return Ok((
      remote_ahead,
      convert_commit(remote_commit),
      remote_behind,
      remote_ref,
    ));
  }

  Err(ES::from(
    "calc_remote_fallback: Didn't find remote ref in remote commit",
  ))
}
