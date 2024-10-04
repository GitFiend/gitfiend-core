use crate::f;
use crate::git::store::STORE;
use crate::server::request_util::{ES, R};
use std::collections::HashSet;
use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};

pub fn load_current_branch(repo_path: &str) -> R<(String, String)> {
  let repo = STORE.get_repo_path(repo_path)?;
  let head = repo.git_path.join("HEAD");

  if let Ok(text) = read_to_string(head) {
    return if let Some(branch) = text.split(':').last() {
      let id = branch.trim();
      let name = id.replace("refs/heads/", "");
      Ok((id.to_string(), name))
    } else {
      Err(ES::from(
        "Failed to load current branch. Failed to parse .git/HEAD. Could be a detached head?",
      ))
    };
  }

  Err(ES::from(
    "Failed to load current branch. Failed to read .git/HEAD",
  ))
}

// Some info
// Branches in git can contain /, but not \
// We read them from disk as / on Linux and Mac, but as \ on Windows.
pub fn read_refs(repo_path: &str, branch_name: &str) -> R<Refs> {
  let mut refs = Refs {
    local_id: None,
    remote_id: None,
    others: HashSet::new(),
  };

  let repo = STORE.get_repo_path(repo_path)?;
  let path = repo.git_path.join("refs");

  let heads_dir = path.join("heads");

  read_local_refs(&heads_dir, &heads_dir, branch_name, &mut refs)?;

  let remotes_dir = path.join("remotes");

  // Sometimes remotes folder doesn't exist.
  if let Ok(remotes) = read_dir(remotes_dir) {
    for item in remotes {
      let p = item?.path();
      let _ = read_remote_refs(&p, &p, branch_name, &mut refs);
    }
  }

  Ok(refs)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Refs {
  pub local_id: Option<String>,
  pub remote_id: Option<String>,
  pub others: HashSet<String>,
}

fn read_local_refs(
  current_path: &PathBuf,
  start_path: &PathBuf,
  branch_name: &str,
  refs_result: &mut Refs,
) -> R<()> {
  for item in read_dir(current_path)? {
    let path = item?.path();

    if path.is_dir() {
      return read_local_refs(&path, start_path, branch_name, refs_result);
    }

    let file_name = path.file_name().unwrap().to_str().unwrap();
    if !file_name.starts_with(".") && file_name != "HEAD" {
      let found_ref = get_ref_name_from_path(&path, start_path);
      if found_ref == branch_name {
        refs_result.local_id = Some(read_id_from_ref_file(&path)?);
      } else {
        refs_result.others.insert(found_ref);
      }
    }
  }

  Ok(())
}

fn get_ref_name_from_path(file: &Path, start_dir: &PathBuf) -> String {
  let ref_path = file.strip_prefix(start_dir).unwrap();

  ref_path
    .iter()
    .filter_map(|p| p.to_str())
    .collect::<Vec<&str>>()
    .join("/")
}

fn read_remote_refs(
  current_dir: &PathBuf,
  start_dir: &PathBuf,
  branch_name: &str,
  refs: &mut Refs,
) -> R<()> {
  for item in read_dir(current_dir)? {
    let p = item?.path();

    if p.is_dir() {
      return read_remote_refs(&p, start_dir, branch_name, refs);
    }

    let file_name = p.file_name().unwrap().to_str().unwrap();
    if file_name.starts_with('.') {
      continue;
    }
    if file_name == "HEAD" {
      let head_branch_full_name = read_head_file(&p)?;

      if head_branch_full_name.ends_with(branch_name) {
        refs.remote_id = refs.local_id.clone();
      }
    } else {
      let found = get_ref_name_from_path(&p, start_dir);
      if found == branch_name {
        refs.remote_id = read_id_from_ref_file(&p).ok();
      } else {
        refs.others.insert(found);
      }
    }
  }

  Ok(())
}

// E.g. "ref: refs/remotes/origin/develop"
fn read_head_file(head_path: &PathBuf) -> R<String> {
  let text = read_to_string(head_path)?;

  if let Some(i) = text.chars().position(|c| c == ':') {
    let path = &text[(i + 1)..];

    return Ok(path.trim().to_string());
  }

  Err(ES::from(&f!("Failed to parse {:?}", head_path)))
}

fn read_id_from_ref_file(file: &PathBuf) -> R<String> {
  Ok(read_to_string(file)?.trim().to_string())
}

#[cfg(test)]
mod tests {
  use crate::git::queries::workspace::load_current_branch::get_ref_name_from_path;
  use std::path::PathBuf;

  #[test]
  fn test_get_ref_name() {
    let start: PathBuf = ["aa", "bb"].iter().collect();
    let ref_parts: PathBuf = ["aa", "bb", "cc", "dd"].iter().collect();

    let res = get_ref_name_from_path(&ref_parts, &start);

    assert_eq!(res, "cc/dd")
  }
}
