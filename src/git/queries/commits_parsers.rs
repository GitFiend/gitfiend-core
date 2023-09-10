use crate::git::git_types::{CommitInfo, DateResult};
use crate::git::queries::refs::{make_ref_info, RefInfoPart};
use crate::git::queries::{RefInfo, P_OPTIONAL_REFS};
use crate::parser::standard_parsers::{ANY_WORD, SIGNED_INT, UNSIGNED_INT, UNTIL_LINE_END, WS};
use crate::parser::Parser;
use crate::{and, character, many, map, or, rep_parser_sep, take_char_while, until_str};
use bstr::BString;

const END: &str = "4a41380f-a4e8-4251-9ca2-bf55186ed32a";
pub const PRETTY_FORMATTED: &str =
  "--pretty=format:%an; %ae; %ad; %H; %P; %B4a41380f-a4e8-4251-9ca2-bf55186ed32a; %d";

pub const SEP_CHAR: char = ';';

pub const P_GROUP: Parser<BString> = take_char_while!(|c: char| { c != SEP_CHAR });

const P_SEP: Parser<char> = map!(and!(WS, character!(SEP_CHAR), WS), |_res: (
  BString,
  char,
  BString
)| { SEP_CHAR });

const P_EMAIL: Parser<BString> = or!(P_GROUP, WS);

const P_DATE: Parser<DateResult> = map!(and!(UNSIGNED_INT, WS, SIGNED_INT), |res: (
  BString,
  BString,
  BString
)| {
  DateResult {
    ms: res.0.to_string().parse::<f32>().unwrap() * 1000.0,
    adjustment: res.2.to_string().parse().unwrap(),
  }
});

const P_PARENTS: Parser<Vec<BString>> = rep_parser_sep!(ANY_WORD, WS);

const P_MESSAGE: Parser<BString> = until_str!(END);

type PCommitResult = (
  /*  0 */ BString,
  /*  1 */ char,
  /*  2 */ BString,
  /*  3 */ char,
  /*  4 */ DateResult,
  /*  5 */ char,
  /*  6 */ BString,
  /*  7 */ char,
  /*  8 */ Vec<BString>,
  /*  9 */ char,
  /* 10 */ BString,
  /* 11 */ char,
  /* 12 */ Vec<RefInfoPart>,
  /* 13 */ BString,
);

// Don't put a comma on the last one otherwise the macro will complain
pub const P_COMMIT_ROW: Parser<CommitInfo> = map!(
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
  |result: PCommitResult| {
    let refs = result
      .12
      .into_iter()
      .map(|info: RefInfoPart| make_ref_info(info, result.6.to_string(), result.4.ms))
      .collect::<Vec<RefInfo>>();

    let num_parents = result.8.len();

    CommitInfo {
      author: result.0.to_string(),
      email: result.2.to_string(),
      date: result.4,
      id: result.6.to_string(),
      index: 0,
      parent_ids: result.8.into_iter().map(|id| id.to_string()).collect(),
      is_merge: num_parents > 1,
      message: result.10.to_string(),
      stash_id: None,
      refs,
      filtered: false,
      num_skipped: 0,
    }
  }
);

pub const P_COMMITS: Parser<Vec<CommitInfo>> = many!(P_COMMIT_ROW);

pub const P_ID_LIST: Parser<Vec<BString>> = rep_parser_sep!(ANY_WORD, UNTIL_LINE_END);
