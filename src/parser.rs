pub struct ParserResult<T> {
  pub value: T,
  pub position: usize,
}

pub type Parser<T> = fn(Vec<char>, usize) -> ParserResult<T>;

fn omg() {
  let thing = vec![1, 2, 3];
  let t = thing.as_slice();
}
