mod refs;

use crate::git::git_types::{DateResult, RefInfoPart};
use crate::git::queries::refs::P_OPTIONAL_REFS;
use crate::parser::standard_parsers::{ANY_WORD, SIGNED_INT, UNSIGNED_INT, WS};
use crate::parser::Parser;
use crate::{and, map, or, rep_parser_sep, take_char_while, until_str, word};
use crate::{character, Input};

const END: &str = "4a41380f-a4e8-4251-9ca2-bf55186ed32a";

const P_GROUP: Parser<String> = take_char_while!(|c: char| { c != ',' });
const P_SEP: Parser<char> = map!(and!(WS, character!(','), WS), |_res: (
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

const P_ANYTHING: Parser<(String, char, String)> = and!(P_GROUP, P_SEP, P_EMAIL);

const P_COMMIT_ROW: Parser<(
  /*  1 */ String,
  /*  2 */ char,
  /*  3 */ String,
  /*  4 */ char,
  /*  5 */ DateResult,
  /*  6 */ char,
  /*  7 */ String,
  /*  8 */ char,
  /*  9 */ Vec<String>,
  /* 10 */ char,
  /* 11 */ String,
  /* 12 */ char,
  /* 13 */ Vec<RefInfoPart>,
)> = and!(
  /*  1 */ P_GROUP, // author
  /*  2 */ P_SEP,
  /*  3 */ P_EMAIL,
  /*  4 */ P_SEP,
  /*  5 */ P_DATE,
  /*  6 */ P_SEP,
  /*  7 */ P_GROUP, // commit id
  /*  8 */ P_SEP,
  /*  9 */ P_PARENTS,
  /* 10 */ P_SEP,
  /* 11 */ P_MESSAGE,
  /* 12 */ P_SEP,
  /* 13 */ P_OPTIONAL_REFS
); // Don't put a comma on the last one otherwise the macro will complain

#[cfg(test)]
mod tests {
  use crate::git::queries::{P_COMMIT_ROW, P_GROUP};
  use crate::parser::{parse_all, parse_part};

  #[test]
  fn test_p_group() {
    let result = parse_part(P_GROUP, "omg,");

    assert!(result.is_some());
  }

  #[test]
  fn test_p_commit_row() {
    let res = parse_all(
      P_COMMIT_ROW,
      "Toby, sugto555@gmail.com, 1648863350 +1300, \
      dd5733ad96082f0f33164bd1e2d72f7540bf7d9f, 2e8966986f620f491c34e6243a546d85dd2322e0, \
      Write commit row parser. Added necessary new git types. 4a41380f-a4e8-4251-9ca2-bf55186ed32a\
      ,  (HEAD -> refs/heads/master, refs/remotes/origin/master)",
    );

    assert_eq!(res.is_some(), true);
  }
}
