use crate::git::git_settings::GIT_PATH;
use crate::git::git_version::GitVersion;
use crate::git::run_git_action::ActionError::Credential;
use crate::git::store::ACTION_LOGS;
use crate::global;
use crate::server::git_request::{ActionOptions, ReqOptions};
use crate::util::global::Global;
use ahash::AHashMap;
use serde::Deserialize;
use serde::Serialize;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader, Error, Read};
use std::process::{Command, Stdio};
use std::{env, thread};
use ts_rs::TS;

#[derive(Clone, Debug)]
pub struct RunGitActionOptions<'a, const N: usize> {
  pub args: [&'a str; N],
  pub repo_path: &'a str,
  pub git_version: GitVersion,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ActionOutput {
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

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum ActionError {
  Credential,
  Git { stdout: String, stderr: String },
  IO(String),
}

impl From<Error> for ActionError {
  fn from(err: Error) -> Self {
    ActionError::IO(err.to_string())
  }
}

pub fn run_git_action<const N: usize>(
  options: RunGitActionOptions<N>,
) -> Result<ActionOutput, ActionError> {
  let mut cmd = Command::new(GIT_PATH.as_path())
    .args(args_with_config(options.args, options.git_version))
    .current_dir(options.repo_path)
    .stdout(Stdio::piped())
    .spawn()?;

  let mut lines: Vec<String> = Vec::new();

  let stdout = cmd
    .stdout
    .as_mut()
    .ok_or_else(|| ActionError::IO("Failed to get stdout as mut".to_string()))?;

  let stdout_reader = BufReader::new(stdout);
  let stdout_lines = stdout_reader.lines();

  for line in stdout_lines.flatten() {
    ACTION_LOGS.push(ActionProgress::Out(line.clone()));
    println!("{}", line);

    lines.push(line);
  }

  let status = cmd.wait()?;

  let mut stderr = String::new();

  if let Some(mut err) = cmd.stderr {
    if let Ok(len) = err.read_to_string(&mut stderr) {
      if len > 0 {
        ACTION_LOGS.push(ActionProgress::Err(stderr.clone()));
      }
    }
  }

  if !status.success() {
    return if has_credential_error(&stderr) {
      Err(Credential)
    } else {
      Err(ActionError::Git {
        stdout: lines.join(""),
        stderr: stderr.clone(),
      })
    };
  }

  Ok(ActionOutput {
    stdout: lines,
    stderr,
  })
}

static ACTIONS: Global<AHashMap<u32, Option<Result<ActionOutput, ActionError>>>> =
  global!(AHashMap::new());

static ACTION_IDS: Global<u32> = global!(0);

fn get_next_action_id() -> u32 {
  if let Some(id) = ACTION_IDS.get() {
    let new_id = id + 1;
    ACTION_IDS.set(new_id);
    new_id
  } else {
    0
  }
}

pub fn run_git_action2<const N: usize>(options: RunGitActionOptions<N>) -> u32 {
  let id = get_next_action_id();

  ACTIONS.insert(id, None);

  let RunGitActionOptions {
    args,
    git_version,
    repo_path,
  } = options;

  let args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
  let repo_path = repo_path.to_string();

  thread::spawn(move || {
    let result = run_git_action_inner(repo_path, git_version, args);

    ACTIONS.insert(id, Some(result));
  });

  id
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PollOptions {
  pub action_id: u32,
}

pub fn poll_action(options: &PollOptions) -> Option<Result<ActionOutput, ActionError>> {
  let result = ACTIONS.get_by_key(&options.action_id)?;

  if result.is_some() {
    ACTIONS.remove(&options.action_id);
  }

  result
}

pub fn run_git_action_inner(
  repo_path: String,
  git_version: GitVersion,
  args: Vec<String>,
) -> Result<ActionOutput, ActionError> {
  let mut cmd = Command::new(GIT_PATH.as_path())
    .args(args_with_config(args, git_version))
    .current_dir(repo_path)
    .stdout(Stdio::piped())
    .spawn()?;

  let mut lines: Vec<String> = Vec::new();

  let stdout = cmd
    .stdout
    .as_mut()
    .ok_or_else(|| ActionError::IO("Failed to get stdout as mut".to_string()))?;

  let stdout_reader = BufReader::new(stdout);
  let stdout_lines = stdout_reader.lines();

  for line in stdout_lines.flatten() {
    ACTION_LOGS.push(ActionProgress::Out(line.clone()));
    println!("{}", line);

    lines.push(line);
  }

  let status = cmd.wait()?;

  let mut stderr = String::new();

  if let Some(mut err) = cmd.stderr {
    if let Ok(len) = err.read_to_string(&mut stderr) {
      if len > 0 {
        ACTION_LOGS.push(ActionProgress::Err(stderr.clone()));
      }
    }
  }

  if !status.success() {
    return if has_credential_error(&stderr) {
      Err(Credential)
    } else {
      Err(ActionError::Git {
        stdout: lines.join(""),
        stderr: stderr.clone(),
      })
    };
  }

  Ok(ActionOutput {
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

// TODO: This seems brittle.
pub fn has_credential_error(stderr: &str) -> bool {
  stderr.contains("could not read Username") || stderr.contains("Invalid username or password")
}
