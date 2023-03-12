use ahash::{AHashMap, AHashSet};
use serde::Deserialize;
use ts_rs::TS;

use crate::git::git_types::{Commit, RefInfo};
use crate::git::queries::commit_calcs::{find_commit_ancestors, get_commit_map_cloned};
use crate::git::queries::patches::patches::load_patches;

#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum CommitFilter {
  Branch { id: String, short_name: String },
  User { author: String, email: String },
  Commit { commit_id: String },
  File { file_name: String },
}

pub fn apply_commit_filters(
  repo_path: &str,
  commits: Vec<Commit>,
  refs: &[RefInfo],
  filters: &[CommitFilter],
) -> Vec<Commit> {
  let commit_map = get_commit_map_cloned(&commits);

  let results: Vec<AHashSet<&str>> = filters
    .iter()
    .map(|filter| match filter {
      CommitFilter::Branch { short_name, .. } => {
        get_all_commits_with_branch_name(short_name, &commit_map, refs)
      }
      CommitFilter::User { author, .. } => get_commits_for_user(author, &commits),
      CommitFilter::Commit { commit_id } => [commit_id.as_str()].into_iter().collect(),
      CommitFilter::File { file_name } => {
        if let Some(patches) = load_patches(repo_path, &commits) {
          return commits
            .iter()
            .filter(|c| {
              if let Some(files) = patches.get(&c.id) {
                return files
                  .iter()
                  .any(|p| p.new_file == *file_name || p.old_file == *file_name);
              }

              false
            })
            .map(|c| c.id.as_str())
            .collect();
        }
        AHashSet::new()
      }
    })
    .collect();

  let ids: AHashSet<String> = commits
    .iter()
    .filter(|c| results.iter().all(|r| r.contains(c.id.as_str())))
    .map(|c| c.id.clone())
    .collect();

  let mut remaining_commits: Vec<Commit> = Vec::new();
  let mut skipped = 0;
  let mut index = 0;

  for c in commits.iter() {
    if ids.contains(c.id.as_str()) {
      let mut c = c.clone();

      c.index = index;
      index += 1;

      c.num_skipped = skipped;
      skipped = 0;
      remaining_commits.push(c);
    } else {
      skipped += 1;
    }
  }

  remaining_commits
}

fn get_all_commits_with_branch_name<'a>(
  short_name: &str,
  commits: &'a AHashMap<String, Commit>,
  refs: &[RefInfo],
) -> AHashSet<&'a str> {
  let mut ids_to_keep = AHashSet::<&'a str>::new();

  refs
    .iter()
    .filter(|r| r.short_name == short_name)
    .flat_map(|r| commits.get(&r.commit_id))
    .for_each(|c| {
      if !ids_to_keep.contains(c.id.as_str()) {
        let ancestors = find_commit_ancestors(c, commits);

        ids_to_keep.insert(c.id.as_str());
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
