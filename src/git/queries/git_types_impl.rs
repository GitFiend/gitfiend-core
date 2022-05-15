use crate::git::git_types::{Hunk, HunkRange};

impl Hunk {
  pub fn new() -> Hunk {
    Hunk {
      lines: Vec::new(),
      old_line_range: HunkRange::new(),
      new_line_range: HunkRange::new(),
      context_line: String::new(),
      index: -1,
    }
  }
}

impl HunkRange {
  pub fn new() -> HunkRange {
    HunkRange {
      start: 0,
      length: 0,
    }
  }
}
