use crate::f;
use crate::git::store::STORE;
use std::collections::HashSet;
use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};

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

#[derive(Debug)]
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

    match read_ref_path(&path, start_path) {
      PathRef::Dir => read_local_refs(&path, start_path, branch_name, refs_result)?,
      PathRef::Ref(name) => {
        if name == branch_name {
          refs_result.local_id = Some(read_to_string(path)?.trim().to_string());
        } else {
          refs_result.others.insert(name);
        }
      }
      PathRef::Hidden | PathRef::Head | PathRef::Unknown => {}
    }
  }

  Ok(())
}

fn read_remote_refs(
  current_path: &PathBuf,
  start_path: &PathBuf,
  branch_name: &str,
  refs_result: &mut Refs,
) -> R<()> {
  for item in read_dir(current_path)? {
    let path = item?.path();

    match read_ref_path(&path, start_path) {
      PathRef::Dir => read_remote_refs(&path, start_path, branch_name, refs_result)?,
      PathRef::Ref(name) => {
        if name == branch_name {
          refs_result.remote_id = Some(read_to_string(path)?.trim().to_string());
        } else {
          refs_result.others.insert(name);
        }
      }
      PathRef::Head => {
        if read_head_file(&path)?.ends_with(branch_name) {
          refs_result.remote_id = refs_result.local_id.clone();
        }
      }
      PathRef::Hidden | PathRef::Unknown => {}
    }
  }

  Ok(())
}

enum PathRef {
  Dir,
  Hidden,
  Head,
  Ref(String),
  Unknown,
}

fn read_ref_path(path: &Path, root_path: &PathBuf) -> PathRef {
  if path.is_dir() {
    PathRef::Dir
  } else if path.is_file() {
    path
      .file_name()
      .and_then(|name| name.to_str())
      .map(|name| {
        if name.starts_with('.') {
          PathRef::Hidden
        } else if name == "HEAD" {
          PathRef::Head
        } else {
          if let Ok(ref_path) = path.strip_prefix(root_path) {
            return PathRef::Ref(
              ref_path
                .components()
                .map(|component| component.as_os_str().to_string_lossy().into_owned())
                .collect::<Vec<String>>()
                .join("/"),
            );
          }

          PathRef::Unknown
        }
      })
      .unwrap_or(PathRef::Unknown)
  } else {
    PathRef::Unknown
  }
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
