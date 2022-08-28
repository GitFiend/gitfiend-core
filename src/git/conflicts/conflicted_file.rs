use core::fmt;
use serde::Serialize;
use std::fmt::Formatter;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ConflictedFile {
  pub lines: Vec<ConflictedFileLine>,
  pub sections: Vec<ConflictedSection>,
  pub ref_name_top: String,
  pub ref_name_bottom: String,
  pub line_ending: String,
  pub max_line_length: usize,
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

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ConflictedFileLine {
  pub text: Option<String>,
  pub section: Option<usize>,
  pub index: usize,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ConflictedSection {
  pub left: Vec<ConflictedLine>,
  pub right: Vec<ConflictedLine>,
}

impl ConflictedSection {
  pub fn get(&self, side: &ConflictedSide) -> &Vec<ConflictedLine> {
    match side {
      ConflictedSide::Left => &self.left,
      ConflictedSide::Right => &self.right,
    }
  }

  pub fn get_mut(&mut self, side: &ConflictedSide) -> &mut Vec<ConflictedLine> {
    match side {
      ConflictedSide::Left => &mut self.left,
      ConflictedSide::Right => &mut self.right,
    }
  }
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub enum ConflictedSide {
  Left,
  Right,
}

impl fmt::Display for ConflictedSide {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match *self {
      ConflictedSide::Left => write!(f, "Left"),
      ConflictedSide::Right => write!(f, "Right"),
    }
  }
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ConflictedLine {
  pub text: String,
  pub blank: bool,
  pub side: ConflictedSide,
  pub section: usize,
  pub conflicted: bool,
  pub key: Option<String>,
}
