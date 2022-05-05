use crate::git::git_types::HunkRange;
use crate::parser::standard_parsers::{SIGNED_INT, UNTIL_LINE_END, WS};
use crate::parser::Parser;
use crate::{and, character, map, or, word};

pub const P_HUNK_LINE_RANGE: Parser<HunkRange> = or!(
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

pub const P_HUNK_LINE_RANGES: Parser<(HunkRange, HunkRange)> = map!(
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
  use crate::git::queries::hunks::hunk_line_parsers::{
    generate_line_ranges_text, P_HUNK_LINE_RANGE, P_HUNK_LINE_RANGES,
  };

  use crate::parser::parse_all;

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
