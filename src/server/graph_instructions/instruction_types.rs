use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PointInstruction {
  pub x: usize,
  pub y: usize,
  pub colour: String,
  pub is_merge: bool,
  pub commit_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CommitPoint {
  pub x: usize,
  pub y: usize,
  pub commit_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LineInstruction {
  pub points: Vec<CommitPoint>,
  pub colour: String,
  pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Instructions {
  pub points: Vec<PointInstruction>,
  pub lines: Vec<LineInstruction>,
}
