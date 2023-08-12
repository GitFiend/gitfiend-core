use crate::glo;
use crate::util::global::Glo;
use serde::{Deserialize, Serialize};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use ts_rs::TS;

pub static COLOURING: Glo<Colouring> = glo!(Colouring::new());

pub struct Colouring {
  pub syntax_set: SyntaxSet,
  pub theme_set: ThemeSet,
  pub theme: ThemeColour,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum ThemeColour {
  Light,
  Dark,
}

impl Colouring {
  pub fn new() -> Self {
    Self {
      syntax_set: SyntaxSet::load_defaults_newlines(),
      theme_set: ThemeSet::load_defaults(),
      theme: ThemeColour::Light,
    }
  }

  pub fn set_theme(&mut self, theme: &ThemeColour) {
    self.theme = *theme;
  }

  pub fn get_colour_line(&mut self, theme: &ThemeColour, file_extension: &str) -> ColourLine {
    self.set_theme(theme);
    let h = self.get_highlighter(file_extension);

    ColourLine {
      colouring: self,
      highlight: h,
    }
  }

  // HighlightLines isn't thread safe, so can't be stored in a global.
  pub fn get_highlighter(&self, file_extension: &str) -> Option<HighlightLines> {
    let syntax = self.syntax_set.find_syntax_by_extension(file_extension)?;

    let theme_str = if self.theme == ThemeColour::Dark {
      "base16-ocean.dark"
    } else {
      "base16-ocean.light"
    };

    Some(HighlightLines::new(
      syntax,
      &self.theme_set.themes[theme_str],
    ))
  }
}

pub struct ColourLine<'a> {
  colouring: &'a Colouring,
  highlight: Option<HighlightLines<'a>>,
}

impl<'a> ColourLine<'a> {
  pub fn colour<'b>(&mut self, line: &'b str) -> Result<Vec<(Style, &'b str)>, String> {
    if let Some(ref mut h) = self.highlight {
      return h
        .highlight_line(line, &self.colouring.syntax_set)
        .map_err(|e| e.to_string());
    }

    Err(String::from("Highlighter isn't loaded for this file"))
  }
}
