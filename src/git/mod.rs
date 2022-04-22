use std::io::{BufRead, BufReader};
use std::process::{Command, Output, Stdio};

pub(crate) mod git_types;
pub(crate) mod queries;

#[derive(Clone, Debug)]
pub struct RunGitOptions<'a, const COUNT: usize> {
  pub args: [&'a str; COUNT],
  pub repo_path: &'a String,
}

pub fn run_git<const COUNT: usize>(options: RunGitOptions<COUNT>) -> Option<String> {
  let out = Command::new("git")
    .args(options.args)
    .current_dir(options.repo_path)
    .output();

  if out.is_ok() {
    let Output { stdout, stderr, .. } = &out.unwrap();

    // TODO: Is stderr sometimes valid and useful git output?
    if stdout.len() > 0 {
      Some(String::from_utf8_lossy(stdout).to_string())
    } else {
      println!("{:?}", String::from_utf8_lossy(stderr).to_string());
      None
    }
  } else {
    None
  }
}

// We should probably use a separate function to the above run_get if we want progress.
// TODO: unused/untested.
pub fn _run_git_with_progress<const COUNT: usize>(options: RunGitOptions<COUNT>) {
  let mut cmd = Command::new("git")
    .args(options.args)
    .current_dir(options.repo_path)
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

  {
    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();

    for line in stdout_lines {
      println!("{:?}", line);
    }
  }

  cmd.wait().unwrap();
}

#[cfg(test)]
mod tests {
  use crate::git::{run_git, RunGitOptions};

  #[test]
  fn test_run_git() {
    let text = run_git(RunGitOptions {
      args: ["--help"],
      repo_path: &".".to_string(),
    });

    assert!(text.is_some());
    assert!(text.unwrap().len() > 0);
  }
}
