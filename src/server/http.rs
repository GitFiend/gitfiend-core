use crate::parser::standard_parsers::{LINE_END, UNTIL_END, UNTIL_LINE_END, WS};
use crate::parser::{Parser, _parse_part};
use crate::{and, character, many, map, take_char_while};
use std::collections::HashMap;

pub fn parse_http_request(request_text: String) -> Option<HttpRequest> {
  _parse_part(HTTP_REQUEST, &request_text)
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

#[cfg(test)]
mod tests {
  use crate::parser::{_parse_part, parse_all};
  use crate::server::http::{HTTP_REQUEST, METHOD_LINE};

  const REQ_TEXT: &str = "POST / HTTP/1.1
HOST: 127.0.0.1:29996
content-type: application/json
content-length: 23

{
  \"repoPath\": \".\"
}";

  #[test]
  fn test_parse_header() {
    let result = _parse_part(METHOD_LINE, REQ_TEXT);

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
