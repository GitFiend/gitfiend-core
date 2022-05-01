use crate::git::git_types::HunkRange;
use crate::parser::standard_parsers::{SIGNED_INT, UNTIL_LINE_END, WS};
use crate::parser::Parser;
use crate::{and, character, map, or, word};
use std::fmt::format;

const P_DIFF_LINE: Parser<(&str, String)> = and!(word!("diff"), UNTIL_LINE_END);

// TODO: Not sure what part to return res.0 or res.1?
const P_OPTIONAL_HEADER: Parser<String> = or!(
  map!(and!(word!("deleted"), UNTIL_LINE_END), |res: (
    &str,
    String
  )| res
    .0
    .to_string()),
  map!(and!(word!("new file"), UNTIL_LINE_END), |res: (
    &str,
    String
  )| res
    .0
    .to_string()),
  WS
);

const P_INDEX_LINE: Parser<(&str, String)> = and!(word!("index"), UNTIL_LINE_END);

const P_OLD_FILE: Parser<(&str, String)> = and!(word!("---"), UNTIL_LINE_END);

const P_NEW_FILE: Parser<(&str, String)> = and!(word!("+++"), UNTIL_LINE_END);

const P_BINARY_INFO: Parser<(&str, String)> = and!(word!("Binary"), UNTIL_LINE_END);

struct FileInfo {
  is_binary: bool,
}
const P_FILE_INFO: Parser<FileInfo> = or!(
  map!(and!(P_OLD_FILE, P_NEW_FILE), |_: _| FileInfo {
    is_binary: false
  }),
  map!(P_BINARY_INFO, |_: _| FileInfo { is_binary: true })
);

const P_DIFF_HEADER: Parser<((&str, String), String, (&str, String), FileInfo)> =
  and!(P_DIFF_LINE, P_OPTIONAL_HEADER, P_INDEX_LINE, P_FILE_INFO);

const P_HUNK_LINE_RANGE: Parser<HunkRange> = or!(
  map!(and!(SIGNED_INT, character!(','), SIGNED_INT), |res: (
    String,
    char,
    String
  )| {
    HunkRange {
      start: res.0.parse::<i64>().unwrap_or(0).abs() as u32,
      length: res.2.parse().unwrap_or(0),
    }
  }),
  map!(SIGNED_INT, |res: String| {
    HunkRange {
      start: res.parse::<i64>().unwrap_or(0).abs() as u32,
      length: 1,
    }
  })
);

const P_HUNK_LINE_RANGES: Parser<(HunkRange, HunkRange)> = map!(
  and!(
    word!("@@"),
    WS,
    P_HUNK_LINE_RANGE,
    WS,
    P_HUNK_LINE_RANGE,
    WS,
    word!("@@")
  ),
  |res: (&str, String, HunkRange, String, HunkRange, String, &str)| { (res.2, res.4) }
);

pub fn generate_line_ranges_text(range: &(HunkRange, HunkRange)) -> String {
  let (old, new) = range;

  format!(
    "@@ -{},{} +{},{} @@",
    old.start, old.length, new.start, new.length
  )
}

#[cfg(test)]
mod tests {
  use crate::git::git_types::HunkRange;
  use crate::git::queries::hunks::hunk_parsers::{
    generate_line_ranges_text, P_DIFF_HEADER, P_HUNK_LINE_RANGE, P_HUNK_LINE_RANGES,
  };
  use crate::parser::parse_all;

  #[test]
  fn test_p_diff_header() {
    let diff_header = "diff --git a/src2/renderer-process/redux-store/repo-state/commits/commits-reducer.test.ts b/src2/renderer-process/redux-store/repo-state/commits/commits-reducer.test.ts
index 4296fe4..5b0d387 100644
--- a/src2/renderer-process/redux-store/repo-state/commits/commits-reducer.test.ts
+++ b/src2/renderer-process/redux-store/repo-state/commits/commits-reducer.test.ts";

    let result = parse_all(P_DIFF_HEADER, diff_header);

    assert!(result.is_some());
  }

  #[test]
  fn test_p_hunk_line_range() {
    let res = parse_all(P_HUNK_LINE_RANGE, "-1,19");

    assert!(res.is_some());
    assert_eq!(
      res.unwrap(),
      HunkRange {
        start: 1,
        length: 19
      }
    );

    let res = parse_all(P_HUNK_LINE_RANGE, "+1");
    assert!(res.is_some());
    assert_eq!(
      res.unwrap(),
      HunkRange {
        start: 1,
        length: 1
      }
    );

    let res = parse_all(P_HUNK_LINE_RANGE, "-1");
    assert!(res.is_some());
    assert_eq!(
      res.unwrap(),
      HunkRange {
        start: 1,
        length: 1
      }
    );
  }

  #[test]
  fn test_p_hunk_line_ranges() {
    let res = parse_all(P_HUNK_LINE_RANGES, "@@ -1 +1,2 @@");

    assert!(res.is_some());

    let ranges = res.unwrap();

    assert_eq!(
      ranges,
      (
        HunkRange {
          start: 1,
          length: 1
        },
        HunkRange {
          start: 1,
          length: 2
        }
      )
    );

    assert_eq!(generate_line_ranges_text(&ranges), "@@ -1,1 +1,2 @@");

    let res = parse_all(P_HUNK_LINE_RANGES, "@@ -0,0 +1,26 @@");

    assert!(res.is_some());

    let ranges = res.unwrap();

    assert_eq!(
      ranges,
      (
        HunkRange {
          start: 0,
          length: 0
        },
        HunkRange {
          start: 1,
          length: 26
        }
      )
    );

    assert_eq!(generate_line_ranges_text(&ranges), "@@ -0,0 +1,26 @@");

    let res = parse_all(P_HUNK_LINE_RANGES, "@@ -1,19 +1,17 @@");

    assert!(res.is_some());

    let ranges = res.unwrap();

    assert_eq!(
      ranges,
      (
        HunkRange {
          start: 1,
          length: 19
        },
        HunkRange {
          start: 1,
          length: 17
        }
      )
    );

    assert_eq!(generate_line_ranges_text(&ranges), "@@ -1,19 +1,17 @@");
  }
}
