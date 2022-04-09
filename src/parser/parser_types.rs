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
macro_rules! or {
  ( $($p:expr),* ) => {
    |input: &mut Input| {
      $({
        let res = $p(input);

        if res.is_some() {
          return res
        }
      })*

      None
    }
  }
}

#[macro_export]
macro_rules! character {
  ($c:expr) => {
    |input: &mut Input| -> Option<char> {
      if !input.end() {
        let r = input.next_char();

        if r == $c {
          input.advance();
          return Some(r);
        }
      }
      None
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

#[macro_export]
macro_rules! take_char_while {
  ($function:expr) => {
    |input: &mut Input| -> Option<String> {
      let start_pos = input.position;

      while !input.end() && $function(input.next_char()) {
        input.advance();
      }

      if start_pos == input.position {
        None
      } else {
        Some(String::from_iter(&input.code[start_pos..input.position]))
      }
    }
  };
}

#[macro_export]
macro_rules! optional_take_char_while {
  ($function:expr) => {
    |input: &mut Input| -> Option<String> {
      let start_pos = input.position;

      while !input.end() && $function(input.next_char()) {
        input.advance();
      }

      if start_pos == input.position {
        Some(String::from(""))
      } else {
        Some(String::from_iter(&input.code[start_pos..input.position]))
      }
    }
  };
}

macro_rules! rep_parser_sep {
  ($parser:expr, $sep_parser:expr) => {
    |input: &mut Input| {
      let mut results = Vec::new();

      while !input.end() {
        let result = $parser(input);

        if result.is_some() {
          results.extend(result);
        } else {
          break;
        }

        let sep_result = $sep_parser(input);

        if sep_result.is_none() {
          break;
        }
      }

      Some(results)
    }
  };
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parser::parse_all;
  use crate::Input;

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
  fn test_or() {
    let result = parse_all(or!(word!("a"), word!("b")), "b");

    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap(), "b");

    let result = parse_all(or!(word!("a"), word!("b"), word!("p")), "c");

    assert_eq!(result.is_none(), true);
  }

  #[test]
  fn test_rep_parser_sep() {
    let parser: Parser<Vec<&str>> = rep_parser_sep!(word!("a"), word!(","));

    let result = parse_all(parser, "a,a,a");

    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().len(), 3);
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
