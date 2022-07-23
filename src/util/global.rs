use std::hash::Hash;
use std::sync::RwLock;

use ahash::AHashMap;
use cached::once_cell::sync::Lazy;

#[macro_export]
macro_rules! global {
  ($value: expr) => {
    Global {
      data: cached::once_cell::sync::Lazy::new(|| std::sync::RwLock::new($value)),
    }
  };
}

pub struct Global<T: Clone> {
  pub data: Lazy<RwLock<T>>,
}

impl<T: Clone> Global<T> {
  pub fn set(&self, new_data: T) {
    if let Ok(mut data) = self.data.write() {
      (*data) = new_data;
    }
  }

  // The result of this should be considered potentially stale.
  pub fn get(&self) -> Option<T> {
    if let Ok(data) = self.data.read() {
      return Some((*data).clone());
    }
    None
  }
}

impl<V> Global<Vec<V>>
where
  V: Clone,
{
  pub fn push(&self, value: V) {
    if let Ok(mut data) = self.data.write() {
      data.push(value);
    }
  }

  pub fn clear(&self) {
    if let Ok(mut data) = self.data.write() {
      data.clear()
    }
  }
}

impl<K, V> Global<AHashMap<K, V>>
where
  K: Hash + Clone + Eq,
  V: Clone,
{
  pub fn insert(&self, key: K, value: V) {
    if let Ok(mut data) = self.data.write() {
      data.insert(key, value);
    }
  }

  pub fn get_by_key(&self, key: &K) -> Option<V> {
    if let Ok(data) = self.data.read() {
      return Some(data.get(key)?.clone());
    }
    None
  }

  pub fn clear(&self) {
    if let Ok(mut data) = self.data.write() {
      data.clear()
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::util::global::Global;

  static MY_GLOBAL: Global<Vec<i32>> = global!(Vec::new());

  #[test]
  fn test_global() {
    assert_eq!(MY_GLOBAL.get(), Some(Vec::new()));

    MY_GLOBAL.set(vec![1, 2, 3]);

    assert_eq!(MY_GLOBAL.get(), Some(vec![1, 2, 3]));
  }
}
