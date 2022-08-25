use std::env;
use std::path::PathBuf;

use crate::dprintln;
use serde::Deserialize;
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, Deserialize, TS)]
#[ts(export)]
pub struct Credentials {
  pub username: String,
  pub password: String,
}

// static CREDENTIALS: Global<Option<Credentials>> = global!(None);

pub fn set_credentials(credentials: &Credentials) -> Option<()> {
  // CREDENTIALS.set(Some(credentials.clone()))

  env::set_var("GITFIEND_USERNAME", &credentials.username);
  env::set_var("GITFIEND_PASSWORD", &credentials.password);

  if let Some(path) = get_ask_pass_path() {
    dprintln!("Setting GIT_ASKPASS to {:?}", path.to_str());

    env::set_var("GIT_ASKPASS", path.to_str()?);
  }

  Some(())
}

pub fn get_ask_pass_path() -> Option<PathBuf> {
  let name = if env::consts::OS == "windows" {
    "ask-pass.exe"
  } else {
    "ask-pass"
  };

  let dir = env::current_dir().ok()?;

  #[cfg(debug_assertions)]
  return Some(
    dir
      .parent()?
      .join("git-fiend")
      .join("src")
      .join("ask-pass")
      .join("target")
      .join("debug")
      .join(name),
  );

  #[cfg(not(debug_assertions))]
  return Some(dir.join(name));
}
