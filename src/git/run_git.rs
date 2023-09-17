use crate::dprintln;
use chardetng::EncodingDetector;
use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, Output};

use crate::git::git_settings::GIT_PATH;
use crate::server::request_util::R;

#[derive(Clone, Debug)]
pub struct RunGitOptions<'a, I, S>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  pub args: I,
  pub repo_path: &'a str,
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
    .output()?;

  let Output { stdout, stderr, .. } = &out;

  Ok(GitOut {
    stdout: read_buffer_to_string(stdout),
    stderr: read_buffer_to_string(stderr),
  })
}

fn read_buffer_to_string(bytes: &[u8]) -> String {
  let mut decoder = EncodingDetector::new();
  decoder.feed(bytes, true);
  let encoding = decoder.guess(None, true);
  let content = encoding.decode(bytes).0;

  content.into_owned()
}

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
    let text = run_git::run_git_err(RunGitOptions {
      args: ["--help"],
      repo_path: ".",
    });

    assert!(text.is_ok());
    assert!(!text.unwrap().stdout.is_empty());
  }

  #[test]
  fn test_git_path() {
    let p = Path::new("git");

    assert_eq!(p.to_str().unwrap(), "git");
  }
}
