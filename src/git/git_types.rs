pub struct DateResult {
  pub ms: i64,
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

pub enum RefType {
  Branch,
  Tag,
  Stash,
}

#[derive(PartialEq, Eq, Debug)]
pub enum RefLocation {
  Local,
  Remote,
}

pub struct Commit {
  pub author: String,
  pub email: String,
  pub date: DateResult,
  pub id: String,
  pub index: i32,
  pub parent_ids: Vec<String>,
  pub is_merge: bool,
  pub message: String,
  pub stash_id: Option<String>,
  pub refs: Vec<RefInfo>,

  pub filtered: bool,
  pub num_skipped: u32,
}

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
  pub time: i64,
}
