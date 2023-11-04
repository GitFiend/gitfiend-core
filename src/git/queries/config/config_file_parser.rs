use crate::parser::standard_parsers::{ANY_WORD, STRING_LITERAL, UNTIL_LINE_END, WS};
use crate::parser::{parse_all_err, Parser};
use crate::server::request_util::R;
use crate::{and, character, f, many, map2, not, or};
use std::fmt;

// See https://git-scm.com/docs/git-config#_syntax
pub enum Config {
  Section(Section),
  Other(Other),
}

pub struct Section(Heading, Vec<Row>);

pub struct Heading(String, Option<String>);

impl fmt::Display for Heading {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Heading(heading, None) => write!(f, "{heading}"),
      Heading(heading, Some(value)) => write!(f, "{heading}.{value}"),
    }
  }
}

enum Row {
  Data(String, String),
  Other(Other),
}

pub enum Other {
  Comment(String),
  Unknown(String),
}

const P_HEADING_1: Parser<Heading> =
  map2!(and!(character!('['), ANY_WORD, character!(']')), res, {
    Heading(res.1, None)
  });

const P_HEADING_2: Parser<Heading> = map2!(
  and!(
    character!('['),
    ANY_WORD,
    WS,
    STRING_LITERAL,
    character!(']')
  ),
  res,
  Heading(res.1, Some(res.3))
);

const P_HEADING: Parser<Heading> = or!(P_HEADING_1, P_HEADING_2);

//   merge = refs/heads/mac-app
const P_ROW: Parser<Row> = map2!(
  and!(WS, ANY_WORD, WS, character!('='), WS, UNTIL_LINE_END),
  res,
  Row::Data(res.1, res.5)
);

pub const P_CONFIG_FILE: Parser<Vec<Config>> =
  map2!(many!(or!(P_SECTION, P_CONFIG_OTHER)), res, res);

const P_SECTION: Parser<Config> = map2!(
  and!(P_HEADING, many!(or!(P_ROW, P_ROW_OTHER))),
  res,
  Config::Section(Section(res.0, res.1))
);

const P_COMMENT: Parser<Other> = map2!(
  and!(WS, or!(character!(';'), character!('#')), UNTIL_LINE_END),
  res,
  Other::Comment(res.2)
);

// Make sure we don't accidentally parse a row or heading as an unknown.
const P_UNKNOWN: Parser<Other> = map2!(
  and!(not!(P_HEADING), not!(P_ROW), UNTIL_LINE_END),
  res,
  Other::Unknown(res.2)
);

const P_OTHER: Parser<Other> = or!(P_COMMENT, P_UNKNOWN);
const P_CONFIG_OTHER: Parser<Config> = map2!(P_OTHER, res, Config::Other(res));
const P_ROW_OTHER: Parser<Row> = map2!(P_OTHER, res, Row::Other(res));

pub fn parse_config_file(input: &str) -> R<Vec<Config>> {
  parse_all_err(P_CONFIG_FILE, input)
}

pub fn make_config_log(input: &str) -> R<String> {
  let config = parse_config_file(input)?;

  Ok(
    config
      .into_iter()
      .map(|config| match config {
        Config::Section(section) => make_section_log(section),
        Config::Other(_) => f!(""),
      })
      .collect::<Vec<String>>()
      .join(""),
  )
}

fn make_section_log(section: Section) -> String {
  let Section(heading, rows) = section;

  let heading = heading.to_string();

  rows
    .into_iter()
    .map(|row| match row {
      Row::Data(key, value) => f!("{heading}.{key}={value}\n"),
      Row::Other(_) => f!(""),
    })
    .collect::<Vec<String>>()
    .join("")
}

#[cfg(test)]
mod tests {
  use crate::git::queries::config::config_file_parser::{make_config_log, P_HEADING};
  use crate::parser::parse_all;

  #[test]
  fn test_p_heading() {
    let result = parse_all(P_HEADING, "[core]");
    assert!(result.is_some());
    assert_eq!(result.unwrap().to_string(), "core");

    let result = parse_all(P_HEADING, "[remote \"origin\"]");
    assert!(result.is_some());
    assert_eq!(result.unwrap().to_string(), "remote.origin");

    let result = parse_all(P_HEADING, "[branch \"my-branch-name\"]");
    assert!(result.is_some());
    assert_eq!(result.unwrap().to_string(), "branch.my-branch-name");

    let result = parse_all(P_HEADING, "[branch \"feature/my-branch-name\"]");
    assert!(result.is_some());
    assert_eq!(result.unwrap().to_string(), "branch.feature/my-branch-name");
  }

  #[test]
  fn test_white_space_at_front() {
    let text = r#"
[core]
	repositoryformatversion = 0
	filemode = true 
"#;
    let result = make_config_log(text);

    assert!(result.is_ok());
    println!("{}", result.unwrap())
  }

  #[test]
  fn test_random_comments() {
    let text = r#"
; Comment
[core]
	repositoryformatversion = 0
	filemode = true 
# hello
"#;
    // let result = parse_all(P_CONFIG_FILE, text).unwrap();
    let result = make_config_log(text).unwrap();

    // assert_eq!(result, result2);
    assert_eq!(
      "core.repositoryformatversion=0
core.filemode=true",
      result.trim(),
    );
  }

  #[test]
  fn parse_read_config() {
    let text = r#"[core]
	repositoryformatversion = 0
	filemode = true
	bare = false
	logallrefupdates = true
[remote "origin"]
	url = https://github.com/GitFiend/rust-server.git
	fetch = +refs/heads/*:refs/remotes/origin/*
[branch "main"]
	remote = origin
	merge = refs/heads/main
[branch "ssr-code-viewer"]
	remote = origin
	merge = refs/heads/ssr-code-viewer
"#;

    let result = make_config_log(text);

    assert!(result.is_ok());

    assert_eq!(
      result.unwrap(),
      r#"core.repositoryformatversion=0
core.filemode=true
core.bare=false
core.logallrefupdates=true
remote.origin.url=https://github.com/GitFiend/rust-server.git
remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
branch.main.remote=origin
branch.main.merge=refs/heads/main
branch.ssr-code-viewer.remote=origin
branch.ssr-code-viewer.merge=refs/heads/ssr-code-viewer
"#
    );
  }

  #[test]
  fn test_parse_config() {
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
	merge = refs/heads/ssr-code-viewer
"#;

    let result = make_config_log(text);

    assert!(result.is_ok());
    println!("{}", result.unwrap())
  }
}
