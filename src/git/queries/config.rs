use crate::git::git_types::GitConfig;
use crate::git::run_git;
use crate::git::run_git::RunGitOptions;
use crate::git::store::CONFIG;
use crate::parser::standard_parsers::{ANY_WORD, STRING_LITERAL, UNTIL_LINE_END, WS};
use crate::parser::{parse_all_err, run_parser, ParseOptions, Parser};
use crate::server::git_request::ReqOptions;
use crate::server::request_util::R;
use crate::{and, f, many, or, until_str, word};
use crate::{character, map};
use bstr::{BString, B};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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

    if let Some(remote) = entries.get(&format!("branch.{}.remote", short_name)) {
      return remote.clone();
    }

    String::from("origin")
  }

  pub fn get_tracking_branch_name(&self, local_branch: &str) -> String {
    let remote = self.get_remote_for_branch(local_branch);

    format!("refs/remotes/{}/{}", remote, local_branch)
  }
}

const P_HEADING_1: Parser<BString> = map!(
  and!(character!('['), ANY_WORD, character!(']')),
  |res: (char, BString, char)| { res.1 }
);

const P_HEADING_2: Parser<BString> = map!(
  and!(
    character!('['),
    ANY_WORD,
    WS,
    STRING_LITERAL,
    character!(']')
  ),
  |res: (char, BString, BString, BString, char)| { BString::from(f!("{}.{}", res.1, res.3)) }
);

const P_HEADING: Parser<BString> = or!(P_HEADING_1, P_HEADING_2);

//   merge = refs/heads/mac-app
const P_ROW: Parser<BString> = map!(
  and!(WS, ANY_WORD, WS, character!('='), WS, UNTIL_LINE_END),
  |res: (BString, BString, BString, char, BString, BString)| {
    BString::from(f!("{}={}\n", res.1, res.5))
  }
);

const P_SECTION: Parser<BString> = map!(and!(P_HEADING, many!(P_ROW)), |(header, rows): (
  BString,
  Vec<BString>
)| {
  let mut section = Vec::<u8>::new();

  for row in rows {
    section.append(&mut bstr::join(b".", &[&header, &row]));
  }

  BString::from(section)
});

const P_CONFIG2: Parser<BString> = map!(many!(P_SECTION), |sections: Vec<BString>| {
  BString::from(bstr::concat(&sections))
});

const P_CONFIG: Parser<HashMap<String, String>> = map!(
  many!(and!(until_str!(b"="), UNTIL_LINE_END)),
  |result: Vec<(BString, BString)>| {
    result
      .into_iter()
      .map(|(key, value)| (key.to_string(), value.to_string()))
      .collect::<HashMap<String, String>>()
  }
);

const P_REMOTE_NAME: Parser<BString> = map!(
  and!(
    word!(B("remote.")),
    until_str!(B(".")),
    word!(B("url")),
    UNTIL_LINE_END
  ),
  |result: (&[u8], BString, &[u8], BString)| { result.1 }
);

/// Use this version on focus of GitFiend only. Get it from the store otherwise.
pub fn load_full_config(options: &ReqOptions) -> R<GitConfig> {
  let ReqOptions { repo_path } = options;

  let config_path = Path::new(repo_path).join(".git").join("config");
  // println!("config exists: {}, {:?}", config_path.exists(), config_path);

  // let t2 = Instant::now();

  let result_text = if let Ok(text) = fs::read_to_string(config_path) {
    let r = parse_all_err(P_CONFIG2, text.as_bytes());
    // println!("time to read text config: {}ms", t2.elapsed().as_millis());
    r
  } else {
    // let t2 = Instant::now();
    Ok(
      run_git::run_git_bstr(RunGitOptions {
        repo_path,
        args: ["config", "--list"],
      })?
      .stdout,
    )
  };

  // println!("time to load git config: {}ms", t2.elapsed().as_millis());

  let config_result = parse_all_err(P_CONFIG, &result_text?);
  let entries = config_result?;
  let mut remotes: HashMap<String, String> = HashMap::new();

  for (key, value) in entries.iter() {
    if key.starts_with("remote") {
      let name = run_parser(
        P_REMOTE_NAME,
        key.as_bytes(),
        ParseOptions {
          must_parse_all: true,
          print_error: false,
        },
      );

      if let Some(name) = name {
        remotes.insert(name.to_string(), value.clone());
      }
    }
  }

  let config = GitConfig { entries, remotes };

  CONFIG.insert(repo_path.clone(), config.clone());

  Ok(config)
}

#[cfg(test)]
mod tests {
  use bstr::B;
  use std::collections::HashMap;

  use crate::git::git_types::GitConfig;
  use crate::git::queries::config::{
    load_full_config, P_CONFIG, P_CONFIG2, P_HEADING, P_REMOTE_NAME,
  };
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
    let config = b"credential.helper=osxkeychain
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
      b"remote.origin2.url=/Users/toby/Repos/test-repo-remote",
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

  #[test]
  fn test_p_heading() {
    let result = parse_all(P_HEADING, b"[core]");
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "core");

    let result = parse_all(P_HEADING, b"[remote \"origin\"]");
    assert!(result.is_some());
    assert_eq!(result.unwrap(), B("remote.origin"));

    let result = parse_all(P_HEADING, B("[branch \"my-branch-name\"]"));
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "branch.my-branch-name");

    let result = parse_all(P_HEADING, B("[branch \"feature/my-branch-name\"]"));
    assert!(result.is_some());
    assert_eq!(result.unwrap(), B("branch.feature/my-branch-name"));
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
[remote "origin"]
	url = https://github.com/GitFiend/git-fiend.git
	fetch = +refs/heads/*:refs/remotes/origin/*
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

    let result = parse_all(P_CONFIG2, text.as_bytes());

    assert!(result.is_some());
    println!("{}", result.unwrap());
  }
}
