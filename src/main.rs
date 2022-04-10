mod git;
mod parser;

use crate::git::queries::commits::load_commits;
use parser::input::Input;
use std::time::Instant;

fn main() {
  let now = Instant::now();

  load_commits(5000);

  println!("load commits took {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
  #[test]
  fn read_file() {}
}
