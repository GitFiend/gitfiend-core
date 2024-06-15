use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqOptions {
  pub repo_path: String,
}

// #[derive(Debug, Deserialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export)]
// pub struct ActionOptions {
//   pub repo_path: String,
// }

// #[derive(Debug, Clone, Deserialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export)]
// pub struct ReqCommitsOptions {
//   pub repo_path: String,
//   pub num_commits: u32,
// }
