use ahash::AHashMap;
use serde::Serialize;
use ts_rs::TS;

use crate::git::run_git_action::ActionError;
use crate::global;
use crate::util::global::Global;

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ActionState {
  pub stdout: Vec<String>,
  pub stderr: Vec<String>,
  pub done: bool,
  pub error: Option<ActionError>,
}

impl ActionState {
  pub fn new() -> Self {
    Self {
      stdout: Vec::new(),
      stderr: Vec::new(),
      done: false,
      error: None,
    }
  }
}

pub static ACTIONS2: Global<AHashMap<u32, ActionState>> = global!(AHashMap::new());

// TODO: this should probably get the id and return it instead of having separate calls.
pub fn start_action(id: u32) {
  ACTIONS2.insert(id, ActionState::new());
}

pub fn add_stderr_log(id: u32, text: &str) {
  if let Some(mut action) = ACTIONS2.get_by_key(&id) {
    action.stderr.push(text.to_string());

    // TODO: Do we actually need to insert it again?
    ACTIONS2.insert(id, action);
  } else {
    eprintln!("add_stderr_log: Didn't find action id {}", id);
  }
}

pub fn add_stdout_log(id: u32, text: &str) {
  if let Some(mut action) = ACTIONS2.get_by_key(&id) {
    action.stdout.push(text.to_string());

    // TODO: Do we actually need to insert it again?
    ACTIONS2.insert(id, action);
  } else {
    eprintln!("add_stdout_log: Didn't find action id {}", id);
  }
}

pub fn set_action_error(id: u32, error: ActionError) {
  if let Some(mut action) = ACTIONS2.get_by_key(&id) {
    action.error = Some(error);
    action.done = true;

    // TODO: Do we actually need to insert it again?
    ACTIONS2.insert(id, action);
  } else {
    eprintln!("set_action_error: Didn't find action id {}", id);
  }
}

pub fn set_action_done(id: u32) {
  if let Some(mut action) = ACTIONS2.get_by_key(&id) {
    action.done = true;

    // TODO: Do we actually need to insert it again?
    ACTIONS2.insert(id, action);
  } else {
    eprintln!("set_action_done: Didn't find action id {}", id);
  }
}

#[cfg(test)]
mod tests {
  use crate::git::action_state::{add_stdout_log, start_action, ACTIONS2};

  #[test]
  fn test_start_action() {
    start_action(3);

    assert!(ACTIONS2.get_by_key(&3).is_some());
  }

  #[test]
  fn test_add_log() {
    start_action(3);
    add_stdout_log(3, "stdout text");

    assert!(!ACTIONS2.get_by_key(&3).unwrap().stdout.is_empty());
    assert_eq!(ACTIONS2.get_by_key(&3).unwrap().stdout[0], "stdout text");
  }
}
