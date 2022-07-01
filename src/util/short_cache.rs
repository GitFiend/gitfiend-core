use ahash::AHashMap;
use std::hash::Hash;

pub struct ShortCache<K, V> {
  map: AHashMap<K, V>,
  name: String,
  duration: usize,
}

impl<K, V> ShortCache<K, V>
where
  K: Hash + Clone + Eq,
  V: Clone,
{
  fn new(name: String, duration: usize) -> Self {
    Self {
      map: AHashMap::new(),
      name,
      duration,
    }
  }

  fn get(&self, key: &K) -> Option<&V> {
    self.map.get(key)
  }

  fn insert(&mut self, key: K, value: V) {
    self.map.insert(key, value);
  }
}
