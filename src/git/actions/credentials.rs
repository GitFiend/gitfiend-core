use std::env;
use std::path::PathBuf;

use crate::dprintln;
use crate::server::request_util::{ES, R};
use serde::Deserialize;
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, TS)]
#[ts(export)]
pub struct Credentials {
  pub username: String,
  pub password: String,
}

pub fn set_credentials(credentials: &Credentials) -> R<()> {
  env::set_var("GITFIEND_USERNAME", &credentials.username);
  env::set_var("GITFIEND_PASSWORD", &credentials.password);

  let path = get_ask_pass_path()?;
  dprintln!("Setting GIT_ASKPASS to {:?}", path.to_str());

  env::set_var(
    "GIT_ASKPASS",
    path
      .to_str()
      .ok_or(ES::from("set_credentials: Failed to convert path to str"))?,
  );

  Ok(())

  // if let Some(path) = get_ask_pass_path() {
  //   dprintln!("Setting GIT_ASKPASS to {:?}", path.to_str());
  //
  //   env::set_var("GIT_ASKPASS", path.to_str()?);
  // }
  //
  // Some(())
}

pub fn get_ask_pass_path() -> R<PathBuf> {
  let name = if env::consts::OS == "windows" {
    "ask-pass.exe"
  } else {
    "ask-pass"
  };

  #[cfg(debug_assertions)]
  let dir = env::current_dir()?;

  let missing_parent = ES::from("get_ask_pass_path: Couldn't get parent dir.");

  #[cfg(debug_assertions)]
  return Ok(
    dir
      .parent()
      .ok_or(missing_parent)?
      .join("git-fiend")
      .join("src")
      .join("ask-pass")
      .join("target")
      .join("release") // Use release version as we typically have no reason to build debug.
      .join(name),
  );

  #[cfg(not(debug_assertions))]
  Ok(
    env::current_exe()?
      .parent()
      .ok_or(missing_parent.clone())?
      .parent()
      .ok_or(missing_parent)?
      .join("ask-pass")
      .join(name),
  )
}
