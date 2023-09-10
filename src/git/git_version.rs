use crate::git::run_git::{run_git, RunGitOptions};
use crate::git::store::GIT_VERSION;
use crate::parser::standard_parsers::UNSIGNED_INT;
use crate::parser::{parse_part, Parser};
use crate::server::git_request::ReqOptions;
use crate::{and, character, map2, rep_parser_sep, take_char_while};
use bstr::BString;
use serde::Serialize;
use ts_rs::TS;

pub fn load_git_version() {
  if let Some(version_str) = run_git(RunGitOptions {
    repo_path: ".",
    args: ["--version"],
  }) {
    if let Some(version) = parse_version(&version_str) {
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
  parse_part(P_VERSION, text.as_bytes())?
}

// Takes something like "git version 2.32.0"
const P_VERSION_STRING: Parser<(BString, Vec<BString>)> = and!(
  take_char_while!(|c: char| !c.is_numeric()),
  rep_parser_sep!(UNSIGNED_INT, character!('.'))
);

const P_VERSION: Parser<Option<GitVersion>> = map2!(P_VERSION_STRING, res, {
  let (_, nums) = res;

  Some(GitVersion {
    major: nums
      .get(0)
      .unwrap_or(&BString::from(""))
      .to_string()
      .parse()
      .unwrap_or(0),
    minor: nums
      .get(1)
      .unwrap_or(&BString::from(""))
      .to_string()
      .parse()
      .unwrap_or(0),
    patch: nums
      .get(2)
      .unwrap_or(&BString::from(""))
      .to_string()
      .parse()
      .unwrap_or(0),
  })
});

#[cfg(test)]
mod tests {
  use crate::git::git_version::{parse_version, GitVersion, P_VERSION_STRING};
  use crate::parser::parse_all;
  use bstr::BString;

  #[test]
  fn test_p_version_string() {
    let result = parse_all(P_VERSION_STRING, b"git version 2.32.0");

    assert!(result.is_some());
    assert_eq!(
      result.unwrap(),
      (
        BString::from("git version "),
        vec![BString::from("2"), BString::from("32"), BString::from("0")]
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
