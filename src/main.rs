mod input;
mod parser;
mod parser_types;

// use std::process::Command;
// use std::io::{self, Write};
// use std::borrow::BorrowMut;
// use std::collections::HashMap;
// use std::env;
// use std::fmt;
use std::fs;
// use std::ops::ShlAssign;
// use std::str;
use std::time::Instant;

fn main() {
  read_file();
  // commands_test();
  // let now = Instant::now();
  //
  // let git = "git";
  //
  // let output = Command::new(git)
  //   .arg("--help")
  //   .output()
  //   .expect("failed to execute process");
  //
  // let text = str::from_utf8(&output.stdout).unwrap();
  //
  // text.chars().for_each(|c| {
  //   // print!("{}", c);
  //   if c == '\n' {
  //     println!("OMG");
  //   }
  // });
  //
  // // let mut res: Vec<char> = text.chars().collect();
  //
  // println!("{}ms", now.elapsed().as_millis());
}

fn read_file() {
  let now = Instant::now();
  let contents = fs::read_to_string("./omg.txt").expect("Something went wrong reading the file");

  println!("read: {}ms", now.elapsed().as_millis());

  let now = Instant::now();
  let array: Vec<_> = contents.split("d").collect();

  println!("{}", array.len());

  println!("split: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
  use super::*;
  // use std::borrow::BorrowMut;
  // use std::ops::Deref;

  #[test]
  fn read_file() {
    let now = Instant::now();
    let contents = fs::read_to_string("./omg.txt").expect("Something went wrong reading the file");

    println!("read: {}ms", now.elapsed().as_millis());

    let now = Instant::now();
    let array: Vec<_> = contents.split("d").collect();

    println!("split: {}ms", now.elapsed().as_millis());
  }
}

// fn commands_test() {
//   let now = Instant::now();
//
//   for _x in 0..10 {
//     let git = "git";
//
//     Command::new(git)
//       .arg("--help")
//       .output()
//       .expect("failed to execute process");
//
//     Command::new(git)
//       .arg("--help")
//       .output()
//       .expect("failed to execute process");
//   }
//
//   println!("{}ms", now.elapsed().as_millis());
//   // println!("status: {}", output.status);
//   let mut omg = Observable::new(5);
//
//   println!("{}", omg);
//
//   omg <<= 6;
//
//   let mut hi = Vec::new();
//   hi.push(32);
//
//   // REACTION_STACK
//
//   println!("{}", omg);
//   println!("{}", omg.val);
// }

// struct Observable<T> {
//   val: T,
// }
//
// impl<T> Observable<T> {
//   pub fn new(val: T) -> Observable<T> {
//     Observable { val }
//   }
// }
//
// impl<T> ShlAssign<T> for Observable<T> {
//   fn shl_assign(&mut self, new_val: T) -> () {
//     self.val = new_val;
//   }
// }
//
// impl<T: std::fmt::Display> fmt::Display for Observable<T> {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "val: {}", self.val)
//   }
// }
