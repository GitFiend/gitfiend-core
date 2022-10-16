use std::env;
use std::path::PathBuf;

pub fn _fake_action_script_path() -> Option<PathBuf> {
  Some(
    env::current_dir()
      .ok()?
      .parent()?
      .join("git-fiend")
      .join("scripts")
      .join("fake-action.sh"),
  )
}
