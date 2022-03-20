//

pub struct Input {
  code: Vec<char>,
  pub position: usize,
  attempted_position: usize,
}

impl Input {
  pub fn new(code: &str) -> Input {
    Input {
      code: code.chars().collect(),
      position: 0,
      attempted_position: 0,
    }
  }

  pub fn advance(&mut self) {
    self.set_position(self.position + 1);
  }

  pub fn advance_by(&mut self, num: usize) {
    self.set_position(self.position + num);
  }

  pub fn next_char(&mut self) -> char {
    self.code[self.position]
  }

  pub fn set_position(&mut self, pos: usize) {
    if pos > self.attempted_position {
      self.attempted_position = pos;
    }
    self.position = pos;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_advance() {
    let mut input = Input::new("wowowoowowow");

    assert_eq!(input.next_char(), 'w');

    input.advance();

    assert_eq!(input.position, 1);
    assert_eq!(input.next_char(), 'o');
  }
}
