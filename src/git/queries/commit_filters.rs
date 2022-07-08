use ahash::{AHashMap, AHashSet};
use serde::Deserialize;
use ts_rs::TS;

use crate::git::git_types::Commit;
use crate::git::queries::commit_calcs::{find_commit_ancestors, get_commit_map_cloned};

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
  let commit_map = get_commit_map_cloned(&commits);

  for filter in filters {
    match filter {
      CommitFilter::Branch { short_name, .. } => {
        let ids = get_all_commits_with_branch_name(short_name, &commit_map);

        commits = commits
          .into_iter()
          .filter(|c| ids.contains(c.id.as_str()))
          .collect();
      }
      CommitFilter::User { author, email } => {
        let ids = get_commits_for_user(author, &commits);
      }
      CommitFilter::Commit { commit_id } => {
        let ids: AHashSet<&str> = [commit_id.as_str()].into_iter().collect();
      }
      CommitFilter::File { file_name } => {
        //
      }
    };
  }

  commits
}

fn get_all_commits_with_branch_name<'a>(
  short_name: &str,
  commits: &'a AHashMap<String, Commit>,
) -> AHashSet<&'a str> {
  let mut ids_to_keep = AHashSet::<&'a str>::new();

  commits
    .iter()
    .filter(|(_, c)| c.refs.iter().any(|r| r.short_name == short_name))
    .for_each(|(id, c)| {
      if !ids_to_keep.contains(id.as_str()) {
        let ancestors = find_commit_ancestors(c, commits);

        ids_to_keep.extend(ancestors);
      }
    });

  // We include any stashes that have one of our commits as a parent.
  for (id, c) in commits {
    if c.stash_id.is_some()
      && c
        .parent_ids
        .iter()
        .any(|id| ids_to_keep.contains(id.as_str()))
    {
      ids_to_keep.insert(id);
    }
  }

  ids_to_keep
}

fn get_commits_for_user<'a>(author: &str, commits: &'a [Commit]) -> AHashSet<&'a str> {
  commits
    .iter()
    .filter(|c| c.author == author)
    .map(|c| c.id.as_str())
    .collect()
}
