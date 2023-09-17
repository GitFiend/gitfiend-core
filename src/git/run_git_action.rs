use serde::Deserialize;
use serde::Serialize;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader, Error, Read};
use std::process::{Command, Stdio};
use std::{env, thread, time};
use time::Duration;
use ts_rs::TS;

use crate::dprintln;
use crate::git::action_state::{
  add_stderr_log, add_stdout_log, set_action_done, set_action_error, start_action, ActionState,
  ACTIONS,
};
use crate::git::git_settings::GIT_PATH;
use crate::git::git_version::GitVersion;
use crate::git::run_git_action::ActionError::{Credential, Git, IO};
use crate::git::store::get_git_version;
use crate::server::request_util::{ES, R};

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum ActionError {
  Credential,
  Git,
  IO(String),
}

impl From<Error> for ActionError {
  fn from(err: Error) -> Self {
    IO(err.to_string())
  }
}

#[derive(Clone, Debug)]
pub struct RunGitActionOptions<'a, const N: usize> {
  pub commands: [Vec<&'a str>; N],
  pub repo_path: &'a str,
}

pub fn run_git_action<const N: usize>(options: RunGitActionOptions<N>) -> u32 {
  let RunGitActionOptions {
    commands,
    repo_path,
  } = options;

  let git_commands: Vec<Vec<String>> = commands
    .iter()
    .map(|c| c.iter().map(|a| a.to_string()).collect())
    .collect();

  run_git_action_with_vec(repo_path, git_commands)
}

pub fn run_git_action_with_vec(repo_path: &str, commands: Vec<Vec<String>>) -> u32 {
  let id = start_action();

  let git_version = get_git_version();

  let repo_path = repo_path.to_string();

  thread::spawn(move || {
    let mut failed = false;

    for c in commands {
      if let Err(e) = run_git_action_inner(id, repo_path.clone(), git_version.clone(), c) {
        set_action_error(id, e);
        failed = true;
        break;
      }
    }

    if !failed {
      set_action_done(id);
    }
  });

  id
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PollOptions {
  pub action_id: u32,
}

// None result means the action doesn't exist.
pub fn poll_action2(options: &PollOptions) -> R<ActionState> {
  let PollOptions { action_id } = options;

  if *action_id == 0 {
    // eprintln!("poll_action2: Requested action id of 0");
    return Err(ES::from("poll_action2: Requested action id of 0"));
  }

  if let Some(action) = ACTIONS.get_by_key(action_id) {
    if action.done {
      ACTIONS.remove(action_id);

      // dprintln!("Num actions {:?}", ACTIONS2.len());
    }

    return Ok(action);
  }

  Err(ES::from("poll_action2: action not found"))
}

pub fn run_git_action_inner(
  id: u32,
  repo_path: String,
  git_version: GitVersion,
  args: Vec<String>,
) -> Result<(), ActionError> {
  let mut cmd = Command::new(GIT_PATH.as_path())
    .args(args_with_config(args, git_version))
    .current_dir(repo_path)
    .stderr(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

  let out = BufReader::new(
    cmd
      .stdout
      .take()
      .ok_or_else(|| IO("stdout.take() failed".to_string()))?,
  );
  let mut err = cmd
    .stderr
    .take()
    .ok_or_else(|| IO("stderr.take() failed".to_string()))?;

  let thread = thread::spawn(move || {
    while let Ok(None) = cmd.try_wait() {
      thread::sleep(Duration::from_millis(50));

      let text = read_available_string_data(&mut err);

      if !text.is_empty() {
        add_stderr_log(id, &text);
      }
    }

    cmd
  });

  out.lines().for_each(|line| {
    if let Ok(line) = line {
      add_stdout_log(id, &line);
    }
  });

  if let Ok(mut cmd) = thread.join() {
    let status = cmd.wait()?;

    if !status.success() {
      let action = ACTIONS
        .get_by_key(&id)
        .ok_or_else(|| IO(format!("Failed to load action {} from ACTIONS", id)))?;

      return if has_credential_error(&action.stderr.join("\n")) {
        Err(Credential)
      } else {
        Err(Git)
      };
    }
  }

  Ok(())
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

/*
git fetch --all --prune
fatal: could not read Username for 'https://github.com': terminal prompts disabled
error: Could not fetch origin

(These are printed to stderr and failure status code returned)
 */

/*
GitHub error message:

remote: Support for password authentication was removed on August 13, 2021.
remote: Please see https://docs.github.com/en/get-started/getting-started-with-git/about-remote-repositories#cloning-with-https-urls for information on currently recommended modes of authentication.
fatal: Authentication failed for 'https://github.com/....git/'
 */
// TODO: This seems brittle.
pub fn has_credential_error(stderr: &str) -> bool {
  stderr.contains("could not read Username")
    || stderr.contains("Invalid username or password")
    || stderr.contains("Authentication failed for")
}
