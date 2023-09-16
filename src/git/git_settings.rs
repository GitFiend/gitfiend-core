use once_cell::sync::Lazy;
use std::env;
use std::path::PathBuf;

pub static GIT_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("git"));

pub fn set_git_env() {
  // We don't want any prompts in the terminal (e.g for password).
  env::set_var("GIT_TERMINAL_PROMPT", "0");

  if env::consts::OS == "macos" {
    if let Ok(path) = env::var("PATH") {
      if !path.contains("usr/local/bin") {
        env::set_var("PATH", format!("{}:/usr/local/bin", path));
      }
    }
  }

  if let Err(err) = fix_path_env::fix() {
    eprintln!("{err}");
  }
}
