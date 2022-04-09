use crate::parser::input::Input;
use crate::parser::Parser;

#[macro_export]
macro_rules! and {
  ( $($p:expr),* ) => {
    |input: &mut Input| {
      let start_pos = input.position;

      Some((
        $({
          let res = $p(input);

          if res.is_none() {
            input.set_position(start_pos);
            return None;
          }
          else {
            res.unwrap()
          }
        },)*
      ))
    }
  }
}

#[macro_export]
macro_rules! character {
  ($c:expr) => {
    |input: &mut Input| -> Option<char> {
      let r = input.next_char();

      if r == $c {
        input.advance();
        Some(r)
      } else {
        None
      }
    }
  };
}

#[macro_export]
macro_rules! word {
  ($text:expr) => {
    |input: &mut Input| {
      let start_pos = input.position;

      for c in $text.chars() {
        if !input.end() && input.next_char() == c {
          input.advance();
        } else {
          input.set_position(start_pos);
          return None;
        }
      }

      Some($text)
    }
  };
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parser::parse_all;

  pub const P_3: Parser<char> = character!('3');
  const ALL: Parser<(char, char, char)> = and!(P_3, P_3, P_3);

  const HELLO: Parser<&str> = word!("hello");

  #[test]
  fn test_word() {
    let mut input = Input::new("hello");

    let result = HELLO(&mut input);

    assert_eq!(result.unwrap(), "hello");
  }

  #[test]
  fn test_word2() {
    let result = parse_all(word!("omg"), "omg");

    assert_eq!(result.unwrap(), "omg");

    let result = parse_all(word!("omg"), "omgg");

    // Expect parse fail due to not all text parsed.
    assert_eq!(result, None);
  }

  #[test]
  fn test_str() {
    let s: &str = "omg";
    let _cc = s.chars();
    let thing = &s[0..];

    assert_eq!(thing, "omg");
    assert_eq!(&s[1..], "mg");

    let c = &s[0..1];
    assert_eq!(c, "o");

    // let t4 = and_again!(1, 2, 3);
    let t4 = and!(P_3, P_3, P_3);
  }

  #[test]
  fn test_p_d() {
    let mut input = Input::new("3");

    let res = P_3(&mut input);

    assert_eq!(res.unwrap(), '3');

    let mut input = Input::new("333");

    let res = ALL(&mut input);

    assert_eq!(res.is_none(), false);

    println!("{}", res.unwrap().0)
  }

  #[test]
  fn test_mac() {
    let mut input = Input::new("3");
    let p2 = character!('3');
    let p3 = character!('4');

    let result2 = p2(&mut input);

    input.set_position(0);
    let result3 = p3(&mut input);

    assert_eq!(result2.unwrap(), '3');

    input.set_position(0);
  }
}
