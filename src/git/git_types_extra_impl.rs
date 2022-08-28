use crate::git::git_types::{Hunk, HunkRange, PatchType, RefLocation, WipPatchType};
use std::fmt;
use std::fmt::Formatter;

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
      WipPatchType::A => write!(f, "A"),
      WipPatchType::C => write!(f, "C"),
      WipPatchType::D => write!(f, "D"),
      WipPatchType::M => write!(f, "M"),
      WipPatchType::R => write!(f, "R"),
      WipPatchType::U => write!(f, "U"),
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

impl HunkRange {
  pub fn new() -> HunkRange {
    HunkRange {
      start: 0,
      length: 0,
    }
  }
}
