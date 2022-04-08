use crate::input::Input;
use crate::parser::ParserResult;
use crate::parser::{Parser, Parser2};
use regex::Regex;

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

fn character4(c: char) -> impl Fn(Vec<char>, usize) -> Option<ParserResult<char>> {
  move |code: Vec<char>, pos: usize| {
    let r = code[pos];

    if r == c {
      Some(ParserResult {
        value: r,
        position: pos + 1,
      })
    } else {
      None
    }
  }
}

fn character5(c: char) -> Box<dyn Fn(Vec<char>, usize) -> Option<ParserResult<char>>> {
  Box::new(move |code: Vec<char>, pos: usize| {
    let r = code[pos];

    if r == c {
      Some(ParserResult {
        value: r,
        position: pos + 1,
      })
    } else {
      None
    }
  })
}

// macro_rules! regex_parser {
//   ($c:expr) => {
//     |input: &mut Input| -> Option<Regex> {
//       let re = Regex::new($c).unwrap();
//       Some(re)
//     }
//   };
// }
//
// fn check_thing(input: &mut Input) {
//   let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
//
//   let slice = &input.code[input.position..];
//   let s: &str = "omg";
//   let thing = &s[0..];
//   // let s2: &str = &[..input.code.as];
//   re.is_match_at(&[..input.code], input.position)
// }

// #[macro_export]
// macro_rules! and {
//   ($p:expr) => {
//     |input: &mut Input| {
//       let res = $p(input);
//
//       res
//     }
//   };
// }

#[macro_export]
macro_rules! and2 {
  ( $( $p:expr ),* ) => {
    |input: &mut Input| {
    let pos = input.position;

    $(
      let res = $p(input);

      res
    )*
    }
  };
}

/*

let r = p1(input);
let r2 = p2(input);

(r, r2)


 */

macro_rules! tup {
  () => {};
  ( $h:expr) => {
    $h
  };
  ( $h:expr, $($tail:tt)* ) => {
    $h, tup!($($tail)*)
  };
}

macro_rules! item_tuple {
  ($($arg:expr),*) => {
    (
      ($($arg,)*),
      ($($arg,)*)
    )
  }
}

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

// macro_rules! tup_inner {
//   () => {};
//   ( $h:expr) => {
//     $h
//   };
//   ( $h:expr, $($tail:tt)* ) => {
//     $h, tup_inner!($($tail)*)
//   };
// }
//
// #[macro_export]
// macro_rules! tup2 {
//   () => {};
//   ( $h:expr) => {
//     ($h)
//   };
//   ( $h:expr, $($tail:tt)* ) => {
//     ($h, tup_inner!($($tail)*))
//   };
// }

#[macro_export]
macro_rules! and3 {
  ( $( $p:expr ),* ) => {
    |input: &mut Input| {
    let pos = input.position;

    $(
      let res = $p(input);

      res
    )*
    }
  };
}

fn try_thing() -> (Option<i32>, Option<i32>, Option<i32>) {
  let mut ok = true;

  (
    { Some(3) },
    {
      ok = false;
      if ok {
        Some(2)
      } else {
        None
      }
    },
    {
      if ok {
        Some(2)
      } else {
        None
      }
    },
  )
}

fn try_thing2(exit: bool) -> Option<(i32, i32, i32)> {
  let r = (
    { 1 },
    {
      if exit {
        return None;
      } else {
        3
      }
    },
    { 2 },
  );

  Some(r)
}
// pub const P_REGEX: Parser2<Regex> = regex_parser!(r"^\d{4}-\d{2}-\d{2}$");
// const SECOND: Parser2<Regex> = P_REGEX;

const A: &str = "a";
// const B: &str = &A[..];

#[macro_export]
macro_rules! character3 {
  ($c:expr) => {
    |input: &mut Input| {
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

const ME: fn(&mut Input) = |input: &mut Input| {};
pub const P_3: Parser2<char> = character3!('3');
const ALL: fn(&mut Input) -> Option<(char, char, char)> = and!(P_3, P_3, P_3);

// const AND_THINGS: fn(&mut Input) -> Option<char> = and!(P_3);

macro_rules! character2 {
  ($c:expr) => {
    |code: Vec<char>, pos: usize| {
      let r = code[pos];

      if r == $c {
        Some(ParserResult {
          value: r,
          position: pos + 1,
        })
      } else {
        None
      }
    }
  };
}

macro_rules! vs {
  ($s:expr) => {
    String::from($s).chars().collect()
  };
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
  fn test_str() {
    let s: &str = "omg";
    let thing = &s[0..];

    assert_eq!(thing, "omg");
    assert_eq!(&s[1..], "mg");

    let c = &s[0..1];
    assert_eq!(c, "o");

    // let t = tup!(1, 2, 3, 4);
    let t2 = item_tuple!(1, 10, 100, 1000);

    let t3 = try_thing();

    // let t4 = and_again!(1, 2, 3);
    let t4 = and!(P_3, P_3, P_3);

    assert_eq!(try_thing2(false), Some((1, 3, 2)));
    assert_eq!(try_thing2(true), None);
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
    let p = character2!('3');

    // let text: Vec<char> = String::from("3").chars().collect();
    let text = vs!("3");

    let result = p(text, 0);
    assert_eq!(result.unwrap().value, '3');

    let mut input = Input::new("3");
    let p2 = character3!('3');
    let p3 = character3!('4');

    let result2 = p2(&mut input);

    input.set_position(0);
    let result3 = p3(&mut input);

    assert_eq!(result2.unwrap(), '3');

    input.set_position(0);

    // let result4 = AND_THINGS(&mut input);

    // assert_eq!(result4.unwrap(), '3');
  }

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

  // #[test]
  // fn test_word_parser_fail() {
  //   let p = word("omg2");
  //   let input = Input::new("omg");
  //
  //   assert_eq!(p(input), None)
  // }

  #[test]
  fn test_new_char_parser() {
    let text: Vec<char> = String::from("x").chars().collect();

    assert_eq!(text[0], 'x');

    let p = character5('x');
    let res = p(text, 0);

    // println!("{}", res);
    // p()
    // assert_eq!(p(input), None)
  }

  // #[test]
  // fn test_using_input_twice() {
  //   let input = Input::new("omg2");
  //
  //   let p = word("omg");
  //   let p2 = character('2');
  //
  //   assert_eq!(p(input), Some(String::from("omg")));
  //   assert_eq!(p2(input), None)
  // }
}
