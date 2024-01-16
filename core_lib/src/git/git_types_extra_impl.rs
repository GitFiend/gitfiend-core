use crate::git::git_types::{
  Hunk, HunkLine, HunkRange, Patch, PatchType, RefLocation, WipPatchType,
};
use std::ffi::OsStr;
use std::fmt;
use std::fmt::Formatter;
use std::path::Path;

impl fmt::Display for PatchType {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match *self {
      PatchType::A => write!(f, "A"),
      PatchType::B => write!(f, "B"),
      PatchType::C => write!(f, "C"),
      PatchType::D => write!(f, "D"),
      PatchType::M => write!(f, "M"),
      PatchType::R => write!(f, "R"),
      PatchType::T => write!(f, "T"),
      PatchType::U => write!(f, "U"),
      PatchType::X => write!(f, "X"),
    }
  }
}

impl fmt::Display for WipPatchType {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match *self {
      WipPatchType::Question => write!(f, "?"),
      WipPatchType::Empty => write!(f, " "),
      WipPatchType::Ignored => write!(f, "!"),
      WipPatchType::A => write!(f, "A"),
      WipPatchType::C => write!(f, "C"),
      WipPatchType::D => write!(f, "D"),
      WipPatchType::M => write!(f, "M"),
      WipPatchType::R => write!(f, "R"),
      WipPatchType::U => write!(f, "U"),
      WipPatchType::T => write!(f, "T"),
    }
  }
}

impl fmt::Display for RefLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match *self {
      RefLocation::Local => write!(f, "Local"),
      RefLocation::Remote => write!(f, "Remote"),
    }
  }
}

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

impl HunkLine {
  pub fn get_hunk<'a>(&self, hunks: &'a [Hunk]) -> Option<&'a Hunk> {
    if self.hunk_index >= 0 {
      return hunks.get(self.hunk_index as usize);
    }
    None
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

impl Patch {
  pub fn get_file_extension(&self) -> String {
    let file_name = if self.new_file.is_empty() {
      &self.old_file
    } else {
      &self.new_file
    };

    Path::new(file_name)
      .extension()
      .and_then(OsStr::to_str)
      .unwrap_or("")
      .to_string()
  }
}
