use crate::git::git_types::{Commit, RefInfo};
use crate::git::queries::commit_calcs::get_commit_ids_between_commit_ids;
use crate::git::queries::commits_parsers::P_ID_LIST;
use crate::git::run_git::{run_git, RunGitOptions};
use crate::git::store;
use crate::parser::parse_all;
use crate::server::git_request::ReqOptions;
use crate::{dprintln, time_result};
use ahash::AHashMap;

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
