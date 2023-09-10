use crate::parser::Parser;
use crate::{
  and, character, conditional_char2, map, map2, optional_take_char_while, or, take_char_while,
  until_parser, word,
};
use bstr::BString;
use bstr::ByteSlice;
use bstr::B;

pub const ANY_WORD: Parser<BString> = take_char_while!(|c: char| { c.is_alphanumeric() });
pub const UNSIGNED_INT: Parser<BString> = take_char_while!(|c: char| { c.is_numeric() });
pub const SIGNED_INT: Parser<BString> = map!(
  and!(or!(word!(B("-")), word!(B("+")), WS_STR), UNSIGNED_INT),
  |res: (&[u8], BString)| { BString::from(bstr::concat(&[res.0, res.1.as_bytes()])) }
);

// TODO: Handle more cases.
pub const STRING_LITERAL: Parser<BString> = map2!(
  and!(
    character!('"'),
    take_char_while!(|c: char| c != '"'),
    character!('"')
  ),
  res,
  res.1
);

// const NUL: Parser<char> = conditional_char2!(c, c.is_control() && !c.is_whitespace());

pub const WS: Parser<BString> = optional_take_char_while!(|c: char| { c.is_whitespace() });
pub const WS_STR: Parser<&[u8]> = map2!(WS, _result, b"");

pub const LINE_END: Parser<&[u8]> = or!(word!(B("\n")), word!(B("\r\n")));
pub const UNTIL_LINE_END: Parser<BString> = until_parser!(LINE_END);
// pub const UNTIL_LINE_END_KEEP: Parser<(String, &str)> =
//   and!(until_parser_keep!(LINE_END), LINE_END);

pub const UNTIL_NUL: Parser<BString> =
  until_parser!(conditional_char2!(c, c.is_control() && !c.is_whitespace()));

// pub const UNTIL_END: Parser<String> = optional_take_char_while!(|c: char| { c != char::from(0) });

#[cfg(test)]
mod tests {
  use crate::parser::standard_parsers::{
    ANY_WORD, SIGNED_INT, UNSIGNED_INT, UNTIL_LINE_END, UNTIL_NUL, WS,
  };
  use crate::parser::{parse_all, parse_part, run_parser, ParseOptions};
  use crate::take_char_while;

  #[test]
  fn test_take_while() {
    let parser = take_char_while!(|c: char| { c.is_alphanumeric() });

    let result = parse_all(parser, b"abcd55");

    assert_eq!(result.unwrap(), "abcd55");
  }

  #[test]
  fn is_this_whitespace() {
    let c = '\r';

    assert!(c.is_whitespace());
    assert!(c.is_control());

    let c = '\0';

    assert!(!c.is_whitespace());
    assert!(c.is_control());
  }

  #[test]
  fn test_any_word() {
    let result = parse_all(ANY_WORD, b"abcd55");

    assert_eq!(result.unwrap(), "abcd55");

    // Should fail for non alpha-numeric.
    let result = parse_all(ANY_WORD, b"@@@@@");

    assert_eq!(result, None);
  }

  #[test]
  fn test_uint() {
    let result = parse_all(UNSIGNED_INT, b"1234");

    assert_eq!(result.unwrap(), "1234");

    // Should fail for non alpha-numeric.
    let result = parse_all(ANY_WORD, b"@@@@@");

    assert_eq!(result, None);
  }

  #[test]
  fn test_signed_int() {
    let result = parse_all(SIGNED_INT, b"1234");

    assert_eq!(result.unwrap(), "1234");

    // Should fail for non alpha-numeric.
    let result = run_parser(
      ANY_WORD,
      b"@@@@@",
      ParseOptions {
        must_parse_all: true,
        print_error: false,
      },
    );

    assert_eq!(result, None);

    assert_eq!(parse_all(SIGNED_INT, b"-1234").unwrap(), "-1234")
  }

  #[test]
  fn test_ws_parser() {
    let result = parse_all(WS, b" ");

    assert!(result.is_some());

    // Expect success even when nothing parsed.
    assert!(parse_all(WS, b"").is_some());

    assert!(parse_all(WS, b"\t").is_some());

    assert!(parse_all(WS, b"\n\n").is_some());

    assert!(run_parser(
      WS,
      b"ab",
      ParseOptions {
        must_parse_all: true,
        print_error: false
      }
    )
    .is_none());
  }

  #[test]
  fn test_until_line_end_parser() {
    let result = parse_part(UNTIL_LINE_END, b"asdfsdf&^HF JC\tasd !@\nasdf");

    assert!(result.is_some());
    assert_eq!(result.unwrap(), "asdfsdf&^HF JC\tasd !@");
  }

  #[test]
  fn test_until_nul() {
    let result = parse_all(UNTIL_NUL, b"omg\0");

    assert!(result.is_some());
    assert_eq!(result.unwrap(), "omg");
  }
}
