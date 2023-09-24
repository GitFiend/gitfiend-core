use crate::parser::standard_parsers::{ANY_WORD, STRING_LITERAL, UNTIL_LINE_END, WS};
use crate::parser::Parser;
use crate::{and, character, f, many, map2, not, or};

const P_HEADING_1: Parser<String> = map2!(and!(character!('['), ANY_WORD, character!(']')), res, {
  res.1
});

const P_HEADING_2: Parser<String> = map2!(
  and!(
    character!('['),
    ANY_WORD,
    WS,
    STRING_LITERAL,
    character!(']')
  ),
  res,
  f!("{}.{}", res.1, res.3)
);

pub const P_HEADING: Parser<String> = or!(P_HEADING_1, P_HEADING_2);

//   merge = refs/heads/mac-app
const P_ROW: Parser<String> = map2!(
  and!(WS, ANY_WORD, WS, character!('='), WS, UNTIL_LINE_END),
  res,
  f!("{}={}\n", res.1, res.5)
);

pub const P_CONFIG2: Parser<String> =
  map2!(many!(or!(P_SECTION, P_OTHER)), sections, sections.join(""));

const P_SECTION: Parser<String> = map2!(and!(P_HEADING, many!(or!(P_ROW, P_OTHER))), res, {
  let (header, rows) = res;
  rows
    .into_iter()
    .flat_map(|row| {
      if row.is_empty() {
        None
      } else {
        Some(f!("{}.{}", header, row))
      }
    })
    .collect::<Vec<String>>()
    .join("")
});

const P_COMMENT: Parser<String> = map2!(
  and!(WS, or!(character!(';'), character!('#')), UNTIL_LINE_END),
  res,
  f!("{}{}", res.1, res.2)
);

// Make sure we don't accidentally parse a row or heading as an unknown.
const P_UNKNOWN: Parser<String> = map2!(
  and!(not!(P_HEADING), not!(P_ROW), UNTIL_LINE_END),
  res,
  res.2
);

const P_OTHER: Parser<String> = or!(P_COMMENT, P_UNKNOWN);

#[cfg(test)]
mod tests {
  use crate::git::queries::config_parser::P_CONFIG2;
  use crate::parser::parse_all;

  #[test]
  fn test_white_space_at_front() {
    let text = r#"
[core]
	repositoryformatversion = 0
	filemode = true 
"#;
    let result = parse_all(P_CONFIG2, text);

    assert!(result.is_some());
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
    let result = parse_all(P_CONFIG2, text);

    assert!(result.is_some());
    println!("{}", result.unwrap())
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

    let result = parse_all(P_CONFIG2, text);

    assert!(result.is_some());

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

    let result = parse_all(P_CONFIG2, text);

    assert!(result.is_some());
    println!("{}", result.unwrap())
  }
}
