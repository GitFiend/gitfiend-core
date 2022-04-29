use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DateResult {
  pub ms: f32,
  pub adjustment: i32,
}

pub struct RefInfoPart {
  pub id: String,
  pub location: RefLocation,
  pub full_name: String,
  pub short_name: String,
  pub remote_name: Option<String>,
  pub sibling_id: Option<String>,
  pub ref_type: RefType,
  pub head: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum RefType {
  Branch,
  Tag,
  Stash,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum RefLocation {
  Local,
  Remote,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Commit {
  pub author: String,
  pub email: String,
  pub date: DateResult,
  pub id: String,
  pub index: usize,
  pub parent_ids: Vec<String>,
  pub is_merge: bool,
  pub message: String,
  pub stash_id: Option<String>,
  pub refs: Vec<RefInfo>,

  pub filtered: bool,
  pub num_skipped: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RefInfo {
  pub id: String,
  pub location: RefLocation,
  pub full_name: String,
  pub short_name: String,
  pub remote_name: Option<String>,
  pub sibling_id: Option<String>,
  pub ref_type: RefType,
  pub head: bool,
  pub commit_id: String,
  pub time: f32,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct GitConfig {
  pub entries: HashMap<String, String>,
  pub remotes: HashMap<String, String>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum PatchType {
  A,
  C,
  B,
  D,
  M,
  R,
  T,
  U,
  X,
}

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

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Patch {
  pub commit_id: String,
  pub old_file: String,
  pub new_file: String,
  pub patch_type: PatchType,
  pub id: String,
  pub is_image: bool,
}
