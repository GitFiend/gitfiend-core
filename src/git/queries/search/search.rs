use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use crate::git::{args_with_config, RunGitOptions};

pub fn run_git_with_progress<I, S>(options: RunGitOptions<I, S>) -> Option<String>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let mut cmd = Command::new("git")
    .args(args_with_config(options.args))
    .current_dir(options.repo_path)
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

  let mut lines: Vec<String> = Vec::new();

  {
    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();

    for line in stdout_lines {
      if let Ok(line) = line {
        lines.push(line);
      }
      // println!("{:?}", line);
    }

    println!("Done!");
  }

  println!("waiting for exit");
  cmd.wait().unwrap();

  Some(lines.join(""))
}

#[cfg(test)]
mod tests {
  use crate::git::queries::search::search::run_git_with_progress;
  use crate::git::RunGitOptions;

  #[test]
  fn test_get_command() {
    let result = run_git_with_progress(RunGitOptions {
      repo_path: ".",
      args: [
        "log",
        "-S",
        "this",
        "--name-status",
        "--pretty=format:%H,",
        &format!("-n{}", 10),
        "-z",
      ],
    });

    assert!(result.is_some());
    println!("{:?}", result);
  }
}
