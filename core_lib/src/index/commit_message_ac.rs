use crate::git::git_types::Commit;
use crate::git::queries::patches::patches::load_patches;
use crate::git::request_util::{ES, R};
use crate::git::store::STORE;
use crate::global;
use crate::index::ac_index::ACIndex;
use crate::util::global::Global;

pub fn commit_message_ac(
  repo_path: &String,
  current_word: &String,
  max_num: usize,
) -> R<Vec<String>> {
  if current_word.is_empty() {
    return Ok(Vec::new());
  }

  let index = get_index(repo_path)?;

  let words = index.find_matching(current_word);

  Ok(words.into_iter().take(max_num).collect())
}

#[derive(Clone)]
struct CommitMessageAC {
  pub repo_path: String,
  pub index: ACIndex,
}

impl CommitMessageAC {
  fn new() -> Self {
    Self {
      repo_path: String::from(""),
      index: ACIndex::new(),
    }
  }
}

static INDEX: Global<CommitMessageAC> = global!(CommitMessageAC::new());

fn get_index(repo_path: &String) -> R<ACIndex> {
  if let Some(index) = INDEX.get() {
    if &index.repo_path == repo_path {
      return Ok(index.index);
    }
  }

  let index = build_index(repo_path)?;

  INDEX.set(CommitMessageAC {
    repo_path: repo_path.clone(),
    index: index.clone(),
  });

  Ok(index)
}

fn build_index(repo_path: &String) -> R<ACIndex> {
  let (commits, refs) = STORE
    .get_commits_and_refs(repo_path)
    .ok_or(ES::from("build_index: Couldn't get commits and refs."))?;
  let patches = load_patches(repo_path, &commits)?;

  let mut index = ACIndex::new();

  for c in commits {
    index.add_word(&c.email);
    index.add_word(&c.author);

    let message_words = get_words_in_commit_message(&c);

    for w in message_words {
      index.add_word(&w);
    }
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

  Ok(index)
}

fn get_words_in_commit_message(commit: &Commit) -> Vec<String> {
  let mut words: Vec<String> = Vec::new();
  let mut word: Vec<char> = Vec::new();

  for c in commit.message.chars() {
    if !char::is_whitespace(c) {
      word.push(c);
    } else {
      if word.len() > 6 {
        words.push(word.iter().collect());
      }
      word.clear();
    }
  }

  words
}
