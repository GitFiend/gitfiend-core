use crate::git::run_git::{run_git_err, RunGitOptions};
use crate::git::store::GIT_VERSION;
use crate::parser::standard_parsers::UNSIGNED_INT;
use crate::parser::{parse_part, Parser};
use crate::server::git_request::ReqOptions;
use crate::{and, character, map, rep_parser_sep, take_char_while};
use serde::Serialize;
use ts_rs::TS;

pub fn load_git_version() {
  if let Ok(version_str) = run_git_err(RunGitOptions {
    repo_path: ".",
    args: ["--version"],
  }) {
    if let Some(version) = parse_version(&version_str.stdout) {
      GIT_VERSION.set(version);
    }
  }
}

// Expect this to return none if Git is not installed.
pub fn git_version(_: &ReqOptions) -> Option<GitVersion> {
  let version = GIT_VERSION.get()?;

  if version.major == 0 {
    load_git_version();

    let version = GIT_VERSION.get()?;

    if version.major != 0 {
      return Some(version);
    }

    return None;
  }

  Some(version)
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct GitVersion {
  pub major: u32,
  pub minor: u32,
  pub patch: u32,
}

impl GitVersion {
  pub fn new() -> Self {
    Self {
      major: 0,
      minor: 0,
      patch: 0,
    }
  }
}

fn parse_version(text: &str) -> Option<GitVersion> {
  parse_part(P_VERSION, text)?
}

// Takes something like "git version 2.32.0"
const P_VERSION_STRING: Parser<(String, Vec<String>)> = and!(
  take_char_while!(|c: char| !c.is_numeric()),
  rep_parser_sep!(UNSIGNED_INT, character!('.'))
);

const P_VERSION: Parser<Option<GitVersion>> =
  map!(P_VERSION_STRING, |res: (String, Vec<String>)| {
    let (_, nums) = res;

    Some(GitVersion {
      major: nums
        .get(0)
        .unwrap_or(&String::from(""))
        .parse()
        .unwrap_or(0),
      minor: nums
        .get(1)
        .unwrap_or(&String::from(""))
        .parse()
        .unwrap_or(0),
      patch: nums
        .get(2)
        .unwrap_or(&String::from(""))
        .parse()
        .unwrap_or(0),
    })
  });

#[cfg(test)]
mod tests {
  use crate::git::git_version::{parse_version, GitVersion, P_VERSION_STRING};
  use crate::parser::parse_all;

  #[test]
  fn test_p_version_string() {
    let result = parse_all(P_VERSION_STRING, "git version 2.32.0");

    assert!(result.is_some());
    assert_eq!(
      result.unwrap(),
      (
        "git version ".to_string(),
        vec!["2".to_string(), "32".to_string(), "0".to_string()]
      )
    );
  }

  #[test]
  fn test_p_version() {
    let result = parse_version("git version 2.32.0");

    assert!(result.is_some());

    assert_eq!(
      result.unwrap(),
      GitVersion {
        major: 2,
        minor: 32,
        patch: 0
      }
    );
  }

  #[test]
  fn test_p_short_version() {
    let result = parse_version("git version 2.32");

    assert!(result.is_some());

    assert_eq!(
      result.unwrap(),
      GitVersion {
        major: 2,
        minor: 32,
        patch: 0
      }
    );
  }

  #[test]
  fn test_p_mac_version() {
    let result = parse_version("git version 2.32.1 (Apple Git-133)");

    assert!(result.is_some());

    assert_eq!(
      result.unwrap(),
      GitVersion {
        major: 2,
        minor: 32,
        patch: 1
      }
    );
  }

  #[test]
  fn test_p_windows_version() {
    let result = parse_version("git version 2.37.3.windows.1");

    assert!(result.is_some());

    assert_eq!(
      result.unwrap(),
      GitVersion {
        major: 2,
        minor: 37,
        patch: 3
      }
    );
  }
}
