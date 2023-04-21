use crate::git::git_types::{Commit, RefInfo, RefLocation, RefType};
use crate::git::queries::commit_calcs::get_commit_ids_between_commit_ids;
use crate::git::queries::commits_parsers::P_ID_LIST;
use crate::git::run_git::{run_git, RunGitOptions};
use crate::git::store;
use crate::parser::parse_all;
use crate::server::git_request::ReqOptions;
use crate::{dprintln, time_result};
use ahash::{AHashMap, AHashSet};
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct UnPushedCommits {
  // Commits that are un-pushed on this branch, but pushed on another.
  pub this_branch: Vec<String>,
  // Commits that haven't been pushed period. These have more edit options available.
  pub all_branches: Vec<String>,
}

pub fn get_un_pushed_commits(options: &ReqOptions) -> UnPushedCommits {
  if let Some(ids) = get_un_pushed_commits_computed(options) {
    let all = get_unique_un_pushed_commits(&options.repo_path, &ids).unwrap_or_default();

    return UnPushedCommits {
      this_branch: ids,
      all_branches: all,
    };
  } else {
    dprintln!("get_un_pushed_commits: Refs not found in commits, fall back to git request.");
  }

  if let Some(out) = run_git(RunGitOptions {
    repo_path: &options.repo_path,
    args: ["log", "HEAD", "--not", "--remotes", "--pretty=format:%H"],
  }) {
    if let Some(ids) = parse_all(P_ID_LIST, &out) {
      return UnPushedCommits {
        // This branch is probably far behind the remote.
        // TODO: Do we include all commits then?
        this_branch: Vec::new(),
        all_branches: ids,
      };
    }
  }

  UnPushedCommits {
    this_branch: Vec::new(),
    all_branches: Vec::new(),
  }
}

// Assumes head has some commits remote ref doesn't. If remote is ahead of ref then, could be misleading.
fn get_unique_un_pushed_commits(
  repo_path: &String,
  un_pushed_ids: &[String],
) -> Option<Vec<String>> {
  let (commits, refs) = store::get_commits_and_refs(repo_path)?;

  let un_pushed_ids: AHashSet<String> = un_pushed_ids.iter().cloned().collect();
  let ref_map: AHashMap<String, RefInfo> = refs.iter().map(|r| (r.id.clone(), r.clone())).collect();
  let commit_map: AHashMap<String, Commit> =
    commits.iter().map(|c| (c.id.clone(), c.clone())).collect();

  let head_ref = get_head_ref(&refs)?;
  let remote = find_sibling_ref(head_ref, &refs)?;
  let head = commits.iter().find(|c| c.id == head_ref.commit_id)?;

  let mut unique: Vec<String> = Vec::new();
  let mut checked: AHashSet<String> = AHashSet::new();

  un_pushed(
    head,
    &remote.commit_id,
    &commit_map,
    &ref_map,
    &un_pushed_ids,
    &mut checked,
    &mut unique,
  );

  Some(unique)
}

fn un_pushed(
  current: &Commit,
  remote_id: &str,
  commits: &AHashMap<String, Commit>,
  refs: &AHashMap<String, RefInfo>,
  un_pushed_ids: &AHashSet<String>,
  checked: &mut AHashSet<String>,
  unique: &mut Vec<String>,
) {
  if checked.contains(&*current.id) {
    return;
  }
  checked.insert(current.id.clone());

  if current.id == remote_id
    || current.refs.iter().any(|ref_id| {
      if let Some(r) = refs.get(ref_id) {
        r.ref_type == RefType::Branch && r.location == RefLocation::Remote
      } else {
        false
      }
    })
  {
    return;
  } else if un_pushed_ids.contains(&current.id) {
    unique.push(current.id.clone());
  }

  for id in &current.parent_ids {
    if let Some(commit) = commits.get(id) {
      un_pushed(
        commit,
        remote_id,
        commits,
        refs,
        un_pushed_ids,
        checked,
        unique,
      );
    }
  }
}

// This will return none if head ref or remote ref can't be found in provided commits.
fn get_un_pushed_commits_computed(options: &ReqOptions) -> Option<Vec<String>> {
  time_result!("get_un_pushed_commits_computed", {
    let (commits, refs) = store::get_commits_and_refs(&options.repo_path)?;

    let commit_map: AHashMap<String, Commit> =
      commits.into_iter().map(|c| (c.id.clone(), c)).collect();

    let head_ref = get_head_ref(&refs)?;
    let remote = find_sibling_ref(head_ref, &refs)?;

    get_commit_ids_between_commit_ids(&head_ref.commit_id, &remote.commit_id, &commit_map)
  })
}

fn get_head_ref(refs: &[RefInfo]) -> Option<&RefInfo> {
  refs.iter().find(|r| r.head)
}

fn find_sibling_ref<'a>(ri: &RefInfo, refs: &'a [RefInfo]) -> Option<&'a RefInfo> {
  if let Some(sibling_id) = &ri.sibling_id {
    return refs.iter().find(|r| &r.id == sibling_id);
  }
  None
}
