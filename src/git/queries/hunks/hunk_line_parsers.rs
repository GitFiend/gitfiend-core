use crate::git::git_types::{HunkLine, HunkLineStatus, HunkRange};
use crate::parser::standard_parsers::{
  LINE_END, SIGNED_INT, UNTIL_LINE_END, UNTIL_LINE_END_KEEP, WS, WS_STR,
};
use crate::parser::Parser;
use crate::{and, character, many, map, or, until_parser_keep_happy, word};

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

struct Line {
  status: HunkLineStatus,
  text: String,
  line_ending: String,
}

pub const P_LINE_AND_END: Parser<(String, &str)> =
  and!(until_parser_keep_happy!(LINE_END), or!(LINE_END, WS_STR));

const P_UNCHANGED_LINE: Parser<Line> = map!(and!(character!(' '), P_LINE_AND_END), |res: (
  char,
  (String, &str)
)| {
  Line {
    status: HunkLineStatus::Unchanged,
    text: res.1 .0,
    line_ending: res.1 .1.to_string(),
  }
});

const P_ADDED_LINE: Parser<Line> = map!(and!(character!('+'), P_LINE_AND_END), |res: (
  char,
  (String, &str)
)| {
  Line {
    status: HunkLineStatus::Added,
    text: res.1 .0,
    line_ending: res.1 .1.to_string(),
  }
});

const P_REMOVED_LINE: Parser<Line> = map!(and!(character!('-'), P_LINE_AND_END), |res: (
  char,
  (String, &str)
)| {
  Line {
    status: HunkLineStatus::Removed,
    text: res.1 .0,
    line_ending: res.1 .1.to_string(),
  }
});

const P_NO_NEW_LINE: Parser<Line> = map!(and!(character!('\\'), UNTIL_LINE_END), |res: (
  char,
  String
)| {
  Line {
    status: HunkLineStatus::Unchanged,
    text: res.1,
    line_ending: String::from(""),
  }
});

const P_LINE_BREAK: Parser<Line> = map!(LINE_END, |res: &str| {
  Line {
    status: HunkLineStatus::Unchanged,
    text: String::from(""),
    line_ending: res.to_string(),
  }
});

const P_HUNK_LINE: Parser<HunkLine> = map!(
  or!(
    P_LINE_BREAK,
    P_UNCHANGED_LINE,
    P_ADDED_LINE,
    P_REMOVED_LINE,
    P_NO_NEW_LINE
  ),
  |line: Line| {
    HunkLine {
      status: line.status,
      old_num: None,
      new_num: None,
      hunk_index: -1,
      text: line.text,
      index: 0,
      line_ending: line.line_ending,
    }
  }
);

const P_HUNK_LINES: Parser<Vec<HunkLine>> = many!(P_HUNK_LINE);

#[cfg(test)]
mod tests {
  use crate::git::git_types::{HunkLineStatus, HunkRange};
  use crate::git::queries::hunks::hunk_line_parsers::{
    generate_line_ranges_text, P_ADDED_LINE, P_HUNK_LINE, P_HUNK_LINES, P_HUNK_LINE_RANGE,
    P_HUNK_LINE_RANGES,
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

  #[test]
  fn test_p_hunk_lines() {
    let hunk_line1 = " describe('test commits state', () => {\r\n";
    let hunk_line2 = "\n";
    let hunk_line3 = "-  it(`can load ${pathToThisRepo}`, async () => {\r\n";
    let hunk_line4 = "+  it('todo', () => {";

    let hunk_lines = format!("{hunk_line1}{hunk_line2}{hunk_line3}{hunk_line4}");

    let out = parse_all(P_HUNK_LINE, hunk_line1);

    assert!(out.is_some());
    assert_eq!(out.unwrap().status, HunkLineStatus::Unchanged);

    let out = parse_all(P_ADDED_LINE, hunk_line4);

    assert!(out.is_some());
    assert_eq!(out.unwrap().status, HunkLineStatus::Added);

    let out = parse_all(P_HUNK_LINES, &hunk_lines);

    assert!(out.is_some());

    let lines = out.unwrap();

    assert_eq!(lines.len(), 4);
  }
}
