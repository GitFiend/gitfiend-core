use ahash::{AHashMap, AHashSet};

use crate::git::git_types::Commit;
use crate::git::run_git::{run_git, RunGitOptions};
use crate::git::store::REF_DIFFS;

fn _get_commit_map(commits: &[Commit]) -> AHashMap<&String, &Commit> {
  commits.iter().map(|c| (&c.id, c)).collect()
}

pub fn get_commit_map_cloned(commits: &[Commit]) -> AHashMap<String, Commit> {
  commits.iter().map(|c| (c.id.clone(), c.clone())).collect()
}

pub fn find_commit_ancestors<'a>(
  commit: &'a Commit,
  commits: &'a AHashMap<String, Commit>,
) -> AHashSet<&'a str> {
  let mut ancestors = AHashSet::<&'a str>::new();
  let mut ancestor_commits: Vec<&Commit> = vec![commit];

  while !ancestor_commits.is_empty() {
    if let Some(c) = ancestor_commits.pop() {
      for id in c.parent_ids.iter() {
        if !ancestors.contains(id as &str) {
          ancestors.insert(id);
          if let Some(parent) = commits.get(id) {
            ancestor_commits.push(parent);
          }
        }
      }
    }
  }

  ancestors
}

// impl Global<AHashMap<String, u32>> {
//   fn get_diff(&self, key: &str) -> Option<u32> {
//     if let Ok(diffs) = REF_DIFFS.read() {
//       return Some(*diffs.get(key)?);
//     }
//     None
//   }
// }

// How many commits ahead is a. The order matters.
pub fn count_commits_between_commit_ids(
  a_id: &String,
  b_id: &String,
  commits: &AHashMap<String, Commit>,
) -> u32 {
  let key = format!("{}{}", a_id, b_id);

  if let Ok(count) = REF_DIFFS.read() {
    if let Some(count) = count.get(&key) {
      return *count;
    }
  }

  // if let Some(count) = REF_DIFFS.get_diff(&key) {
  //   return count;
  // }

  if let Some(a) = commits.get(a_id) {
    if let Some(b) = commits.get(b_id) {
      if a.id == b.id {
        return 0;
      }

      let mut num = 0;

      let mut a_ancestors = find_commit_ancestors(a, commits);
      a_ancestors.insert(&a.id);

      let mut b_ancestors = find_commit_ancestors(b, commits);
      b_ancestors.insert(&b.id);

      for id in a_ancestors.into_iter() {
        if !b_ancestors.contains(&id) {
          num += 1;
        }
      }

      if let Ok(mut diffs) = REF_DIFFS.write() {
        diffs.insert(key, num);
      }
      // REF_DIFFS.insert(key, num);

      return num;
    }
  }

  0
}

// How many commits ahead is a. The order matters.
pub fn get_commit_ids_between_commit_ids(
  a_id: &String,
  b_id: &String,
  commits: &AHashMap<String, Commit>,
) -> Option<Vec<String>> {
  let a = commits.get(a_id)?;
  let b = commits.get(b_id)?;

  Some(get_commit_ids_between_commits(a, b, commits))
}

// How many commits ahead is a. The order matters.
fn get_commit_ids_between_commits(
  a: &Commit,
  b: &Commit,
  commits: &AHashMap<String, Commit>,
) -> Vec<String> {
  let mut ids: Vec<String> = Vec::new();

  if a.id == b.id {
    return ids;
  }

  let mut a_ancestors = find_commit_ancestors(a, commits);
  a_ancestors.insert(&a.id);

  let mut b_ancestors = find_commit_ancestors(b, commits);
  b_ancestors.insert(&b.id);

  for id in a_ancestors.into_iter() {
    if !b_ancestors.contains(&id) {
      ids.push(id.to_string());
    }
  }

  ids
}

pub fn count_commits_between_fallback(repo_path: &str, commit_id1: &str, commit_id2: &str) -> u32 {
  let out = run_git(RunGitOptions {
    args: [
      "rev-list",
      &format!("{}..{}", commit_id1, commit_id2),
      "--count",
    ],
    repo_path,
  });

  if let Some(out) = out {
    return out.trim().parse::<u32>().ok().unwrap_or(0);
  }

  0
}

#[cfg(test)]
mod tests {
  use crate::git::store::REF_DIFFS;

  #[test]
  fn test_ref_diffs() {
    if let Ok(mut diffs) = REF_DIFFS.write() {
      diffs.insert("OMG".to_string(), 1);
    }
    // REF_DIFFS.insert("OMG".to_string(), 1);

    assert!(REF_DIFFS.read().unwrap().get("OMG").is_some());
    // assert!(REF_DIFFS.get_diff("OMG").is_some());
  }
}
