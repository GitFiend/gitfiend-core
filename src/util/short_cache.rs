use ahash::AHashMap;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct ShortCache<V> {
  map: AHashMap<String, V>,
  name: String,
  duration: Duration,
  last_access: Instant,
}

impl<V> ShortCache<V>
where
  V: Clone,
{
  pub fn new(name: String, duration: Duration) -> Self {
    Self {
      map: AHashMap::new(),
      name,
      duration,
      last_access: Instant::now(),
    }
  }

  pub fn get(&mut self, key: &str) -> Option<&V> {
    let now = Instant::now();

    let duration_since = now - self.last_access;

    if duration_since > self.duration {
      self.map.clear();

      return None;
    }

    self.map.get(key)
  }

  pub fn insert(&mut self, key: &str, value: V) {
    self.last_access = Instant::now();

    self.map.insert(String::from(key), value);
  }
}
