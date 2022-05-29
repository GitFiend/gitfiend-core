use crate::git::git_types::Commit;
use std::collections::{HashMap, HashSet};

fn find_commit_ancestors(commit: &Commit, commits: &HashMap<String, Commit>) -> HashSet<String> {
  let mut ancestors = HashSet::<String>::new();
  let mut ancestor_commits: Vec<&Commit> = vec![&commit];

  while ancestor_commits.len() > 0 {
    if let Some(c) = ancestor_commits.pop() {
      for id in c.parent_ids.iter() {
        if !ancestors.contains(id) {
          ancestors.insert(id.clone());
          if let Some(parent) = commits.get(id) {
            ancestor_commits.push(parent);
          }
        }
      }
    }
  }

  ancestors
}

pub fn count_commits_between_commit_ids(
  a_id: &String,
  b_id: &String,
  commits: &HashMap<String, Commit>,
) -> u32 {
  if let Some(ids) = get_commit_ids_between_commits2(a_id, b_id, commits) {
    ids.len() as u32
  } else {
    0
  }
}

// How many commits ahead is a. The order matters.
pub fn get_commit_ids_between_commits2(
  a_id: &String,
  b_id: &String,
  commits: &HashMap<String, Commit>,
) -> Option<Vec<String>> {
  // let commit_map: HashMap<String, Commit> = commits
  //   .clone()
  //   .into_iter()
  //   .map(|c| (c.id.clone(), c))
  //   .collect();

  let a = commits.get(a_id)?;
  let b = commits.get(b_id)?;

  Some(get_commit_ids_between_commits(a, b, commits))
}

// How many commits ahead is a. The order matters.
fn get_commit_ids_between_commits(
  a: &Commit,
  b: &Commit,
  commits: &HashMap<String, Commit>,
) -> Vec<String> {
  let mut ids: Vec<String> = Vec::new();

  if a.id == b.id {
    return ids;
  }

  let mut a_ancestors = find_commit_ancestors(&a, &commits);
  a_ancestors.insert(a.id.clone());

  let mut b_ancestors = find_commit_ancestors(&b, &commits);
  b_ancestors.insert(b.id.clone());

  for id in a_ancestors.into_iter() {
    if !b_ancestors.contains(&id) {
      ids.push(id);
    }
  }

  ids
}
