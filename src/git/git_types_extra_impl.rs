use crate::git::git_types::{PatchType, WipPatchType};
use std::fmt;

impl fmt::Display for PatchType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
