use once_cell::sync::Lazy;
use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;

pub static GIT_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("git"));

pub fn set_git_env() {
  // We don't want any prompts in the terminal (e.g for password).
  set_env_var("GIT_TERMINAL_PROMPT", "0");

  if env::consts::OS == "macos" {
    if let Ok(path) = env::var("PATH") {
      if !path.contains("usr/local/bin") {
        set_env_var("PATH", format!("{}:/usr/local/bin", path));
      }
    }
  }

  if let Err(err) = fix_path_env::fix() {
    eprintln!("{err}");
  }
}

pub fn set_env_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
  unsafe {
    env::set_var(key, value);
  }
}
