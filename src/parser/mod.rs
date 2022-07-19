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
      eprintln!(
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
      );
    }

    return None;
  }

  result
}

#[macro_export]
macro_rules! map {
  ($parser:expr, $function:expr) => {
    |input: &mut crate::parser::input::Input| {
      let result = $parser(input);

      if result.is_some() {
        Some($function(result.unwrap()))
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
    let my_parser = map!(word!("omg"), |result: &str| String::from(result));

    let res = parse_all(my_parser, "omg");

    assert_eq!(res.unwrap(), String::from("omg"));
  }

  #[test]
  fn test_map2() {
    let my_parser = map!(character!('c'), |result: char| String::from(result));

    let res = parse_all(my_parser, "c");

    assert_eq!(res.unwrap(), String::from("c"));
  }
}
