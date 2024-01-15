use crate::index::commit_message_ac::commit_message_ac;
use crate::index::create_branch_ac::create_branch_ac;
use crate::server::request_util::R;
use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct MessageAC {
  pub current_word: String,
  pub repo_path: String,
  pub max_num: usize,
  pub kind: ACType,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub enum ACType {
  CommitMessage,
  CreateBranch,
}

pub fn auto_complete(options: &MessageAC) -> R<Vec<String>> {
  let MessageAC {
    current_word,
    repo_path,
    max_num,
    kind,
  } = options;

  match kind {
    ACType::CommitMessage => commit_message_ac(repo_path, current_word, *max_num),
    ACType::CreateBranch => create_branch_ac(repo_path, current_word, *max_num),
  }
}
