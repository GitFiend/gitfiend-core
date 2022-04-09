use crate::git::git_types::DateResult;
use crate::parser::standard_parsers::{ANY_WORD, SIGNED_INT, UNSIGNED_INT, WS};
use crate::parser::Parser;
use crate::{and, map, or, rep_parser_sep, take_char_while, until_str};
use crate::{character, Input};
use std::io::SeekFrom::End;

const END: &str = "4a41380f-a4e8-4251-9ca2-bf55186ed32a";

const P_GROUP: Parser<String> = take_char_while!(|c: char| { c != ',' });
const P_SEP: Parser<char> = map!(and!(WS, character!(','), WS), |res: (
  String,
  char,
  String
)| { ',' });

const P_EMAIL: Parser<String> = or!(P_GROUP, WS);

const P_DATE: Parser<DateResult> = map!(and!(UNSIGNED_INT, WS, SIGNED_INT), |res: (
  String,
  String,
  String
)| {
  DateResult {
    ms: res.0.parse::<i64>().unwrap() * 1000,
    adjustment: res.2.parse().unwrap(),
  }
});

const P_PARENTS: Parser<Vec<String>> = rep_parser_sep!(ANY_WORD, WS);

const P_MESSAGE: Parser<String> = until_str!(END);

pub const P_COMMIT_ROW: Parser<(
  String,
  char,
  String,
  char,
  DateResult,
  char,
  String,
  char,
  Vec<String>,
  char,
  String,
)> =
  and!(P_GROUP, P_SEP, P_EMAIL, P_SEP, P_DATE, P_SEP, P_GROUP, P_SEP, P_PARENTS, P_SEP, P_MESSAGE);

#[cfg(test)]
mod tests {
  use crate::parser::parse_all;
  use crate::P_COMMIT_ROW;

  #[test]
  fn test_p_commit_row() {
    let res = parse_all(
      P_COMMIT_ROW,
      "Toby, sugto555@gmail.com, 1648863350 +1300, \
      dd5733ad96082f0f33164bd1e2d72f7540bf7d9f, 2e8966986f620f491c34e6243a546d85dd2322e0, \
      Write commit row parser. Added necessary new git types. 4a41380f-a4e8-4251-9ca2-bf55186ed32a",
    );

    assert_eq!(res.is_some(), true);
  }
}
