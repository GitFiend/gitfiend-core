use std::process::Command;

pub(crate) mod git_types;
pub(crate) mod queries;

pub struct RunGitOptions<'a, const COUNT: usize> {
  pub args: [&'a str; COUNT],
  pub repo_path: String,
}

pub fn run_git<const COUNT: usize>(options: RunGitOptions<COUNT>) -> String {
  let out = Command::new("git")
    .args(options.args)
    .current_dir(options.repo_path)
    .output()
    .expect("failed to execute process");

  String::from_utf8_lossy(&out.stdout).to_string()
}

#[cfg(test)]
mod tests {
  use crate::git::{run_git, RunGitOptions};

  #[test]
  fn test_run_git() {
    let text = run_git(RunGitOptions {
      args: ["--help"],
      repo_path: ".".to_string(),
    });

    assert!(text.len() > 0);
  }
}
