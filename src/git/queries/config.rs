use std::collections::HashMap;

use crate::git::git_types::GitConfig;
use crate::git::run_git;
use crate::git::run_git::RunGitOptions;
use crate::git::store::CONFIG;
use crate::map;
use crate::parser::standard_parsers::UNTIL_LINE_END;
use crate::parser::{parse_all, run_parser, ParseOptions, Parser};
use crate::server::git_request::ReqOptions;
use crate::{and, many, until_str, word};

impl GitConfig {
  pub fn new() -> GitConfig {
    GitConfig {
      entries: HashMap::new(),
      remotes: HashMap::new(),
    }
  }

  // We take short_name because this is the same between remote and local refs.
  pub fn get_remote_for_branch(&self, short_name: &str) -> String {
    let GitConfig { entries, .. } = self;

    if let Some(push_remote) = entries.get(&format!("branch.{}.pushremote", short_name)) {
      return push_remote.clone();
    }

    if let Some(push_default) = entries.get("remote.pushdefault") {
      return push_default.clone();
    }

    if let Some(remote) = entries.get(&format!("branch.${}.remote", short_name)) {
      return remote.clone();
    }

    String::from("origin")
  }

  pub fn get_tracking_branch_name(&self, local_branch: &str) -> String {
    let remote = self.get_remote_for_branch(local_branch);

    format!("refs/remotes/{}/{}", remote, local_branch)
  }
}

const P_CONFIG: Parser<HashMap<String, String>> = map!(
  many!(and!(until_str!("="), UNTIL_LINE_END)),
  |result: Vec<(String, String)>| { result.into_iter().collect::<HashMap<String, String>>() }
);

const P_REMOTE_NAME: Parser<String> = map!(
  and!(
    word!("remote."),
    until_str!("."),
    word!("url"),
    UNTIL_LINE_END
  ),
  |result: (&str, String, &str, String)| { result.1 }
);

/// Use this version on focus of GitFiend only. Get it from the store otherwise.
pub fn load_full_config(options: &ReqOptions) -> Option<GitConfig> {
  let result_text = run_git::run_git(RunGitOptions {
    repo_path: &options.repo_path,
    args: ["config", "--list"],
  });

  let config_result = parse_all(P_CONFIG, result_text?.as_str());
  let entries = config_result?;
  let mut remotes = HashMap::new();

  for (key, value) in entries.iter() {
    if key.starts_with("remote") {
      let name = run_parser(
        P_REMOTE_NAME,
        key,
        ParseOptions {
          must_parse_all: true,
          print_error: false,
        },
      );

      if let Some(name) = name {
        remotes.insert(name, value.clone());
      }
    }
  }

  let config = GitConfig { entries, remotes };

  // store_config(&config);
  // if let Ok(mut store) = store_lock.write() {
  //   (*store).config = config.clone();
  // }

  CONFIG.set(config.clone());

  Some(config)
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use crate::git::git_types::GitConfig;
  use crate::git::queries::config::{load_full_config, P_CONFIG, P_REMOTE_NAME};
  use crate::parser::parse_all;
  use crate::server::git_request::ReqOptions;

  #[test]
  fn load_config() {
    let result = load_full_config(&ReqOptions {
      repo_path: ".".to_string(),
    });

    assert!(result.is_some());
    assert!(!result.unwrap().entries.is_empty());
  }

  #[test]
  fn test_p_config() {
    let config = "credential.helper=osxkeychain
user.email=something@gmail.com
user.name=username
filter.lfs.clean=git-lfs clean -- %f
filter.lfs.smudge=git-lfs smudge -- %f
filter.lfs.process=git-lfs filter-process
filter.lfs.required=true
credential.helper=osxkeychain
core.repositoryformatversion=0
core.filemode=true
core.bare=false
core.logallrefupdates=true
core.ignorecase=true
core.precomposeunicode=true
remote.origin.url=https://gitlab.com/torquescrew/test-repo.git
remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
branch.master.remote=origin
branch.master.merge=refs/heads/master
branch.branch1.remote=origin
branch.branch1.merge=refs/heads/branch1
branch.other-branch.remote=origin
branch.other-branch.merge=refs/heads/other-branch
branch.left.remote=origin
branch.left.merge=refs/heads/left
branch.right.remote=origin
branch.right.merge=refs/heads/right
branch.a.remote=origin
branch.a.merge=refs/heads/a
remote.origin2.url=/Users/toby/Repos/test-repo-remote
remote.origin2.fetch=+refs/heads/*:refs/remotes/origin2/*
";
    let result = parse_all(P_CONFIG, config);
    assert!(result.is_some());

    let c = result.unwrap();

    assert_eq!(c.get("user.name").unwrap(), "username");
    assert_eq!(
      c.get("remote.origin2.fetch").unwrap(),
      "+refs/heads/*:refs/remotes/origin2/*"
    );
  }

  #[test]
  fn test_p_remote_name() {
    let result = parse_all(
      P_REMOTE_NAME,
      "remote.origin2.url=/Users/toby/Repos/test-repo-remote",
    );

    assert!(result.is_some());
    assert_eq!(result.unwrap(), "origin2");
  }

  #[test]
  fn test_get_remote_for_branch() {
    let config = GitConfig {
      entries: HashMap::from([
        ("remote.pushdefault".to_string(), "origin2".to_string()),
        ("branch.a.remote".to_string(), "origin3".to_string()),
      ]),
      remotes: HashMap::new(),
    };

    assert_eq!(config.get_remote_for_branch("a"), "origin2");
  }
}
