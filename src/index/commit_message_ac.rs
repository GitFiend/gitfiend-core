use crate::git::queries::patches::patches::load_patches;
use crate::git::store;
use crate::index::ac_index::ACIndex;
use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct MessageAC {
  pub current_word: String,
  pub repo_path: String,
}

pub fn commit_message_ac(options: &MessageAC) -> Option<Vec<String>> {
  let MessageAC {
    current_word,
    repo_path,
  } = options;

  let index = build_index(repo_path)?;

  Some(index.find_matching(current_word))
}

fn build_index(repo_path: &String) -> Option<ACIndex> {
  let (commits, refs) = store::get_commits_and_refs(repo_path)?;
  let patches = load_patches(repo_path, &commits)?;

  let mut index = ACIndex::new();

  for c in commits {
    index.add_word(&c.email);
    index.add_word(&c.author);
  }

  for r in refs {
    index.add_word(&r.short_name);
  }

  for c in patches.values() {
    for p in c {
      index.add_word(&p.new_file);
      index.add_word(&p.old_file);
    }
  }

  Some(index)
}
