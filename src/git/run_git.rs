use crate::dprintln;
use bstr::BString;
use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, Output};

use crate::git::git_settings::GIT_PATH;
use crate::server::request_util::{to_r, R};

#[derive(Clone, Debug)]
pub struct RunGitOptions<'a, I, S>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  pub args: I,
  pub repo_path: &'a str,
}

pub fn run_git<I, S>(options: RunGitOptions<I, S>) -> Option<String>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let result = Command::new(Path::new(GIT_PATH.as_path()))
    .args(options.args)
    .current_dir(options.repo_path)
    .output();

  if let Ok(out) = result {
    let Output { stdout, stderr, .. } = &out;

    // TODO: Is stderr sometimes valid and useful git output?
    if !stdout.is_empty() {
      return Some(String::from_utf8_lossy(stdout).to_string());
    } else if !stderr.is_empty() {
      dprintln!("StdErr: {:?}", String::from_utf8_lossy(stderr).to_string());
    }
  }

  None
}

pub struct GitOut {
  pub stdout: String,
  pub stderr: String,
}

pub fn run_git_err<I, S>(options: RunGitOptions<I, S>) -> R<GitOut>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let out = Command::new(Path::new(GIT_PATH.as_path()))
    .args(options.args)
    .current_dir(options.repo_path)
    .output()
    .map_err(|e| e.to_string())?;

  let Output { stdout, stderr, .. } = &out;

  Ok(GitOut {
    stdout: String::from_utf8_lossy(stdout).to_string(),
    stderr: String::from_utf8_lossy(stderr).to_string(),
  })
}

pub struct GitOut2 {
  pub stdout: BString,
  pub stderr: BString,
}

pub fn run_git_bstr<I, S>(options: RunGitOptions<I, S>) -> R<GitOut2>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let out = Command::new(Path::new(GIT_PATH.as_path()))
    .args(options.args)
    .current_dir(options.repo_path)
    .output()
    .map_err(to_r)?;

  let Output { stdout, stderr, .. } = out;

  Ok(GitOut2 {
    stdout: BString::from(stdout),
    stderr: BString::from(stderr),
  })
}

// TODO: Tidy this up.
pub fn run_git_buffer<I, S>(options: RunGitOptions<I, S>) -> Option<Vec<u8>>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let result = Command::new(Path::new(GIT_PATH.as_path()))
    .args(options.args)
    .current_dir(options.repo_path)
    .output();

  if let Ok(out) = result {
    let Output { stdout, stderr, .. } = out;

    if !stdout.is_empty() {
      return Some(stdout);
    } else if !stderr.is_empty() {
      dprintln!("StdErr: {:?}", String::from_utf8_lossy(&stderr).to_string());
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use crate::git::run_git;
  use crate::git::run_git::RunGitOptions;

  #[test]
  fn test_run_git() {
    let text = run_git::run_git(RunGitOptions {
      args: ["--help"],
      repo_path: ".",
    });

    assert!(text.is_some());
    assert!(!text.unwrap().is_empty());
  }

  #[test]
  fn test_git_path() {
    let p = Path::new("git");

    assert_eq!(p.to_str().unwrap(), "git");
  }
}
