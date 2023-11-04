use crate::git::queries::config::config_file_parser::{
  parse_config_file, ConfigFile, ConfigSection, Row,
};
use crate::git::store::{RepoPath, STORE};
use crate::server::request_util::R;
use serde::Deserialize;
use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};
use ts_rs::TS;

const MAX_SCAN_DEPTH: u8 = 5;
const MAX_DIR_SIZE: usize = 50;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ScanOptions {
  pub repo_path: String,
  pub workspaces_enabled: bool,
}

pub fn scan_workspace(options: &ScanOptions) -> Vec<PathBuf> {
  let dir = PathBuf::from(&options.repo_path);

  let repo_paths = if !options.workspaces_enabled {
    scan_single_repo(dir)
  } else {
    let mut repo_paths: Vec<RepoPath> = Vec::new();
    scan_workspace_inner(dir, &mut repo_paths, 0);
    repo_paths
  };

  println!("repo_paths: {:?}", repo_paths);

  let result = repo_paths.iter().map(|r| r.path.clone()).collect();

  // We don't continue opening a repo if empty. Don't clobber REPO_PATHS
  if !repo_paths.is_empty() {
    STORE.set_repo_paths(repo_paths);
  }

  result
}

fn scan_single_repo(dir: PathBuf) -> Vec<RepoPath> {
  get_git_repo(&dir).into_iter().collect()

  // if is_git_repo(&dir) {
  //   vec![RepoPath {
  //     path: dir.clone(),
  //     git_path: dir.join(".git"),
  //     submodule: false,
  //   }]
  // } else {
  //   vec![]
  // }
}

fn scan_workspace_inner(dir: PathBuf, repo_paths: &mut Vec<RepoPath>, depth: u8) {
  if let Some(repo_path) = get_git_repo(&dir) {
    repo_paths.push(repo_path);
  }
  // if is_git_repo(&dir) {
  //   repo_paths.push(RepoPath {
  //     path: dir.clone(),
  //     git_path: dir.join(".git"),
  //     submodule: false,
  //   });
  // }

  if depth < MAX_SCAN_DEPTH {
    let entries = get_dir_entries(&dir);

    if entries.len() < MAX_DIR_SIZE {
      for e in entries {
        if e.is_dir() && !is_hidden(&e) {
          scan_workspace_inner(e, repo_paths, depth + 1);
        } else if depth == 0 && e.iter().any(|c| c == ".gitmodules") {
          if let Ok(submodules) = read_git_modules(&e) {
            println!("submodules: {:?}", submodules);
            repo_paths.extend(submodules);
            break;
          }
        }
      }
    }
  }
}

fn read_git_modules(file_path: &PathBuf) -> R<Vec<RepoPath>> {
  let text = read_to_string(file_path)?;
  let config: Vec<ConfigFile> = parse_config_file(&text)?;

  let mut submodules: Vec<RepoPath> = Vec::new();

  for c in config {
    if let ConfigFile::Section(section) = c {
      let ConfigSection(heading, rows) = section;
      if heading.0 == "submodule" {
        if let Some(Row::Data(_, path)) = rows.iter().find(|row| match row {
          Row::Data(path, _) => path == "path",
          Row::Other(_) => false,
        }) {
          submodules.push(RepoPath {
            path: file_path.clone(),
            git_path: file_path.join(path),
            submodule: true,
          })
        }
      }
    }
  }

  Ok(submodules)
}

fn get_dir_entries(dir: &PathBuf) -> Vec<PathBuf> {
  if let Ok(entries) = read_dir(dir) {
    let paths: Vec<PathBuf> = entries
      .filter(|e| e.is_ok())
      .map(|e| e.unwrap().path())
      .collect();

    return paths;
  }

  vec![]
}

fn get_git_repo(dir: &Path) -> Option<RepoPath> {
  if dir.is_dir() {
    let git_file_path = dir.join(".git");

    if git_file_path.is_file() {
      let text = read_to_string(&git_file_path).ok()?;
      let path = parse_submodule_git_file(&text)?;

      return Some(RepoPath {
        path: dir.to_path_buf(),
        git_path: dir.join(path).join(".git"),
        submodule: true,
      });
    }

    if git_file_path.exists() {
      return Some(RepoPath {
        path: dir.to_path_buf(),
        git_path: dir.join(".git"),
        submodule: false,
      });
    }
  }

  None
}

// // TODO: Doesn't check whether .git is a file or dir
// //
// // Check if it's a file? If so, read it so we can find the real dir.
// fn is_git_repo(dir: &Path) -> bool {
//   if dir.is_dir() {
//     let git_file_path = dir.join(".git");
//
//     if git_file_path.is_file() {
//       //
//     }
//
//     return git_file_path.exists();
//   }
//
//   false
// }

fn parse_submodule_git_file(text: &str) -> Option<String> {
  if let Some(i) = text.chars().position(|c| c == ':') {
    let path = &text[(i + 1)..];

    return Some(String::from(path.trim()));
  }
  None
}

fn is_hidden(entry: &Path) -> bool {
  if let Some(last) = entry.components().last() {
    return last.as_os_str().to_str().unwrap_or("").starts_with('.');
  }
  false
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_git_file() {
    let text = "gitdir: ../.git/modules/fiend-ui";

    let p = parse_submodule_git_file(text);

    assert!(p.is_some());
    assert_eq!(p.unwrap(), "../.git/modules/fiend-ui");
  }
}
