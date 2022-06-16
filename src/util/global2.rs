use cached::once_cell::sync::Lazy;
use std::ops::Deref;
use std::sync::RwLock;

#[macro_export]
macro_rules! global2 {
  ($value: expr) => {
    Global2 {
      data: cached::once_cell::sync::Lazy::new(|| std::sync::RwLock::new($value)),
    }
  };
}

pub struct Global2<T: Clone> {
  pub data: Lazy<RwLock<T>>,
}

impl<T: Clone> Global2<T> {
  pub fn set(&self, new_data: T) {
    if let Ok(mut data) = self.data.write() {
      (*data) = new_data;
    }
  }

  pub fn get(&self) -> Option<T> {
    if let Ok(data) = self.data.read() {
      return Some((*data).clone());
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use crate::util::global2::Global2;

  static MY_GLOBAL: Global2<Vec<i32>> = global2!(Vec::new());

  #[test]
  fn test_global() {
    assert_eq!(MY_GLOBAL.get(), Some(Vec::new()));

    MY_GLOBAL.set(vec![1, 2, 3]);

    assert_eq!(MY_GLOBAL.get(), Some(vec![1, 2, 3]));
  }
}
