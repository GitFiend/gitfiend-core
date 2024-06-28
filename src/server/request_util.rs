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

#[macro_export]
macro_rules! parse_json {
  ($request: expr) => {{
    let mut content = String::new();

    if let Err(_e) = $request.as_reader().read_to_string(&mut content) {
      dprintln!("{}", _e);
      return;
    }

    match serde_json::from_str(&content) {
      Ok(options) => options,
      Err(_e) => {
        dprintln!("{}", _e);
        None
      }
    }
  }};
}

#[macro_export]
macro_rules! send_response {
  ($request: expr, $result: expr) => {{
    let result = serde_json::to_string(&$result);

    match result {
      Ok(serialized) => {
        match $request.respond(Response::from_string(serialized)) {
          Ok(_) => {}
          Err(_e) => {
            dprintln!("{}", _e);
          }
        };
      }
      Err(_e) => {
        dprintln!("{}", _e);
      }
    }
  }};
}

#[macro_export]
macro_rules! handle_request {
  ($request:expr, $handler: ident) => {{
    match $crate::parse_json!($request) {
      Some(options) => {
        $crate::time_block!(stringify!($handler), {
          $crate::send_response!($request, $handler(&options));
        });
      }
      None => {}
    };
  }};
}

#[macro_export]
macro_rules! handle_function_request {
  ($request:expr, $($handler:ident),*) => {{
    match $request.url() {
      $(
      concat!("/f/", stringify!($handler)) => {
        $crate::handle_request!($request, $handler);
      },
      )*
      _unknown_url => {
        dprintln!("Unknown url {}", _unknown_url);
      }
    }
  }};
}
