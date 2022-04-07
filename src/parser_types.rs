use crate::input::Input;
use crate::parser::Parser;
use crate::parser::ParserResult;

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

// fn characterF(c: char) -> Fn(Vec<char>, usize) -> Option<Result<char>> {
//   |code: Vec<char>, pos: usize| {
//
//   }
// }

// fn character3(c: char) -> Box<dyn Fn(Vec<char>, usize) -> Option<char>> {
//   Box::new(|code: Vec<char>, pos: usize| {
//     let r = code[pos];
//     // let r = input.next_char();
//
//     if r == c {
//       // input.advance();
//
//       Some(r)
//     } else {
//       None
//     }
//   })
// }

// fn character2(c: char) -> Box<dyn Fn(Input) -> Option<char>> {
//   Box::new(|mut input: Input| {
//     let r = input.next_char();
//
//     if r == c {
//       input.advance();
//
//       Some(r)
//     } else {
//       None
//     }
//   })
// }

// fn character5(c: char) -> fn(Vec<char>, usize) -> Option<ParserResult<char>> {
//   fn (code: Vec<char>, pos: usize)  {
//     let r = code[pos];
//
//     if r == c {
//       Some(ParserResult {
//         value: r,
//         position: pos + 1,
//       })
//     } else {
//       None
//     }
//   }
// }

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

macro_rules! foo {
  // macro foo {
  ($name: ident) => {
    pub struct $name;

    impl $name {
      pub fn new() -> $name {
        $name
      }
    }
  };
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
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
  fn test_mac() {
    let p = character2!('3');

    let text: Vec<char> = String::from("3").chars().collect();

    let result = p(text, 0);

    // let s = Some(ParserResult {
    //   value: '3',
    //   position: 1,
    // });

    assert_eq!(result.unwrap().value, '3');
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

  #[test]
  fn test_word_parser_fail() {
    let p = word("omg2");
    let input = Input::new("omg");

    assert_eq!(p(input), None)
  }

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
