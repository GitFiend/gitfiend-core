use crate::parser::standard_parsers::{LINE_END, UNTIL_END, UNTIL_LINE_END, WS};
use crate::parser::{parse_part, Parser};
use crate::{and, character, many, map, take_char_while};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;

pub fn parse_http_request(request_text: String) -> Option<HttpRequest> {
  parse_part(HTTP_REQUEST, &request_text)
}

const NOT_WS: Parser<String> = take_char_while!(|c: char| { !c.is_whitespace() });

const METHOD_LINE: Parser<(String, String, String)> = map!(
  and!(NOT_WS, WS, NOT_WS, WS, NOT_WS, LINE_END),
  |result: (String, String, String, String, String, &str)| {
    let (method, _, url, _, protocol, _) = result;

    (method, url, protocol)
  }
);

const HEADER: Parser<(String, String)> = map!(
  and!(
    take_char_while!(|c: char| { !c.is_whitespace() && c != ':' }),
    character!(':'),
    WS,
    UNTIL_LINE_END
  ),
  |res: (String, char, String, String)| { (res.0, res.3) }
);

const HTTP_REQUEST: Parser<HttpRequest> = map!(
  and!(METHOD_LINE, many!(HEADER), LINE_END, UNTIL_END),
  |result: (
    (String, String, String),
    Vec<(String, String)>,
    &str,
    String
  )| {
    let ((method, url, protocol), headers, _, body) = result;

    HttpRequest {
      method,
      url,
      protocol,
      headers: headers.into_iter().collect(),
      body,
    }
  }
);

#[derive(Debug)]
pub struct HttpRequest {
  pub method: String,
  pub url: String,
  pub protocol: String,
  pub headers: HashMap<String, String>,
  pub body: String,
}

// pub fn handle_sync_request2<'a, O: Deserialize<'a>, R: Serialize>(
//   body: &'a str,
//   stream: &mut TcpStream,
//   handler: fn(&O) -> R,
//   url: &str,
// ) -> Result<(), Box<dyn Error>> {
//   let options = from_str(body)?;
//
//   let handler_result = handler(&options);
//   let serialized = serde_json::to_string(&handler_result)?;
//
//   let response = format!(
//     "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//     serialized.len(),
//     serialized
//   );
//
//   println!("completed {}", url);
//   stream.write_all(response.as_bytes())?;
//   // stream.flush()?;
//
//   Ok(())
// }

pub fn handle_sync_request<'a, O: Deserialize<'a>, R: Serialize>(
  body: &'a str,
  stream: &mut TcpStream,
  handler: fn(&O) -> R,
  url: &str,
) {
  if let Ok(options) = from_str(body) {
    let handler_result = handler(&options);
    if let Ok(serialized) = serde_json::to_string(&handler_result) {
      let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        serialized.len(),
        serialized
      );
      // "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"

      println!("completed {}, response: {}", url, response);
      let ok = stream.write_all(response.as_bytes());

      if ok.is_err() {
        println!("Failed to write response!")
      }
      stream.flush();
    } else {
      println!("Failed to serialise request options!")
    }
  } else {
    println!("Failed to parse request body!")
  }
}

#[macro_export]
macro_rules! requests {
  ($request:expr, $stream:expr, $($handler:ident),*) => {{
    let url = $request.url.as_str();
    let body = $request.body.as_str();

    match url {
      $(
      concat!("/", stringify!($handler)) => {
        $crate::server::custom_server::http::handle_sync_request(body, $stream, $handler, url);
      },
      )*
      unknown_url => {
        println!("Unknown url {}", unknown_url);
      }
    }
  }};
}

#[cfg(test)]
mod tests {
  use crate::parser::{parse_all, parse_part};
  use crate::server::custom_server::http::{HTTP_REQUEST, METHOD_LINE};

  const REQ_TEXT: &str = "POST / HTTP/1.1
HOST: 127.0.0.1:29996
content-type: application/json
content-length: 23

{
  \"repoPath\": \".\"
}";

  #[test]
  fn test_parse_header() {
    let result = parse_part(METHOD_LINE, REQ_TEXT);

    assert!(result.is_some());
    assert_eq!(
      result.unwrap(),
      ("POST".to_string(), "/".to_string(), "HTTP/1.1".to_string())
    );
  }

  #[test]
  fn test_start_server() {
    let result = parse_all(HTTP_REQUEST, REQ_TEXT);

    assert!(result.is_some());
  }
}
