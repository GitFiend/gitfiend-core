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

pub static ACTIONS: Global<AHashMap<u32, ActionState>> = global!(AHashMap::new());

// TODO: this should probably get the id and return it instead of having separate calls.
pub fn start_action() -> u32 {
  let id = get_next_action_id();

  ACTIONS.insert(id, ActionState::new());

  id
}

pub fn add_stderr_log(id: u32, text: &str) {
  if let Some(mut action) = ACTIONS.get_by_key(&id) {
    action.stderr.push(text.to_string());

    // TODO: Do we actually need to insert it again?
    ACTIONS.insert(id, action);
  } else {
    eprintln!("add_stderr_log: Didn't find action id {}", id);
  }
}

pub fn add_stdout_log(id: u32, text: &str) {
  if let Some(mut action) = ACTIONS.get_by_key(&id) {
    action.stdout.push(text.to_string());

    // TODO: Do we actually need to insert it again?
    ACTIONS.insert(id, action);
  } else {
    eprintln!("add_stdout_log: Didn't find action id {}", id);
  }
}

pub fn set_action_error(id: u32, error: ActionError) {
  if let Some(mut action) = ACTIONS.get_by_key(&id) {
    action.error = Some(error);
    action.done = true;

    // TODO: Do we actually need to insert it again?
    ACTIONS.insert(id, action);
  } else {
    eprintln!("set_action_error: Didn't find action id {}", id);
  }
}

pub fn set_action_done(id: u32) {
  if let Some(mut action) = ACTIONS.get_by_key(&id) {
    action.done = true;

    // TODO: Do we actually need to insert it again?
    ACTIONS.insert(id, action);
  } else {
    eprintln!("set_action_done: Didn't find action id {}", id);
  }
}

#[cfg(test)]
mod tests {
  use crate::git::action_state::{add_stdout_log, start_action, ACTIONS};

  #[test]
  fn test_start_action() {
    let id = start_action();

    assert!(ACTIONS.get_by_key(&id).is_some());
  }

  #[test]
  fn test_add_log() {
    let id = start_action();
    add_stdout_log(id, "stdout text");

    assert!(!ACTIONS.get_by_key(&id).unwrap().stdout.is_empty());
    assert_eq!(ACTIONS.get_by_key(&id).unwrap().stdout[0], "stdout text");
  }
}
