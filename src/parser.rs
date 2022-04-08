use crate::Input;

pub type Parser<T> = fn(&mut Input) -> Option<T>;

pub fn parse_all<T>(parser: Parser<T>, text: &str) -> Option<T> {
  parse_inner(parser, text, true)
}

fn parse_inner<T>(parser: Parser<T>, text: &str, must_parse_all: bool) -> Option<T> {
  let mut input = Input::new(text);

  let result = parser(&mut input);

  if must_parse_all && !input.end() {
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

    return None;
  }

  result
}
