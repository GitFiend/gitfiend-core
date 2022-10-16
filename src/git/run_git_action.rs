use std::ffi::OsStr;
use std::io::{stderr, Error, Read};
use std::process::{Command, Stdio};
use std::{env, thread, time};
use time::Duration;

use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

use crate::dprintln;
use crate::git::action_state::{
  add_stderr_log, add_stdout_log, set_action_done, set_action_error, start_action, ActionState,
  ACTIONS2,
};
use crate::git::git_settings::GIT_PATH;
use crate::git::git_version::GitVersion;
use crate::git::run_git_action::ActionError::{Credential, IO};
use crate::git::store::GIT_VERSION;

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
  let id = start_action();

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
    for c in git_commands {
      let ok = run_git_action_inner(id, repo_path.clone(), git_version.clone(), c);

      if !ok {
        break;
      }
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
pub fn poll_action2(options: &PollOptions) -> Option<ActionState> {
  let PollOptions { action_id } = options;

  if *action_id == 0 {
    eprintln!("poll_action2: Requested action id of 0");
    return None;
  }

  if let Some(action) = ACTIONS2.get_by_key(action_id) {
    if action.done {
      ACTIONS2.remove(action_id);

      dprintln!("Num actions {:?}", ACTIONS2.len());
    }

    return Some(action);
  }

  None
}

pub fn run_git_action_inner(
  id: u32,
  repo_path: String,
  git_version: GitVersion,
  args: Vec<String>,
) -> bool {
  // let mut cmd = Command::new(_fake_action_script_path().expect("Fake action script path"))
  //   .stdout(Stdio::piped())
  //   .stderr(Stdio::piped())
  //   .spawn()?;

  if let Ok(mut cmd) = Command::new(GIT_PATH.as_path())
    .args(args_with_config(args, git_version))
    .current_dir(repo_path)
    .stderr(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
  {
    let mut stderr_lines: Vec<String> = Vec::new();

    while let Ok(None) = cmd.try_wait() {
      thread::sleep(Duration::from_millis(50));

      if let Some(stderr) = cmd.stderr.as_mut() {
        let text = read_available_string_data(stderr);

        if !text.is_empty() {
          add_stderr_log(id, &text);

          stderr_lines.push(text);
        }
      }

      if let Some(stdout) = cmd.stdout.as_mut() {
        let text = read_available_string_data(stdout);

        if !text.is_empty() {
          add_stdout_log(id, &text);
        }
      }
    }

    if let Ok(status) = cmd.wait() {
      if !status.success() {
        if has_credential_error(&stderr_lines.join("\n")) {
          set_action_error(id, Credential);
        } else {
          set_action_error(id, ActionError::Git);
        }
        return false;
      }

      set_action_done(id);
    } else {
      set_action_error(id, IO(String::from("Failed to get status on cmd.wait()")));
    }
  } else {
    set_action_error(id, IO(String::from("Failed to spawn command")));
  }

  true
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
