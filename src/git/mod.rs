use std::process::Command;

pub(crate) mod git_types;
pub(crate) mod queries;

pub struct RunGitOptions {
  pub args: Vec<String>,
  pub repo_path: String,
}

pub fn run_git(options: RunGitOptions) -> String {
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
      args: vec!["--help".to_string()],
      repo_path: "/home/toby/Repos/vscode".to_string(),
    });

    println!("{}", text);
  }
}
