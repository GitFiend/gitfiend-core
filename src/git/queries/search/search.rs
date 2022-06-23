use crate::git::queries::commits::COMMITS;
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

pub fn search_commits(options: &SearchOptions) -> Option<Vec<SearchResult>> {
  let SearchOptions {
    repo_path,
    search_text,
    num_results,
  } = options;

  let commits = COMMITS.get_by_key(repo_path)?;
  let search_text = search_text.to_lowercase();
  let mut results: Vec<SearchResult> = Vec::new();

  for commit in commits {
    let mut matches: HashSet<SearchMatchType> = HashSet::new();

    if commit.id.to_lowercase().contains(&search_text) {
      matches.insert(SearchMatchType::CommitId);
    }
    if commit.email.to_lowercase().contains(&search_text) {
      matches.insert(SearchMatchType::Email);
    }
    if commit.author.to_lowercase().contains(&search_text) {
      matches.insert(SearchMatchType::Author);
    }
    if commit.message.to_lowercase().contains(&search_text) {
      matches.insert(SearchMatchType::CommitMessage);
    }

    results.push(SearchResult {
      commit_id: commit.id,
      matches,
    });

    if results.len() > *num_results {
      break;
    }
  }

  Some(results)
}
