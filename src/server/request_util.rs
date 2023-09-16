use ts_rs::TS;

// TODO: Try refactor to use this instead of R
pub type R2<T> = Result<T, ES>;

#[derive(Debug, Clone, TS)]
#[ts(export)]
pub enum ES {
  Text(String),
}

impl From<std::io::Error> for ES {
  fn from(err: std::io::Error) -> Self {
    ES::Text(err.to_string())
  }
}

pub type R<T> = Result<T, String>;

#[macro_export]
macro_rules! parse_json {
  ($request: expr) => {{
    let mut content = String::new();

    if let Err(e) = $request.as_reader().read_to_string(&mut content) {
      dprintln!("{}", e);
      return;
    }

    match serde_json::from_str(&content) {
      Ok(options) => options,
      Err(e) => {
        dprintln!("{}", e);
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
          Err(e) => {
            dprintln!("{}", e);
          }
        };
      }
      Err(e) => {
        dprintln!("{}", e);
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
      unknown_url => {
        dprintln!("Unknown url {}", unknown_url);
      }
    }
  }};
}
