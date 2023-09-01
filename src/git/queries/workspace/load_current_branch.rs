use crate::server::request_util::R;
use std::fs::read_dir;
use std::io;
use std::path::Path;

pub fn load_current_branch(repo_path: &str) -> R<String> {
  let head = Path::new(repo_path).join(".git").join("HEAD");

  if let Ok(text) = std::fs::read_to_string(head) {
    return if let Some(branch) = text.split("/").last() {
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

pub fn read_refs(repo_path: &str, branch_name: &str) -> io::Result<()> {
  let refs = Path::new(repo_path).join(".git").join("refs").join("heads");

  let mut branches = Vec::<String>::new();

  for item in read_dir(refs)? {
    let path = item?.path();

    if path.is_file() {
      if let Some(file_name) = path.file_name() {
        if file_name == branch_name {
          //
        } else {
          // branches.push(file_name.to_str().to_string());
        }
      }
    }
  }

  Ok(())
}
