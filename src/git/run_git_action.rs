use crate::git::git_settings::GIT_PATH;
use crate::git::git_version::GitVersion;
use crate::git::store::ACTION_LOGS;
use crate::server::git_request::ReqOptions;
use serde::Serialize;
use std::env;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio};
use ts_rs::TS;

#[derive(Clone, Debug)]
pub struct RunGitActionOptions<'a, I, S>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  pub args: I,
  pub repo_path: &'a str,
  pub git_version: GitVersion,
}

pub struct ActionResult {
  pub stdout: Vec<String>,
  pub stderr: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum ActionProgress {
  Out(String),
  Err(String),
}

pub fn run_git_action<I, S>(options: RunGitActionOptions<I, S>) -> Option<ActionResult>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let mut cmd = Command::new(GIT_PATH.as_path())
    .args(args_with_config(options.args, options.git_version))
    .current_dir(options.repo_path)
    .stdout(Stdio::piped())
    .spawn()
    .ok()?;

  let mut lines: Vec<String> = Vec::new();

  let stdout = cmd.stdout.as_mut()?;
  let stdout_reader = BufReader::new(stdout);
  let stdout_lines = stdout_reader.lines();

  for line in stdout_lines.flatten() {
    ACTION_LOGS.push(ActionProgress::Out(line.clone()));
    println!("{}", line);

    lines.push(line);
  }

  cmd.wait().ok()?;

  let mut stderr = String::new();

  if let Some(mut err) = cmd.stderr {
    if let Ok(len) = err.read_to_string(&mut stderr) {
      if len > 0 {
        ACTION_LOGS.push(ActionProgress::Err(stderr.clone()));
      }
    }
  }

  Some(ActionResult {
    stdout: lines,
    stderr,
  })
}

pub fn args_with_config<I, S>(args: I, git_version: GitVersion) -> Vec<String>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let mut new_args = Vec::<String>::new();

  if let Some(config_args) = config_override_arg(git_version) {
    new_args.extend(config_args);
  }

  for a in args {
    if let Some(arg) = a.as_ref().to_str() {
      new_args.push(arg.to_string());
    }
  }

  println!("git {}", new_args.join(" "));

  new_args
}

fn config_override_arg(git_version: GitVersion) -> Option<[String; 2]> {
  match env::consts::OS {
    "windows" => Some([
      String::from("-c"),
      // String::from("credential.helper=manager-core"),
      format!(
        "credential.helper=manager{}",
        if git_version.major >= 2 && git_version.minor >= 29 {
          "-core"
        } else {
          ""
        }
      ),
    ]),
    "linux" => Some([String::from("-c"), String::from("credential.helper=store")]),
    _ => None,
  }
}

pub fn get_action_logs(_: &ReqOptions) -> Vec<ActionProgress> {
  ACTION_LOGS.get().unwrap_or_default()
}

pub fn clear_action_logs(_: &ReqOptions) -> Option<()> {
  ACTION_LOGS.clear();

  Some(())
}

pub fn print_action_result(out: Option<ActionResult>) {
  if let Some(out) = out {
    eprintln!("{:?} {:?}", out.stdout, out.stderr);
  } else {
    eprintln!("No result from git action");
  }
}
