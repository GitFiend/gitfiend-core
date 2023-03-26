use ahash::{HashMap, HashMapExt};
use std::str::Chars;

#[derive(Debug)]
pub struct ACIndex {
  nodes: HashMap<char, ACNode>,
}
impl ACIndex {
  pub fn new() -> Self {
    Self {
      nodes: HashMap::new(),
    }
  }

  pub fn add_word(&mut self, word: &str) {
    let mut chars = word.chars();

    if let Some(c) = chars.next() {
      if let Some(n) = self.nodes.get_mut(&c) {
        n.add_word(&mut chars);
      } else {
        self.nodes.insert(c, ACNode::new(c, &mut chars));
      }
    }
  }

  pub fn find_matching(&self, word_prefix: &str) -> Vec<String> {
    let chars = word_prefix.chars();

    let mut nodes = &self.nodes;

    let mut endings = Vec::new();

    for c in chars {
      if let Some(n) = nodes.get(&c) {
        nodes = &n.nodes;
      } else {
        // failed to match.
        return endings;
      }
    }

    for n in nodes.values() {
      let e = n.get_word_endings();

      endings.extend(e);
    }

    endings.sort();

    endings
      .into_iter()
      .map(|suffix| format!("{}{}", word_prefix, suffix))
      .collect()
  }
}

#[derive(Debug)]
struct ACNode {
  char: char,
  nodes: HashMap<char, ACNode>,
}
impl ACNode {
  fn new(char: char, remaining: &mut Chars) -> ACNode {
    let mut n = ACNode {
      char,
      nodes: HashMap::new(),
    };

    n.add_word(remaining);

    n
  }

  fn add_word(&mut self, remaining: &mut Chars) {
    if let Some(c) = remaining.next() {
      if let Some(n) = self.nodes.get_mut(&c) {
        n.add_word(remaining);
      } else {
        self.nodes.insert(c, ACNode::new(c, remaining));
      }
    }
  }

  fn get_word_endings(&self) -> Vec<String> {
    if self.nodes.is_empty() {
      return vec![self.char.to_string()];
    }

    let mut matches = Vec::new();

    for n in self.nodes.values() {
      for s in n.get_word_endings() {
        matches.push(format!("{}{}", self.char, s));
      }
    }

    matches
  }
}

#[cfg(test)]
mod tests {
  use crate::index::ac_index::ACIndex;

  #[test]
  fn add_word_abcd() {
    let mut index = ACIndex::new();

    index.add_word("abcd");

    assert_eq!(index.find_matching("a"), ["abcd".to_string()]);
  }

  #[test]
  fn add_multiple_words() {
    let mut index = ACIndex::new();

    index.add_word("abcd");
    index.add_word("abcd");
    index.add_word("aaaa");
    index.add_word("abbb");
    index.add_word("bbbb");
    index.add_word("dddd");

    assert_eq!(
      index.find_matching("a"),
      ["aaaa".to_string(), "abbb".to_string(), "abcd".to_string()]
    );

    assert_eq!(index.find_matching("b"), ["bbbb".to_string()]);
  }
}
