use ahash::{HashMap, HashMapExt};
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct ACNode {
  pub char: char,
  pub nodes: HashMap<char, ACNode>,
  pub end_of_word: bool,
}
impl ACNode {
  pub fn new(char: char, remaining: &mut Chars) -> ACNode {
    let mut n = ACNode {
      char,
      nodes: HashMap::new(),
      end_of_word: false,
    };

    n.add_word(remaining);

    n
  }

  pub fn add_word(&mut self, remaining: &mut Chars) {
    if let Some(c) = remaining.next() {
      if let Some(n) = self.nodes.get_mut(&c) {
        n.add_word(remaining);
      } else {
        self.nodes.insert(c, ACNode::new(c, remaining));
      }
    } else {
      self.end_of_word = true;
    }
  }

  pub fn get_word_endings(&self) -> Vec<String> {
    let mut matches = Vec::new();

    if self.end_of_word {
      matches.push(self.char.to_string());
    }

    for n in self.nodes.values() {
      for s in n.get_word_endings() {
        let end = format!("{}{}", self.char, s);

        matches.push(end);
      }
    }

    matches
  }
}

#[cfg(test)]
mod tests {
  use crate::index::ac_node::ACNode;

  #[test]
  fn add_word_abcd() {
    let remaining = String::from("bcd");
    let mut chars = remaining.chars();

    let node = ACNode::new('a', &mut chars);

    let endings = node.get_word_endings();

    assert_eq!(endings.len(), 1);
  }
}
