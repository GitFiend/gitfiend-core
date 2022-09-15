use std::collections::{HashMap, HashSet};

use serde::Serialize;
use ts_rs::TS;

use crate::git::git_types::Patch;
use crate::git::queries::patches::cache::load_patches_cache;
use crate::git::queries::search::{FileMatch, SearchOptions};
use crate::git::store::COMMITS;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Hash, TS)]
#[ts(export)]
pub enum SearchMatchType {
  CommitId,
  CommitMessage,
  Email,
  Author,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CoreSearchResult {
  commit_id: String,
  matches: HashSet<SearchMatchType>,
  patches: Vec<Patch>,
  diffs: Vec<FileMatch>,
  ref_ids: Vec<String>,
}

pub fn search_commits(options: &SearchOptions) -> Option<Vec<CoreSearchResult>> {
  let SearchOptions {
    repo_path,
    search_text,
    num_results,
  } = options;

  let commits = COMMITS.get_by_key(repo_path)?;
  let patches = load_patches_cache(repo_path)?;
  let search_text = search_text.to_lowercase();
  let mut results: Vec<CoreSearchResult> = Vec::new();

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

    let matching_patches = get_matching_patches(&search_text, &commit.id, &patches);

    let ref_ids: Vec<String> = commit
      .refs
      .iter()
      .filter(|r| r.full_name.to_lowercase().contains(&search_text))
      .map(|r| r.id.clone())
      .collect();

    if !matches.is_empty() || !matching_patches.is_empty() || !ref_ids.is_empty() {
      results.push(CoreSearchResult {
        commit_id: commit.id.clone(),
        matches,
        patches: matching_patches,
        diffs: Vec::new(),
        ref_ids,
      });
    }

    if results.len() > *num_results {
      break;
    }
  }

  Some(results)
}

fn get_matching_patches(
  search_text: &str,
  commit_id: &str,
  patches: &HashMap<String, Vec<Patch>>,
) -> Vec<Patch> {
  if let Some(files) = patches.get(commit_id) {
    return files
      .iter()
      .filter(|p| {
        p.old_file.to_lowercase().contains(search_text)
          || p.new_file.to_lowercase().contains(search_text)
      })
      .cloned()
      .collect::<Vec<Patch>>();
  }

  Vec::new()
}
