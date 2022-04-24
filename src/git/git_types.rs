use serde::Serialize;
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Debug, Serialize, TS)]
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

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub enum RefType {
  Branch,
  Tag,
  Stash,
}

#[derive(PartialEq, Eq, Debug, Serialize, TS)]
#[ts(export)]
pub enum RefLocation {
  Local,
  Remote,
}

#[derive(Debug, Serialize, TS)]
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

#[derive(Debug, Serialize, TS)]
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

#[derive(PartialEq, Eq, Debug, Serialize, TS)]
#[ts(export)]
pub enum PatchType {
  A,
  C,
  D,
  M,
  R,
  T,
  U,
  X,
  B,
}

#[derive(Debug, Serialize, TS)]
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
