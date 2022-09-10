use once_cell::sync::Lazy;
use std::env;
use std::path::{Path, PathBuf};

pub static GIT_PATH: Lazy<PathBuf> = Lazy::new(get_git_path);

fn get_git_path() -> PathBuf {
  // #[cfg(feature = "internal-git")]
  // if let Some(dir) = get_internal_git_dir() {
  //   return if env::consts::OS == "windows" {
  //     dir.join("cmd").join("git.exe")
  //   } else {
  //     dir.join("bin").join("git")
  //   };
  // }

  PathBuf::from("git")
}

fn get_internal_git_dir() -> Option<PathBuf> {
  if let Ok(dir) = env::current_dir() {
    // Start from /gitfiend-seed/rust-server in dev mode
    #[cfg(debug_assertions)]
    return Some(
      dir
        .parent()?
        .join("git-fiend")
        .join("node_modules")
        .join("dugite")
        .join("git"),
    );

    // Start from /app.asar.unpacked/output-code/core in release mode.
    #[cfg(not(debug_assertions))]
    return Some(dir.parent()?.join("git"));
  }

  None
}

fn get_git_exec_location(git_dir: &Path) -> PathBuf {
  if env::consts::OS == "windows" {
    git_dir.join("mingw64").join("libexec").join("git-core")
  } else {
    git_dir.join("libexec").join("git-core")
  }
}

pub fn set_git_env() {
  // #[cfg(feature = "internal-git")]
  // env::set_var("GIT_EXEC_PATH", get_git_exec_location(GIT_PATH.as_path()));

  // We don't want any prompts in the terminal (e.g for password).
  env::set_var("GIT_TERMINAL_PROMPT", "0");

  if env::consts::OS == "macos" {
    if let Ok(path) = env::var("PATH") {
      if !path.contains("usr/local/bin") {
        env::set_var("PATH", format!("{}:/usr/local/bin", path));
      }
    }
  }
}
