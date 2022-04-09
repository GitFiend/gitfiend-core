use crate::parser::input::Input;
use crate::parser::Parser;
use crate::{optional_take_char_while, take_char_while};

pub const ANY_WORD: Parser<String> = take_char_while!(|c: char| { c.is_alphanumeric() });
pub const UNSIGNED_NUMBER: Parser<String> = take_char_while!(|c: char| { c.is_numeric() });
pub const WS: Parser<String> = optional_take_char_while!(|c: char| { c.is_whitespace() });

#[cfg(test)]
mod tests {
  use crate::parser::input::Input;
  use crate::parser::standard_parsers::{ANY_WORD, UNSIGNED_NUMBER, WS};
  use crate::parser::{parse_all, run_parser, ParseOptions};
  use crate::take_char_while;

  #[test]
  fn test_take_while() {
    let parser = take_char_while!(|c: char| { c.is_alphanumeric() });

    let result = parse_all(parser, "abcd55");

    assert_eq!(result.unwrap(), "abcd55");
  }

  #[test]
  fn test_any_word() {
    let result = parse_all(ANY_WORD, "abcd55");

    assert_eq!(result.unwrap(), "abcd55");

    // Should fail for non alpha-numeric.
    let result = parse_all(ANY_WORD, "@@@@@");

    assert_eq!(result, None);
  }

  #[test]
  fn test_uint() {
    let result = parse_all(UNSIGNED_NUMBER, "1234");

    assert_eq!(result.unwrap(), "1234");

    // Should fail for non alpha-numeric.
    let result = parse_all(ANY_WORD, "@@@@@");

    assert_eq!(result, None);
  }

  #[test]
  fn test_ws_parser() {
    let result = parse_all(WS, " ");

    assert_eq!(result.is_some(), true);

    // Expect success even when nothing parsed.
    assert_eq!(parse_all(WS, "").is_some(), true);

    assert_eq!(parse_all(WS, "\t").is_some(), true);

    assert_eq!(parse_all(WS, "\n\n").is_some(), true);

    assert_eq!(
      run_parser(
        WS,
        "ab",
        ParseOptions {
          must_parse_all: true,
          print_error: false
        }
      )
      .is_none(),
      true
    );
  }
}
