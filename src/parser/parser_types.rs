#[macro_export]
macro_rules! and {
  ( $($parser:expr),* ) => {
    |input: &mut Input| {
      let start_pos = input.position;

      Some((
        $({
          let res = $parser(input);

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
macro_rules! conditional_char {
  ($function:expr) => {
    |input: &mut Input| -> Option<char> {
      let start_pos = input.position;

      let c = input.next_char();

      if $function(c) {
        input.advance();

        Some(c)
      } else {
        None
      }
    }
  };
}

// Fails if no chars consumed. Successful conditional doesn't consume text.
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

// Parses up to and excludes the char in the conditional.
// Always succeeds.
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

// All is consumed, but $str is not included in the result.
#[macro_export]
macro_rules! until_str {
  ($str:expr) => {
    |input: &mut Input| -> Option<String> {
      let char_vec: Vec<char> = $str.chars().collect();
      let str_len = $str.len();
      let start_pos = input.position;
      let end = input.code.len() - str_len;

      while input.position <= end {
        let p = input.position;

        if &input.code[p..p + str_len] == char_vec {
          input.set_position(p + str_len);

          return Some(String::from_iter(&input.code[start_pos..p]));
        }

        input.advance();
      }

      input.set_position(start_pos);
      None
    }
  };
}

// Parses until parser is found or the end of input. Always succeeds.
// All text is consumed, but end parser result is not included (TODO: Check this)
#[macro_export]
macro_rules! until_parser {
  ($parser:expr) => {
    |input: &mut Input| -> Option<String> {
      let start_pos = input.position;
      let mut current_pos = start_pos;

      while !input.end() {
        current_pos = input.position;
        let result = $parser(input);

        if result.is_some() {
          break;
        }

        input.advance();
      }

      return Some(String::from_iter(&input.code[start_pos..current_pos]));
    }
  };
}

#[macro_export]
macro_rules! many {
  ($parser:expr) => {
    |input: &mut Input| {
      let mut results = Vec::new();

      while !input.end() {
        let result = $parser(input);

        if result.is_none() {
          break;
        }
        results.extend(result);
      }

      Some(results)
    }
  };
}

#[macro_export]
macro_rules! rep_sep {
  ($parser:expr, $sep:expr) => {
    rep_parser_sep!($parser, and!(WS, word!($sep), WS))
  };
}

#[macro_export]
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
  use crate::parser::standard_parsers::WS;
  use crate::parser::{parse_all, Parser, _parse_part};
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
  fn test_rep_sep() {
    let parser: Parser<Vec<&str>> = rep_sep!(word!("a"), ",");

    let result = parse_all(parser, "a, a , a");

    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().len(), 3);
  }

  #[test]
  fn test_until_str() {
    let parser: Parser<String> = until_str!("omg");

    let result = parse_all(parser, "aaaaaaaomg");

    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap(), "aaaaaaa");
  }

  #[test]
  fn test_many() {
    let parser = many!(character!('c'));

    let result = parse_all(parser, "cccccc");

    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 6);

    let result = _parse_part(parser, "x");

    // Succeed with no results.
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 0);
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

    let _t4 = and!(P_3, P_3, P_3);
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
  fn test_until_parser() {
    let parser: Parser<String> = until_parser!(word!("omg"));

    let result = parse_all(parser, "aaaaaaaomg");

    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap(), "aaaaaaa");
  }
}
