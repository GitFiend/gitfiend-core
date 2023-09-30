use std::collections::HashSet;
use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};

use crate::server::request_util::{ES, R};

pub fn load_current_branch(repo_path: &str) -> R<(String, String)> {
  let head = Path::new(repo_path).join(".git").join("HEAD");

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

  let path = Path::new(repo_path).join(".git").join("refs");

  read_refs_inner(&path.join("heads"), branch_name, &mut refs, RefsType::Local)?;

  // Sometimes remotes folder doesn't exist.
  let _ = read_refs_inner(
    &path.join("remotes"),
    branch_name,
    &mut refs,
    RefsType::Remote,
  );

  Ok(refs)
}

#[derive(Debug, Clone, Copy)]
enum RefsType {
  Local,
  Remote,
}

#[derive(Debug)]
pub struct Refs {
  pub local_id: Option<String>,
  pub remote_id: Option<String>,
  pub others: HashSet<String>,
}

fn read_refs_inner(
  refs_path: &PathBuf,
  branch_name: &str,
  refs_result: &mut Refs,
  refs_type: RefsType,
) -> R<()> {
  for item in read_dir(refs_path)? {
    let path = item?.path();

    if path.is_file() {
      if path.to_str().unwrap_or("").ends_with(branch_name) {
        match refs_type {
          RefsType::Local => {
            refs_result.local_id = Some(read_to_string(path)?.trim().to_string());
          }
          RefsType::Remote => {
            refs_result.remote_id = Some(read_to_string(path)?.trim().to_string());
          }
        }
      } else {
        refs_result.others.insert(
          PathBuf::from(&path.strip_prefix(refs_path)?)
            .to_str()
            .unwrap_or("")
            .to_string(),
        );
      }
    } else if path.is_dir() {
      read_refs_inner(&path, branch_name, refs_result, refs_type)?;
    }
  }

  Ok(())
}
