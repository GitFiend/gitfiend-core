#[macro_export]
macro_rules! and {
  ( $($parser:expr),* ) => {
    |input: &mut $crate::parser::input::Input| {
      let start_pos = input.position;

      Some((
        $({
          if let Some(res) = $parser(input) {
            res
          }
          else {
            input.set_position(start_pos);
            return None;
          }
        },)*
      ))
    }
  }
}

// fn character(c: char) -> impl Fn(&mut Input) -> Option<char> {
//   move |input: &mut Input| -> Option<char> {
//     if !input.end() {
//       let r = input.next_char();
//
//       if r == c {
//         input.advance();
//         return Some(r);
//       }
//     }
//     None
//   }
// }

#[macro_export]
macro_rules! or {
  ( $($p:expr),* ) => {
    |input: &mut $crate::parser::input::Input| {
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
    |input: &mut $crate::parser::input::Input| -> Option<char> {
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
    |input: &mut $crate::parser::input::Input| {
      let start_pos = input.position;

      #[allow(unused_imports)]
      use bstr::ByteSlice;

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
    |input: &mut $crate::parser::input::Input| -> Option<char> {
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

#[macro_export]
macro_rules! conditional_char2 {
  ($name: ident, $function: expr) => {
    |input: &mut $crate::parser::input::Input| -> Option<char> {
      let $name = input.next_char();

      if $function {
        input.advance();

        Some($name)
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
    |input: &mut $crate::parser::input::Input| -> Option<bstr::BString> {
      let start_pos = input.position;

      while !input.end() && $function(input.next_char()) {
        input.advance();
      }

      if start_pos == input.position {
        None
      } else {
        Some(bstr::BString::from_iter(
          (&input.code[start_pos..input.position]).iter().cloned(),
        ))
      }
    }
  };
}

// Parses up to and excludes the char in the conditional.
// Always succeeds.
#[macro_export]
macro_rules! optional_take_char_while {
  ($function:expr) => {
    |input: &mut $crate::parser::input::Input| -> Option<bstr::BString> {
      let start_pos = input.position;

      while !input.end() && $function(input.next_char()) {
        input.advance();
      }

      if start_pos == input.position {
        Some(BString::from(""))
      } else {
        Some(bstr::BString::from_iter(
          (&input.code[start_pos..input.position]).iter().cloned(),
        ))
      }
    }
  };
}

// All is consumed, but $str is not included in the result.
#[macro_export]
macro_rules! until_str {
  ($str:expr) => {
    |input: &mut $crate::parser::input::Input| -> Option<bstr::BString> {
      #[allow(unused_imports)]
      use bstr::ByteSlice;

      let char_vec: Vec<char> = $str.chars().collect();
      let str_len = $str.len();
      let start_pos = input.position;
      let end = input.code.len() - str_len;

      while input.position <= end {
        let p = input.position;

        if &input.code[p..p + str_len] == char_vec {
          input.set_position(p + str_len);

          return Some(BString::from_iter(input.code[start_pos..p].iter().cloned()));
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
  ($parser: expr) => {
    |input: &mut $crate::parser::input::Input| -> Option<BString> {
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

      return Some(BString::from_iter(
        input.code[start_pos..current_pos].iter().cloned(),
      ));
    }
  };
}

// Parses until parser is found. Fails if parser is not found.
// Only text before parser is consumed, end parser result is not included
#[macro_export]
macro_rules! until_parser_keep {
  ($parser:expr) => {
    |input: &mut $crate::parser::input::Input| -> Option<bstr::BString> {
      let start_pos = input.position;

      while !input.end() {
        let current_pos = input.position;
        let result = $parser(input);

        if result.is_some() {
          input.set_position(current_pos);

          return Some(BString::from_iter(
            input.code[start_pos..current_pos].iter().cloned(),
          ));
        }

        input.advance();
      }

      return None;
    }
  };
}

// Parses until parser is found.
// Only text before parser is consumed, end parser result is not included
// Always succeeds
#[macro_export]
macro_rules! until_parser_keep_happy {
  ($parser:expr) => {
    |input: &mut $crate::parser::input::Input| -> Option<bstr::BString> {
      let start_pos = input.position;

      while !input.end() {
        let current_pos = input.position;
        let result = $parser(input);

        if result.is_some() {
          input.set_position(current_pos);

          break;
        }

        input.advance();
      }

      return Some(BString::from_iter(
        input.code[start_pos..input.position].into_iter().cloned(),
      ));
    }
  };
}

#[macro_export]
macro_rules! many {
  ($parser:expr) => {
    |input: &mut $crate::parser::input::Input| {
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
    |input: &mut $crate::parser::input::Input| {
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
  use bstr::{BString, B};

  use crate::parser::input::Input;
  use crate::parser::standard_parsers::WS;
  use crate::parser::{parse_all, parse_part, run_parser, ParseOptions, Parser};

  pub const P_3: Parser<char> = character!('3');
  const ALL: Parser<(char, char, char)> = and!(P_3, P_3, P_3);

  const HELLO: Parser<&[u8]> = word!(b"hello");

  #[test]
  fn test_word() {
    let mut input = Input::new(b"hello");

    let result = HELLO(&mut input);

    assert_eq!(result.unwrap(), b"hello");
  }

  #[test]
  fn test_word2() {
    let result = parse_all(word!(b"omg"), b"omg");

    assert_eq!(result.unwrap(), b"omg");

    let result = parse_all(word!(b"omg"), b"omgg");

    // Expect parse fail due to not all text parsed.
    assert_eq!(result, None);
  }

  #[test]
  fn test_or() {
    let result = parse_all(or!(word!(b"a"), word!(b"b")), b"b");

    assert!(result.is_some());
    assert_eq!(result.unwrap(), b"b");

    let result = parse_all(or!(word!(b"a"), word!(b"b"), word!(b"p")), b"c");

    assert!(result.is_none());
  }

  #[test]
  fn test_rep_parser_sep() {
    let parser: Parser<Vec<&[u8]>> = rep_parser_sep!(word!(B("a")), word!(b","));

    let result = parse_all(parser, b"a,a,a");

    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 3);
  }

  #[test]
  fn test_rep_sep() {
    let parser: Parser<Vec<&[u8]>> = rep_sep!(word!(B("a")), b",");

    let result = parse_all(parser, b"a, a , a");

    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 3);
  }

  #[test]
  fn test_until_str() {
    let parser: Parser<BString> = until_str!(B("omg"));

    let result = parse_all(parser, b"aaaaaaaomg");

    assert!(result.is_some());
    assert_eq!(result.unwrap(), "aaaaaaa");
  }

  #[test]
  fn test_many() {
    let parser = many!(character!('c'));

    let result = parse_all(parser, b"cccccc");

    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 6);

    let result = parse_part(parser, b"x");

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
    let mut input = Input::new(b"3");

    let res = P_3(&mut input);

    assert_eq!(res.unwrap(), '3');

    let mut input = Input::new(b"333");

    let res = ALL(&mut input);

    assert!(res.is_some());

    println!("{}", res.unwrap().0)
  }

  #[test]
  fn test_until_parser() {
    let parser: Parser<BString> = until_parser!(word!(b"omg"));

    let result = parse_all(parser, b"aaaaaaaomg");

    assert!(result.is_some());
    assert_eq!(result.unwrap(), BString::from("aaaaaaa"));
  }

  #[test]
  fn test_until_parser_keep() {
    let parser = and!(until_parser_keep!(word!(b"omg")), word!(b"omg"));

    let result = parse_all(parser, b"aaaaaaaomg");

    assert!(result.is_some());
    assert_eq!(result.unwrap(), (BString::from(b"aaaaaaa"), b"omg"));

    let parser = until_parser_keep!(word!(b"omg"));

    let result = run_parser(
      parser,
      b"aaaaaaaa",
      ParseOptions {
        must_parse_all: true,
        print_error: false,
      },
    );

    assert!(result.is_none());
  }
}
