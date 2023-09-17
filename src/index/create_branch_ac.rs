use crate::git::store::get_commits_and_refs;
use crate::index::ac_index::ACIndex;
use crate::server::request_util::{ES, R};

pub fn create_branch_ac(repo_path: &String, current_word: &str, max_num: usize) -> R<Vec<String>> {
  let (_, refs) =
    get_commits_and_refs(repo_path).ok_or(ES::from("create_branch_ac: Couldn't get refs."))?;
  let mut index = ACIndex::new();

  for r in refs {
    index.add_word(&r.short_name);
  }

  Ok(
    index
      .find_matching(current_word)
      .into_iter()
      .take(max_num)
      .collect(),
  )
}
