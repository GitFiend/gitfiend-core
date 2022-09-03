use core::fmt;
use serde::Serialize;
use std::fmt::Formatter;
use ts_rs::TS;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ConflictedFile {
  pub lines: Vec<ConflictedFileLine>,
  pub sections: Vec<CFSection>,
  pub ref_name_top: String,
  pub ref_name_bottom: String,
  pub line_ending: String,
  pub max_line_length: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum CFLine {
  Ok(OkLine),
  Blank(BlankLine),
  Slot(SlotLine),
  Conflict(ConflictLine),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct OkLine {
  text: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct BlankLine {
  section: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SlotLine {
  section: usize,
  index: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ConflictLine {
  pub text: String,
  pub side: CFSide,
  pub section: usize,
  pub key: String,
}

impl ConflictedFile {
  pub fn new() -> Self {
    Self {
      lines: vec![],
      sections: vec![],
      ref_name_top: String::new(),
      ref_name_bottom: String::new(),
      line_ending: String::from("\n"),
      max_line_length: 0,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, TS)]
#[ts(export)]
pub struct ConflictedLine {
  pub text: String,
  pub blank: bool,
  pub side: CFSide,
  pub section: usize,
  pub conflicted: bool,
  pub key: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, TS)]
#[ts(export)]
pub struct ConflictedFileLine {
  pub text: Option<String>,
  // conflicted if section is some.
  pub section: Option<usize>,
  pub index: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, TS)]
#[ts(export)]
pub struct CFSection {
  pub left: Vec<ConflictedLine>,
  pub right: Vec<ConflictedLine>,
}

impl CFSection {
  pub fn get(&self, side: &CFSide) -> &Vec<ConflictedLine> {
    match side {
      CFSide::Left => &self.left,
      CFSide::Right => &self.right,
    }
  }

  pub fn get_mut(&mut self, side: &CFSide) -> &mut Vec<ConflictedLine> {
    match side {
      CFSide::Left => &mut self.left,
      CFSide::Right => &mut self.right,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, TS)]
#[ts(export)]
pub enum CFSide {
  Left,
  Right,
}

impl fmt::Display for CFSide {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match *self {
      CFSide::Left => write!(f, "Left"),
      CFSide::Right => write!(f, "Right"),
    }
  }
}
