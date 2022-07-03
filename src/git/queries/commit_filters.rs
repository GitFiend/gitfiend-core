use ahash::AHashMap;
use serde::Deserialize;
use ts_rs::TS;

use crate::git::git_types::{Commit, RefInfo};
use crate::git::queries::commits::find_sibling_ref;

#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum CommitFilter {
  Branch { id: String, short_name: String },
  User { author: String, email: String },
  Commit { commit_id: String },
  File { file_name: String },
}

pub fn apply_commit_filters(mut commits: Vec<Commit>, filters: &Vec<CommitFilter>) -> Vec<Commit> {
  let unfiltered_commits = commits.clone();

  let commit_map: AHashMap<String, Commit> = commits
    .clone()
    .into_iter()
    .map(|c| (c.id.clone(), c))
    .collect();

  for filter in filters {
    match filter {
      CommitFilter::Branch { id, short_name } => {
        if let Some(highest_ref) = get_highest_sibling_ref(&unfiltered_commits, short_name) {
          if let Some(c) = commit_map.get(&highest_ref.commit_id) {
            //
          }
        }
      }
      CommitFilter::User { author, email } => {}
      CommitFilter::Commit { commit_id } => {}
      CommitFilter::File { file_name } => {}
    };
  }

  commits
}

// TODO: This is probably buggy when there are multiple remotes.
// We should just look up the id of the given ref and then look up the sibling.
// We then compare the index.
// TODO: Consider how this should work with multiple remotes. Show all with the same short_name?
fn get_highest_sibling_ref<'a>(
  unfiltered_commits: &'a Vec<Commit>,
  short_name: &str,
) -> Option<&'a RefInfo> {
  let mut highest_ref: Option<(&Commit, &RefInfo)> = None;

  for c in unfiltered_commits {
    for r in c.refs.iter() {
      if r.short_name == short_name {
        if let Some(highest) = &highest_ref {
          if c.index < highest.0.index {
            highest_ref = Some((c, r));
          }
        } else {
          highest_ref = Some((c, r));
        }
      }
    }
  }

  Some(highest_ref?.1)
}

// fn get_highest_sibling_ref2<'a>(
//   unfiltered_commits: &'a Vec<Commit>,
//   commits_map: &AHashMap<String, Commit>,
//   id: &str,
// ) -> Option<&'a RefInfo> {
//   let given_ref: &RefInfo = unfiltered_commits
//     .iter()
//     .find(|c| c.refs.iter().any(|r| &r.id == id))?
//     .refs
//     .iter()
//     .find(|r| &r.id == id)?;
//
//   if let Some(sibling) = find_sibling_ref(given_ref, unfiltered_commits) {
//     if let Some(commit) = commits_map.get(&sibling.commit_id) {
//       if commit.index < g
//     }
//   }
//
//   Some(given_ref)
// }
