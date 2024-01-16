use crate::index::ac_node::ACNode;
use ahash::{HashMap, HashMapExt};

#[derive(Debug, Clone)]
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

  #[test]
  fn add_word_abcd_with_endings() {
    let mut index = ACIndex::new();

    index.add_word("ab");
    index.add_word("abcd");

    assert_eq!(
      index.find_matching("a"),
      ["ab".to_string(), "abcd".to_string()]
    );
  }

  #[test]
  fn char_boundaries() {
    let test: Vec<char> = "test".chars().collect();

    assert_eq!(test[1..].len(), 3);

    let text = "🍝️test";

    let mut it = text.chars();
    assert_eq!(it.next(), Some('🍝'));

    // U+FE0F Variation Selector-16
    assert_eq!(it.next(), Some('\u{fe0f}'));

    let test: Vec<char> = text.chars().collect();

    assert_eq!(test[0].to_string(), "🍝");
    assert_eq!(test[0], '🍝');
    assert_eq!(test[0..1], ['🍝']);
    assert_eq!(test[1], '\u{fe0f}');
    assert_eq!(test[0..].len(), 6);
    assert_eq!(test[1..].len(), 5);

    let s = String::from(text);

    assert_eq!(s.len(), 11);
    assert_eq!(&s[7..9], "te");

    assert!(s.is_char_boundary(0));
    assert!(!s.is_char_boundary(1));
    assert!(!s.is_char_boundary(2));
    assert!(!s.is_char_boundary(3));
    assert!(s.is_char_boundary(4));
    assert!(!s.is_char_boundary(5));
    assert!(!s.is_char_boundary(6));
    assert!(s.is_char_boundary(7));

    assert_eq!(&s[..4], "🍝");
    assert_eq!(&s[..7], "🍝\u{fe0f}");
  }
}
