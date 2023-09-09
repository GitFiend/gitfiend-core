use std::fmt::Display;
/*
TODO: Stop passing options into each request as reference.

Stop using unwrap in these macros.

Convert macros where possible to functions.
 */

#[macro_export]
macro_rules! parse_json {
  ($request: expr) => {{
    let mut content = String::new();
    $request.as_reader().read_to_string(&mut content).unwrap();

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
    let serialized = serde_json::to_string(&$result).unwrap();

    match $request.respond(Response::from_string(serialized)) {
      Ok(_) => {}
      Err(e) => {
        dprintln!("{}", e);
      }
    };
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

pub type R<T> = Result<T, String>;
pub fn to_r<T: Display>(e: T) -> String {
  e.to_string()
}
