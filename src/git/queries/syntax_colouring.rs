use crate::glo;
use crate::util::global::Glo;
use std::sync::RwLock;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub static COLOURING: Glo<Colouring> = glo!(Colouring::new());

pub struct Colouring {
  pub syntax_set: SyntaxSet,
  pub theme_set: ThemeSet,
}

impl Colouring {
  pub fn new() -> Self {
    Self {
      syntax_set: SyntaxSet::load_defaults_newlines(),
      theme_set: ThemeSet::load_defaults(),
    }
  }
}
