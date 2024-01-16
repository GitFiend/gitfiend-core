use crate::git::git_types::{Hunk, HunkLine, HunkLineStatus, HunkRange};
use crate::git::queries::hunks::hunk_line_parsers::{Line, P_HUNK_LINES, P_HUNK_LINE_RANGES};
use crate::parser::standard_parsers::{UNTIL_LINE_END, WS};
use crate::parser::Parser;
use crate::{and, many, map, map2, or, word};

type IgnoredLine<'a> = (&'a str, String);

const P_DIFF_LINE: Parser<IgnoredLine> = and!(word!("diff"), UNTIL_LINE_END);

const P_OPTIONAL_HEADER: Parser<String> = or!(
  map2!(and!(word!("deleted"), UNTIL_LINE_END), __, {
    String::from("deleted")
  }),
  map2!(and!(word!("new file"), UNTIL_LINE_END), __, {
    String::from("new file")
  }),
  P_RENAME_HEADER,
  WS
);

/*
Parse something like:

similarity index 88%
rename from BetterName.txt
rename to BetterName2.txt
 */
const P_RENAME_HEADER: Parser<String> = map2!(
  and!(
    and!(word!("similarity"), UNTIL_LINE_END),
    and!(word!("rename"), UNTIL_LINE_END),
    and!(word!("rename"), UNTIL_LINE_END)
  ),
  __,
  String::from("rename")
);

const P_INDEX_LINE: Parser<IgnoredLine> = and!(word!("index"), UNTIL_LINE_END);

const P_OLD_FILE: Parser<IgnoredLine> = and!(word!("---"), UNTIL_LINE_END);

const P_NEW_FILE: Parser<IgnoredLine> = and!(word!("+++"), UNTIL_LINE_END);

const P_BINARY_INFO: Parser<IgnoredLine> = and!(word!("Binary"), UNTIL_LINE_END);

struct FileInfo {
  is_binary: bool,
}
const P_FILE_INFO: Parser<FileInfo> = or!(
  map2!(
    and!(P_OLD_FILE, P_NEW_FILE),
    __,
    FileInfo { is_binary: false }
  ),
  map2!(P_BINARY_INFO, __, FileInfo { is_binary: true })
);

const P_DIFF_HEADER: Parser<FileInfo> = map2!(
  and!(P_DIFF_LINE, P_OPTIONAL_HEADER, P_INDEX_LINE, P_FILE_INFO),
  res,
  res.3
);

const P_HUNK: Parser<Hunk> = map2!(
  and!(P_HUNK_LINE_RANGES, UNTIL_LINE_END, P_HUNK_LINES),
  res,
  {
    let old_line_range = res.0 .0;
    let new_line_range = res.0 .1;

    let old_num = old_line_range.start;
    let new_num = new_line_range.start;

    Hunk {
      old_line_range,
      new_line_range,
      context_line: String::from(""),
      lines: get_hunk_lines(old_num, new_num, res.2),
      index: -1,
    }
  }
);

pub const P_HUNKS: Parser<Vec<Hunk>> = map!(and!(P_DIFF_HEADER, many!(P_HUNK)), |res: (
  FileInfo,
  Vec<Hunk>
)| {
  if res.0.is_binary {
    return vec![Hunk {
      old_line_range: HunkRange {
        start: 0,
        length: 0,
      },
      new_line_range: HunkRange {
        start: 0,
        length: 0,
      },
      context_line: String::from(""),
      lines: Vec::new(),
      index: 0,
    }];
  }

  res
    .1
    .into_iter()
    .enumerate()
    .map(|(i, mut hunk)| {
      let index = i as i32;

      hunk.index = index;

      hunk.lines = hunk
        .lines
        .into_iter()
        .map(|mut line| {
          line.hunk_index = index;
          line
        })
        .collect();

      hunk
    })
    .collect()
});

fn get_hunk_lines(old_num: i32, new_num: i32, lines: Vec<Line>) -> Vec<HunkLine> {
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
  use crate::git::queries::hunks::hunk_parsers::{P_DIFF_HEADER, P_HUNK};
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
  fn test_p_hunk() {
    let line_range = "@@ -1,19 +1,17 @@";
    let hunk_line1 = " describe('test commits state', () => {\r\n";
    let hunk_line2 = "\n";
    let hunk_line3 = "-  it(`can load ${pathToThisRepo}`, async () => {\r\n";
    let hunk_line4 = "+  it('todo', () => {";
    let hunk_lines = format!("{hunk_line1}{hunk_line2}{hunk_line3}{hunk_line4}");

    let hunk_text = format!("{}\n{}", line_range, hunk_lines);

    let out = parse_all(P_HUNK, &hunk_text);

    assert!(out.is_some());
  }
}
