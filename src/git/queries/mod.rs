use crate::git::git_types::DateResult;
use crate::parser::standard_parsers::{SIGNED_INT, UNSIGNED_INT, WS};
use crate::parser::Parser;
use crate::{and, map, or, take_char_while};
use crate::{character, Input};

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

pub const P_COMMIT_ROW: Parser<(String, char, String, char, DateResult, char)> =
  and!(P_GROUP, P_SEP, P_EMAIL, P_SEP, P_DATE, P_SEP);

#[cfg(test)]
mod tests {
  use crate::parser::parse_all;
  use crate::P_COMMIT_ROW;

  #[test]
  fn test_p_commit_row() {
    let res = parse_all(P_COMMIT_ROW, "Toby, sugto555@gmail.com, 1648863350 +1300");

    assert_eq!(res.is_some(), true);
  }
}
