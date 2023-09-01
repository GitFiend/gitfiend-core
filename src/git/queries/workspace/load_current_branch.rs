use crate::f;
use crate::server::request_util::R;
use std::fs::{read_dir, read_to_string};
use std::io;
use std::path::{Path, PathBuf};

pub fn load_current_branch(repo_path: &str) -> R<String> {
  let head = Path::new(repo_path).join(".git").join("HEAD");

  if let Ok(text) = read_to_string(head) {
    return if let Some(branch) = text.split(':').last() {
      Ok(branch.trim().to_string())
    } else {
      Err(
        "Failed to load current branch. Failed to parse .git/HEAD. Could be a detached head?"
          .to_string(),
      )
    };
  }

  Err("Failed to load current branch. Failed to read .git/HEAD".to_string())
}

pub fn read_refs(repo_path: &str, branch_name: &str) -> R<Refs> {
  let mut refs = Refs {
    local_id: String::new(),
    remote_id: String::new(),
    others: Vec::new(),
  };

  let root_path = Path::new(repo_path).join(".git");
  let branch = root_path.join(branch_name);
  let path = Path::new(repo_path).join(".git").join("refs");

  read_refs_inner(&path, &branch, &mut refs).map_err(|e| e.to_string())?;

  println!("{:?}", refs);

  Ok(refs)
}

#[derive(Debug)]
pub struct Refs {
  pub local_id: String,
  pub remote_id: String,
  pub others: Vec<PathBuf>,
}

fn read_refs_inner(
  refs_path: &PathBuf,
  branch_name: &PathBuf,
  refs_result: &mut Refs,
) -> io::Result<()> {
  for item in read_dir(refs_path)? {
    let path = item?.path();

    if path.is_file() {
      if path == *branch_name {
        refs_result.local_id = read_to_string(path)?.trim().to_string();
      } else {
        refs_result.others.push(path);
      }
    } else if path.is_dir() {
      read_refs_inner(&path, branch_name, refs_result)?;
    }
  }

  Ok(())
}
