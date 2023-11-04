use crate::parser::standard_parsers::{ANY_WORD, STRING_LITERAL, UNTIL_LINE_END, WS};
use crate::parser::{parse_all_err, Parser};
use crate::server::request_util::R;
use crate::{and, character, f, many, map2, not, or};

// See https://git-scm.com/docs/git-config#_syntax
pub enum Config {
  Section(Section),
  Other(Other),
}

pub struct Section(Heading, Vec<Row>);

pub struct Heading(String, Option<String>);

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

  let heading = match heading {
    Heading(heading, None) => f!("{heading}"),
    Heading(heading, Some(value)) => f!("{heading}.{value}"),
  };

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
  use super::*;

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

    let config = make_config_log(text);

    assert!(config.is_ok());

    assert_eq!(
      config.unwrap(),
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
}
