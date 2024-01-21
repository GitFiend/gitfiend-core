use crate::f;
use serde::Serialize;
use ts_rs::TS;

pub type R<T> = Result<T, ES>;

#[derive(Debug, Clone, TS, Serialize)]
#[ts(export)]
pub enum ES {
  Text(String),
}

impl ES {
  pub fn from(text: &str) -> Self {
    Self::Text(text.to_string())
  }
}

impl<T> From<std::sync::PoisonError<T>> for ES {
  fn from(err: std::sync::PoisonError<T>) -> Self {
    ES::Text(err.to_string())
  }
}

impl From<std::io::Error> for ES {
  fn from(err: std::io::Error) -> Self {
    ES::Text(err.to_string())
  }
}

impl From<Box<dyn std::any::Any + Send>> for ES {
  fn from(err: Box<dyn std::any::Any + Send>) -> Self {
    ES::Text(f!("{:?}", err))
  }
}

impl From<std::path::StripPrefixError> for ES {
  fn from(err: std::path::StripPrefixError) -> Self {
    ES::Text(err.to_string())
  }
}
