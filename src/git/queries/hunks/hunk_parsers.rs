use crate::git::git_types::{Hunk, HunkLine, HunkLineStatus, HunkRange};
use crate::git::queries::hunks::hunk_line_parsers::{Line, P_HUNK_LINES, P_HUNK_LINE_RANGES};
use crate::parser::standard_parsers::{UNTIL_LINE_END, WS};
use crate::parser::Parser;
use crate::{and, map, or, word};

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

const P_HUNK: Parser<Hunk> = map!(
  and!(P_HUNK_LINE_RANGES, UNTIL_LINE_END, P_HUNK_LINES),
  |res: ((HunkRange, HunkRange), String, Vec<Line>)| {
    let old_line_range = res.0 .0;
    let new_line_range = res.0 .1;

    let old_num = old_line_range.start;
    let new_num = new_line_range.start;

    let hunk = Hunk {
      old_line_range,
      new_line_range,
      context_line: String::from(""),
      lines: get_hunk_lines(old_num, new_num, res.2),
      index: 0,
    };

    hunk
  }
);

fn get_hunk_lines(old_num: u32, new_num: u32, lines: Vec<Line>) -> Vec<HunkLine> {
  let mut old_num = old_num;
  let mut new_num = new_num;

  let mut hunk_lines: Vec<HunkLine> = Vec::new();

  for (i, line) in lines.into_iter().enumerate() {
    match line.status {
      HunkLineStatus::Unchanged => {
        hunk_lines.push(HunkLine::from_line(
          line,
          i as u32,
          -1,
          Some(old_num),
          Some(new_num),
        ));
        old_num += 1;
        new_num += 1;
      }
      HunkLineStatus::Added => {
        hunk_lines.push(HunkLine::from_line(line, i as u32, -1, None, Some(new_num)));
        new_num += 1;
      }
      HunkLineStatus::Removed => {
        hunk_lines.push(HunkLine::from_line(line, i as u32, -1, Some(old_num), None));
        old_num += 1;
      }
      _ => {}
    };
  }

  hunk_lines
}

#[cfg(test)]
mod tests {
  use crate::git::queries::hunks::hunk_parsers::P_DIFF_HEADER;
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
}
