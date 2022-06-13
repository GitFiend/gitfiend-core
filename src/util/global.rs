use cached::once_cell::sync::Lazy;
use std::sync::RwLock;

#[macro_export]
macro_rules! global {
  () => {
    Global {
      data: cached::once_cell::sync::Lazy::new(|| std::sync::RwLock::new(None)),
    }
  };
}

pub struct Global<T: Clone> {
  pub data: Lazy<RwLock<Option<T>>>,
}

impl<T: Clone> Global<T> {
  pub fn set(&self, new_data: T) {
    if let Ok(mut data) = self.data.write() {
      (*data) = Some(new_data);
    }
  }

  pub fn get(&self) -> Option<T> {
    if let Ok(data) = self.data.read() {
      return (*data).clone();
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use crate::util::global::Global;

  static MY_GLOBAL: Global<Vec<i32>> = global!();

  #[test]
  fn test_global() {
    assert_eq!(MY_GLOBAL.get(), None);

    MY_GLOBAL.set(vec![1, 2, 3]);

    assert_eq!(MY_GLOBAL.get(), Some(vec![1, 2, 3]));
  }
}
