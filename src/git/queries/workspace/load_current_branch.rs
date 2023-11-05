use crate::git::store::STORE;
use std::collections::HashSet;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

use crate::server::request_util::{ES, R};

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

pub fn read_refs(repo_path: &str, branch_name: &str) -> R<Refs> {
  let mut refs = Refs {
    local_id: None,
    remote_id: None,
    others: HashSet::new(),
  };

  let repo = STORE.get_repo_path(repo_path)?;
  let path = repo.git_path.join("refs");

  // We convert to a path so we aren't comparing / with \ on Windows.
  let branch_name_path = PathBuf::from(branch_name);

  read_local_refs(
    &path.join("heads"),
    &path.join("heads"),
    &branch_name_path,
    &mut refs,
  )?;

  // Sometimes remotes folder doesn't exist.
  let _ = read_remote_refs(
    &path.join("remotes"),
    &path.join("remotes"),
    &branch_name_path,
    &mut refs,
  );

  Ok(refs)
}

#[derive(Debug)]
pub struct Refs {
  pub local_id: Option<String>,
  pub remote_id: Option<String>,
  pub others: HashSet<String>,
}

fn read_local_refs(
  current_path: &PathBuf,
  start_path: &PathBuf,
  branch_name: &PathBuf,
  refs_result: &mut Refs,
) -> R<()> {
  for item in read_dir(current_path)? {
    let path = item?.path();

    if path.is_file() {
      if path.ends_with(branch_name) {
        refs_result.local_id = Some(read_to_string(path)?.trim().to_string());
      } else {
        refs_result.others.insert(
          path
            .strip_prefix(start_path)?
            .to_str()
            .unwrap_or("")
            .to_string(),
        );
      }
    } else if path.is_dir() {
      read_local_refs(&path, start_path, branch_name, refs_result)?;
    }
  }

  Ok(())
}

fn read_remote_refs(
  current_path: &PathBuf,
  start_path: &PathBuf,
  branch_name: &PathBuf,
  refs_result: &mut Refs,
) -> R<()> {
  for item in read_dir(current_path)? {
    let path = item?.path();

    if path.is_file() {
      if path.ends_with(branch_name) {
        refs_result.remote_id = Some(read_to_string(path)?.trim().to_string());
      } else {
        let p: PathBuf = path.strip_prefix(start_path)?.iter().skip(1).collect();
        let name = p.to_str().unwrap_or("").to_string();

        if name == "HEAD" {
          if PathBuf::from(read_to_string(path)?).ends_with(branch_name) {
            refs_result.remote_id = refs_result.local_id.clone();
          }
        } else if !name.starts_with('.') {
          refs_result.others.insert(name);
        }
      }
    } else if path.is_dir() {
      read_remote_refs(&path, start_path, branch_name, refs_result)?;
    }
  }

  Ok(())
}
