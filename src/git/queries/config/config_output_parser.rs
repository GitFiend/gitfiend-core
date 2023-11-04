use crate::parser::standard_parsers::UNTIL_LINE_END;
use crate::parser::Parser;
use crate::{and, many, map, until_str, word};
use std::collections::HashMap;

// Parses logged config from git command.
pub const P_CONFIG: Parser<HashMap<String, String>> = map!(
  many!(and!(until_str!("="), UNTIL_LINE_END)),
  |result: Vec<(String, String)>| {
    result.into_iter().collect::<HashMap<String, String>>()
  }
);

pub const P_REMOTE_NAME: Parser<String> = map!(
  and!(
    word!("remote."),
    until_str!("."),
    word!("url"),
    UNTIL_LINE_END
  ),
  |result: (&str, String, &str, String)| { result.1 }
);

pub const P_SUBMODULE_NAME: Parser<String> = map!(
  and!(
    word!("submodule."),
    until_str!("."),
    word!("url"),
    UNTIL_LINE_END
  ),
  |result: (&str, String, &str, String)| { result.1 }
);

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use crate::git::git_types::GitConfig;
  use crate::git::queries::config::config_file_parser::make_config_log;
  use crate::git::queries::config::config_output_parser::{
    P_CONFIG, P_REMOTE_NAME, P_SUBMODULE_NAME,
  };
  use crate::git::queries::config::load_full_config;
  use crate::parser::parse_all;
  use crate::server::git_request::ReqOptions;

  #[test]
  fn load_config() {
    let result = load_full_config(&ReqOptions {
      repo_path: ".".to_string(),
    });

    assert!(result.is_ok());
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
      submodules: Default::default(),
    };

    assert_eq!(config.get_remote_for_branch("a"), "origin2");
  }

  #[test]
  fn test_p_submodule() {
    let result = parse_all(P_SUBMODULE_NAME, "submodule.fiend-ui.url");

    assert!(result.is_some());
    assert_eq!(result.unwrap(), "fiend-ui");
  }

  #[test]
  fn test_parse_config2() {
    let text = r#"[core]
	repositoryformatversion = 0
	filemode = true
	bare = false
	logallrefupdates = true
	ignorecase = true
	precomposeunicode = true
# Some comment.	
[remote "origin"]
	url = https://github.com/GitFiend/git-fiend.git
	fetch = +refs/heads/*:refs/remotes/origin/*
	
; Some comment 2.
[branch "main"]
	remote = origin
	merge = refs/heads/main
[branch "cleanup"]
	remote = origin
	merge = refs/heads/cleanup
[branch "commit-switcher"]
	remote = origin
	merge = refs/heads/commit-switcher
[branch "server"]
	remote = origin
	merge = refs/heads/server
[branch "ws"]
	remote = origin
	merge = refs/heads/ws
[branch "alt-toolbar"]
	remote = origin
	merge = refs/heads/alt-toolbar
[branch "alt-ref-view"]
	remote = origin
	merge = refs/heads/alt-ref-view
[branch "image-conflicts"]
	remote = origin
	merge = refs/heads/image-conflicts
[branch "auto-complete"]
	remote = origin
	merge = refs/heads/auto-complete
[branch "mac-app"]
	remote = origin
	merge = refs/heads/mac-app
[branch "try-tauri"]
	remote = origin
	merge = refs/heads/try-tauri
[branch "split-view"]
	remote = origin
	merge = refs/heads/split-view
[branch "ssr-code-viewer"]
	remote = origin
	merge = refs/heads/ssr-code-viewer"#;

    let result = make_config_log(text);

    assert!(result.is_ok());
  }
}
