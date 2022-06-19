use crate::git::queries::search::SearchOptions;
use serde::Serialize;
use std::collections::HashSet;
use ts_rs::TS;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Hash, TS)]
#[ts(export)]
pub enum SearchMatchType {
  RefName,
  CommitId,
  CommitMessage,
  FileName,
  Email,
  Author,
  Diff, // This is for combining in the client.
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SearchResult {
  commit_id: String,
  matches: HashSet<SearchMatchType>,
  // diff: TODO
}

pub fn search_commits(options: &SearchOptions) {
  //
}
