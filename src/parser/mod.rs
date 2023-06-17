use crate::dprintln;
use crate::parser::input::Input;

pub(crate) mod input;
mod parser_types;
pub(crate) mod standard_parsers;

pub type Parser<T> = fn(&mut Input) -> Option<T>;

pub fn parse_all<T>(parser: Parser<T>, text: &str) -> Option<T> {
  run_parser(
    parser,
    text,
    ParseOptions {
      must_parse_all: true,
      print_error: true,
    },
  )
}

pub fn parse_all_err<T>(parser: Parser<T>, text: &str) -> Result<T, String> {
  run_parser_err(
    parser,
    text,
    ParseOptions {
      must_parse_all: true,
      print_error: false,
    },
  )
}

// Beware: Doesn't complain.
pub fn parse_part<T>(parser: Parser<T>, text: &str) -> Option<T> {
  run_parser(
    parser,
    text,
    ParseOptions {
      must_parse_all: false,
      print_error: false,
    },
  )
}

pub struct ParseOptions {
  pub must_parse_all: bool,
  pub print_error: bool,
}

pub fn run_parser<T>(parser: Parser<T>, text: &str, options: ParseOptions) -> Option<T> {
  let mut input = Input::new(text);

  let result = parser(&mut input);

  if options.must_parse_all && !input.end() {
    if options.print_error {
      eprintln!("{}", get_error_message(&input));
    }

    return None;
  }

  result
}

pub fn run_parser_err<T>(
  parser: Parser<T>,
  text: &str,
  options: ParseOptions,
) -> Result<T, String> {
  let mut input = Input::new(text);

  if let Some(res) = parser(&mut input) {
    if options.must_parse_all && !input.end() {
      let message = get_error_message(&input);

      if options.print_error {
        dprintln!("{}", message);
      }

      return Err(message);
    }

    return Ok(res);
  }

  let message = get_error_message(&input);

  if options.print_error {
    dprintln!("{}", message);
  }

  Err(message)
}

fn get_error_message(input: &Input) -> String {
  format!(
    r#"
PARSE FAILURE AT POSITION {}:
      SUCCESSFULLY PARSED:
    "{}"

    FAILED AT:
    "{}"
    "#,
    input.attempted_position,
    input.successfully_parsed(),
    input.unparsed()
  )
}

#[macro_export]
macro_rules! map {
  ($parser:expr, $function:expr) => {
    |input: &mut $crate::parser::input::Input| {
      if let Some(result) = $parser(input) {
        Some($function(result))
      } else {
        None
      }
    }
  };
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{character, word};

  #[test]
  fn test_map() {
    let my_parser = map!(word!("omg"), String::from);

    let res = parse_all(my_parser, "omg");

    assert_eq!(res.unwrap(), String::from("omg"));
  }

  #[test]
  fn test_map2() {
    let my_parser = map!(character!('c'), String::from);

    let res = parse_all(my_parser, "c");

    assert_eq!(res.unwrap(), String::from("c"));
  }
}
