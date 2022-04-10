mod refs;

use crate::git::git_types::{Commit, DateResult, RefInfo, RefInfoPart};
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

// Don't put a comma on the last one otherwise the macro will complain
const P_COMMIT_ROW: Parser<Commit> = map!(
  and!(
    /*  0 */ P_GROUP, // author
    /*  1 */ P_SEP,
    /*  2 */ P_EMAIL,
    /*  3 */ P_SEP,
    /*  4 */ P_DATE,
    /*  5 */ P_SEP,
    /*  6 */ P_GROUP, // commit id
    /*  7 */ P_SEP,
    /*  8 */ P_PARENTS,
    /*  9 */ P_SEP,
    /* 10 */ P_MESSAGE,
    /* 11 */ P_SEP,
    /* 12 */ P_OPTIONAL_REFS
  ),
  |result: (
    /*  0 */ String,
    /*  1 */ char,
    /*  2 */ String,
    /*  3 */ char,
    /*  4 */ DateResult,
    /*  5 */ char,
    /*  6 */ String,
    /*  7 */ char,
    /*  8 */ Vec<String>,
    /*  9 */ char,
    /* 10 */ String,
    /* 11 */ char,
    /* 12 */ Vec<RefInfoPart>,
  )| {
    let refs = result
      .12
      .into_iter()
      .map(|info: RefInfoPart| make_ref_info(info, result.6.to_owned(), result.4.ms))
      .collect::<Vec<RefInfo>>();

    let num_parents = result.8.len();

    Commit {
      author: result.0,
      email: result.2,
      date: result.4,
      id: result.6,
      index: 0,
      parent_ids: result.8,
      is_merge: num_parents > 1,
      message: result.10,
      stash_id: None,
      refs,
      filtered: false,
      num_skipped: 0,
    }
  }
);

fn make_ref_info(info: RefInfoPart, commit_id: String, time: i64) -> RefInfo {
  match info {
    RefInfoPart {
      id,
      location,
      full_name,
      short_name,
      remote_name,
      sibling_id,
      ref_type,
      head,
    } => RefInfo {
      id,
      location,
      full_name,
      short_name,
      remote_name,
      sibling_id,
      ref_type,
      head,
      commit_id,
      time,
    },
  }
}

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
