use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct MessageAC {
  pub current_word: String,
  pub repo_path: String,
}

pub fn commit_message_ac(options: &MessageAC) {
  let MessageAC {
    current_word,
    repo_path,
  } = options;

  //
}
