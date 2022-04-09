use crate::parser::standard_parsers::{UNSIGNED_INT, WS};
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

const P_DATE: Parser<(String, String, String)> = and!(UNSIGNED_INT, WS, UNSIGNED_INT);

pub const P_COMMIT_ROW: Parser<(String, char, String, char)> = and!(P_GROUP, P_SEP, P_EMAIL, P_SEP);

#[cfg(test)]
mod tests {
  use crate::parser::parse_all;
  use crate::P_COMMIT_ROW;

  #[test]
  fn test_p_commit_row() {
    let res = parse_all(P_COMMIT_ROW, "Toby, sugto555@gmail.com");

    assert_eq!(res.is_some(), true);
  }
}
