use std::ffi::OsStr;
use std::io::{Error, Read};
use std::process::{Command, Stdio};
use std::{env, thread, time};
use time::Duration;

use ahash::AHashMap;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

use crate::git::git_settings::GIT_PATH;
use crate::git::git_version::GitVersion;
use crate::git::run_git_action::ActionError::{Credential, IO};
use crate::git::store::{ACTION_LOGS, GIT_VERSION};
use crate::server::git_request::ReqOptions;
use crate::util::global::Global;
use crate::{dprintln, global};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ActionOutput {
  pub stdout: String,
  pub stderr: String,
}

impl ActionOutput {
  fn new() -> Self {
    Self {
      stdout: String::new(),
      stderr: String::new(),
    }
  }
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
    IO(err.to_string())
  }
}

static ACTIONS: Global<AHashMap<u32, Option<Result<ActionOutput, ActionError>>>> =
  global!(AHashMap::new());

// 0 will be treated as an error.
static ACTION_IDS: Global<u32> = global!(1);

fn get_next_action_id() -> u32 {
  if let Some(id) = ACTION_IDS.get() {
    let new_id = id + 1;
    ACTION_IDS.set(new_id);
    new_id
  } else {
    0
  }
}

#[derive(Clone, Debug)]
pub struct RunGitActionOptions<'a, const N: usize> {
  pub commands: [Vec<&'a str>; N],
  pub repo_path: &'a str,
}

pub fn run_git_action<const N: usize>(options: RunGitActionOptions<N>) -> u32 {
  let id = get_next_action_id();

  ACTIONS.insert(id, None);

  let RunGitActionOptions {
    commands,
    repo_path,
  } = options;

  let git_version = GIT_VERSION.get().unwrap_or_else(GitVersion::new);

  let git_commands: Vec<Vec<String>> = commands
    .iter()
    .map(|c| c.iter().map(|a| a.to_string()).collect())
    .collect();
  let repo_path = repo_path.to_string();

  thread::spawn(move || {
    let mut output = ActionOutput::new();

    for c in git_commands {
      let result = run_git_action_inner(repo_path.clone(), git_version.clone(), c);

      if let Ok(result) = result {
        output.stdout.push_str(&result.stdout);
        output.stderr.push_str(&result.stderr);
      } else {
        ACTIONS.insert(id, Some(result));
        return;
      }
    }

    ACTIONS.insert(id, Some(Ok(output)));
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
  let PollOptions { action_id } = options;

  if *action_id == 0 {
    return Some(Err(IO(String::from(
      "Action id is 0. There was an error before run_git_action began.",
    ))));
  }

  let result = ACTIONS.get_by_key(action_id)?;

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
  // let mut cmd: Child = Command::new(_fake_action_script_path().expect("Fake action script path"))
  //   .stdout(Stdio::piped())
  //   .stderr(Stdio::piped())
  //   .spawn()?;

  let mut cmd = Command::new(GIT_PATH.as_path())
    .args(args_with_config(args, git_version))
    .current_dir(repo_path)
    .stderr(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

  let mut stderr_lines: Vec<String> = Vec::new();

  while let Ok(None) = cmd.try_wait() {
    if let Some(stderr) = cmd.stderr.as_mut() {
      let text = read_available_string_data(stderr);

      if !text.is_empty() {
        ACTION_LOGS.push(ActionProgress::Err(text.clone()));
        stderr_lines.push(text);
      }
    }

    thread::sleep(Duration::from_millis(50));
  }

  let status = cmd.wait()?;

  let mut stdout = String::new();

  if let Some(mut out) = cmd.stdout.take() {
    if let Ok(len) = out.read_to_string(&mut stdout) {
      if len > 0 {
        ACTION_LOGS.push(ActionProgress::Out(stdout.clone()));
      }
    }
  }

  if !status.success() {
    return if has_credential_error(&stderr_lines.join("\n")) {
      Err(Credential)
    } else {
      Err(ActionError::Git {
        stdout,
        stderr: stderr_lines.join("\n"),
      })
    };
  }

  Ok(ActionOutput {
    stdout,
    stderr: stderr_lines.join("\n"),
  })
}

fn read_available_string_data<T>(readable: &mut T) -> String
where
  T: Read,
{
  const BUFFER_SIZE: usize = 100;

  let mut all_data: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);

  loop {
    let mut buffer = [0; BUFFER_SIZE];

    if let Ok(size) = readable.read(&mut buffer) {
      all_data.extend(&buffer[..size]);

      if size == BUFFER_SIZE {
        continue;
      }
    }

    break;
  }

  String::from_utf8_lossy(&all_data).to_string()
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

  dprintln!("git {}", new_args.join(" "));

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

/*
git fetch --all --prune
fatal: could not read Username for 'https://github.com': terminal prompts disabled
error: Could not fetch origin

(These are printed to stderr and failure status code returned)
 */

// TODO: This seems brittle.
pub fn has_credential_error(stderr: &str) -> bool {
  stderr.contains("could not read Username") || stderr.contains("Invalid username or password")
}
