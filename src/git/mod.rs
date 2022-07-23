use crate::parser::standard_parsers::UNSIGNED_INT;
use crate::parser::{parse_part, Parser};
use crate::server::git_request::ReqOptions;
use crate::{and, character, map, rep_parser_sep, take_char_while};
use run_git::RunGitOptions;
use serde::Serialize;
use ts_rs::TS;

pub(crate) mod git_settings;
pub(crate) mod git_types;
pub(crate) mod git_types_extra_impl;
pub(crate) mod queries;
pub(crate) mod run_git;
pub(crate) mod run_git_action;
pub(crate) mod store;

// Expect this to return none if Git is not installed.
pub fn git_version(_: &ReqOptions) -> Option<GitVersion> {
  let version_str = run_git::run_git(RunGitOptions {
    repo_path: ".",
    args: ["--version"],
  })?;

  parse_version(&version_str)
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct GitVersion {
  pub major: u32,
  pub minor: u32,
  pub patch: u32,
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
      major: nums.get(0)?.parse().ok()?,
      minor: nums.get(1)?.parse().ok()?,
      patch: nums.get(2)?.parse().ok()?,
    })
  });

#[cfg(test)]
mod tests {
  use crate::git::{parse_version, GitVersion, P_VERSION_STRING};
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
}
