use crate::input::Input;

fn character(c: char) -> Box<dyn Fn(Input) -> Option<char>> {
  Box::new(move |mut input: Input| {
    let r = input.next_char();

    if r == c {
      input.advance();

      Some(r)
    } else {
      None
    }
  })
}

fn word(str: &str) -> Box<dyn Fn(Input) -> Option<String>> {
  let text = String::from(str);

  Box::new(move |mut input: Input| {
    let p = input.position;

    for c in text.chars() {
      if input.next_char() == c {
        input.advance();
      } else {
        input.set_position(p);
        return None;
      }
    }

    Some(text.clone())
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_char() {
    let p = character('3');

    let input = Input::new("3");

    assert_eq!(p(input), Some('3'));

    let input = Input::new("2");

    assert_eq!(p(input), None);
  }

  #[test]
  fn test_word_parser_success() {
    let p = word("omg");
    let input = Input::new("omg");

    assert_eq!(p(input), Some(String::from("omg")))
  }

  #[test]
  fn test_word_parser_fail() {
    let p = word("omg2");
    let input = Input::new("omg");

    assert_eq!(p(input), None)
  }
}
