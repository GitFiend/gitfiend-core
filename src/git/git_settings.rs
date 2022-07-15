use cached::once_cell::sync::Lazy;
use std::env;
use std::path::{Path, PathBuf};

pub static GIT_PATH: Lazy<PathBuf> = Lazy::new(get_git_path);

fn get_git_path() -> PathBuf {
  #[cfg(feature = "internal-git")]
  if let Some(dir) = get_internal_git_dir() {
    return if env::consts::OS == "windows" {
      dir.join("cmd").join("git.exe")
    } else {
      dir.join("bin").join("git")
    };
  }

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

// #[cfg(test)]
// mod tests {
//   use std::env;
//
//   #[test]
//   fn test_internal_git_path_dev() {
//     println!("{:?}", env::current_dir());
//   }
// }
