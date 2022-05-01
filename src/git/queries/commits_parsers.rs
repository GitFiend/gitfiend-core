use crate::git::git_types::{Commit, DateResult};
use crate::git::queries::refs::{make_ref_info, RefInfoPart};
use crate::git::queries::{RefInfo, P_OPTIONAL_REFS};
use crate::parser::standard_parsers::{ANY_WORD, SIGNED_INT, UNSIGNED_INT, UNTIL_LINE_END, WS};
use crate::parser::Parser;
use crate::{and, character, many, map, or, rep_parser_sep, take_char_while, until_str};

const END: &str = "4a41380f-a4e8-4251-9ca2-bf55186ed32a";
pub const PRETTY_FORMATTED: &str =
  "--pretty=format:%an, %ae, %ad, %H, %P, %B4a41380f-a4e8-4251-9ca2-bf55186ed32a, %d";

pub const P_GROUP: Parser<String> = take_char_while!(|c: char| { c != ',' });

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
    ms: res.0.parse::<f32>().unwrap() * 1000.0,
    adjustment: res.2.parse().unwrap(),
  }
});

const P_PARENTS: Parser<Vec<String>> = rep_parser_sep!(ANY_WORD, WS);

const P_MESSAGE: Parser<String> = until_str!(END);

// Don't put a comma on the last one otherwise the macro will complain
pub const P_COMMIT_ROW: Parser<Commit> = map!(
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
    /* 12 */ P_OPTIONAL_REFS,
    /* 13 */ WS
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
    /* 13 */ String
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

pub const P_COMMITS: Parser<Vec<Commit>> = many!(P_COMMIT_ROW);

pub const P_ID_LIST: Parser<Vec<String>> = rep_parser_sep!(ANY_WORD, UNTIL_LINE_END);
