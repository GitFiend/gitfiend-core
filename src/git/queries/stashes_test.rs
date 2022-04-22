#[cfg(test)]
mod tests {
  use crate::git::queries::stashes::load_stashes;

  #[test]
  fn test_load_stashes() {
    let result = load_stashes(&"/home/toby/Repos/gitfiend-seed/git-fiend".to_string());

    println!("{:?}", result);
  }
}
