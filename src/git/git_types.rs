use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DateResult {
  pub ms: usize,
  pub adjustment: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, TS)]
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
pub struct CommitInfo {
  pub author: String,
  pub email: String,
  pub date: DateResult,
  pub id: String,
  pub index: usize,
  pub parent_ids: Vec<String>,
  pub is_merge: bool,
  pub message: String,
  pub stash_id: String,
  pub refs: Vec<RefInfo>,

  pub filtered: bool,
  pub num_skipped: u32,
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
  pub stash_id: String,
  pub refs: Vec<String>,

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
  pub sibling_id: String,
  pub ref_type: RefType,
  pub head: bool,
  pub commit_id: String,
  pub time: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct LocalRefCommitDiff {
  pub ahead_of_remote: u32,
  pub behind_remote: u32,
  pub ahead_of_head: u32,
  pub behind_head: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RefCommitDiff {
  pub ahead_of_head: u32,
  pub behind_head: u32,
}

// See https://git-scm.com/docs/git-status for meaning.
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Hunk {
  pub old_line_range: HunkRange,
  pub new_line_range: HunkRange,
  pub context_line: String,
  pub lines: Vec<HunkLine>,
  pub index: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct HunkLine {
  pub status: HunkLineStatus,
  pub old_num: Option<i32>,
  pub new_num: Option<i32>,
  pub hunk_index: i32,
  pub text: String,
  pub index: u32,
  pub line_ending: String,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum HunkLineStatus {
  #[serde(rename = "+")]
  Added,
  #[serde(rename = "-")]
  Removed,
  #[serde(rename = " ")]
  Unchanged,
  HeaderStart,
  HeaderEnd,
  Skip,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct HunkRange {
  pub start: i32,
  pub length: i32,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum WipPatchType {
  #[serde(rename = "?")]
  Question,
  #[serde(rename = " ")]
  Empty,
  #[serde(rename = "!")]
  Ignored,
  A,
  C,
  D,
  M,
  R,
  U,
  T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct WipPatch {
  pub old_file: String,
  pub new_file: String,
  pub patch_type: WipPatchType,
  pub staged_type: WipPatchType,
  pub un_staged_type: WipPatchType,
  pub conflicted: bool,
  pub id: String,
  pub is_image: bool,
}
