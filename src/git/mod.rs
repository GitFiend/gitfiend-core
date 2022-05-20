use crate::git::store::RwStore;
use crate::server::git_request::ReqOptions;
use std::env;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::process::{Command, Output, Stdio};

pub(crate) mod git_types;
pub(crate) mod queries;
pub(crate) mod store;

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
  let result = Command::new("git")
    .args(options.args)
    .current_dir(options.repo_path)
    .output();

  if let Ok(out) = result {
    let Output { stdout, stderr, .. } = &out;

    // TODO: Is stderr sometimes valid and useful git output?
    if stdout.len() > 0 {
      return Some(String::from_utf8_lossy(stdout).to_string());
    } else {
      println!("{:?}", String::from_utf8_lossy(stderr).to_string());
    }
  }

  None
}

///
/// We should probably use a separate function to the above run_get if we want progress.
/// TODO: unused/untested.
pub fn _run_git_with_progress<I, S>(options: RunGitOptions<I, S>)
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

pub fn args_with_config<I, S>(args: I) -> Vec<String>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let mut new_args = Vec::<String>::new();

  if let Some(config_args) = config_override_arg() {
    new_args.extend(config_args);
  }

  for a in args {
    if let Some(arg) = a.as_ref().to_str() {
      new_args.push(arg.to_string());
    }
  }

  new_args
}

fn config_override_arg() -> Option<[String; 2]> {
  match env::consts::OS {
    "windows" => Some([
      String::from("-c"),
      String::from("credential.helper=manager"),
    ]),
    "linux" => Some([String::from("-c"), String::from("credential.helper=store")]),
    _ => None,
  }
}

// Expect this to return none if Git is not installed.
pub fn git_version(_: &ReqOptions, _: RwStore) -> Option<String> {
  run_git(RunGitOptions {
    repo_path: ".",
    args: ["--version"],
  })
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
