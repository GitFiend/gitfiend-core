use crate::util::global::Glo;
use crate::{f, glo};
use serde::{Deserialize, Serialize};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Style, ThemeSet};
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

    println!(
      "file_ext: {}, highlighter loaded: {}",
      file_extension,
      h.is_some()
    );

    ColourLine {
      colouring: self,
      highlight: h,
      extension: file_extension.to_string(),
    }
  }

  // HighlightLines isn't thread safe, so can't be stored in a global.
  pub fn get_highlighter(&self, file_extension: &str) -> Option<HighlightLines> {
    let ext = match file_extension {
      "ts" => "js",
      "tsx" => "js",
      _ => file_extension,
    };

    let syntax = self.syntax_set.find_syntax_by_extension(ext)?;

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

  pub fn _get_supported_things(self) -> (Vec<String>, Vec<String>) {
    let themes = self.theme_set.themes.keys().cloned().collect();

    let extensions = self
      .syntax_set
      .syntaxes()
      .iter()
      .flat_map(|s| s.file_extensions.clone())
      .collect();

    (themes, extensions)
  }
}

pub struct ColourLine<'a> {
  pub colouring: &'a Colouring,
  pub highlight: Option<HighlightLines<'a>>,
  pub extension: String,
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

  // We are trying to highlight fragments of code the have missing context. Fake up the context.
  pub fn start_fragment(&mut self) {
    let _ = self.colour("{\n");
  }

  pub fn end_fragment(&mut self) {
    let _ = self.colour("}\n");
  }
}

// pub fn scale_colour(colour: Color, theme: &ThemeColour) -> Color {
//   let Color { r, g, b, .. } = colour;
//
//   let sum: u32 = r as u32 + g as u32 + b as u32;
//
//   let max = r.max(g).max(b);
//   let min = r.min(g).min(b);
//
//   println!("sum: {}, max: {}", sum, r.max(g).max(b));
//
//   if theme == &ThemeColour::Light {
//     if min > 0 {
//       let scale = 255.0 / min as f32;
//       println!("diff: {}", min);
//
//       return Color {
//         r: ((r - min) as f32 * scale) as u8,
//         g: ((g - min) as f32 * scale) as u8,
//         b: ((b - min) as f32 * scale) as u8,
//         a: 255,
//       };
//     }
//   } else if max < 255 {
//     let scale = 255.0 / max as f32;
//     println!("scale: {}", scale);
//
//     return Color {
//       r: (r as f32 * scale).round() as u8,
//       g: (g as f32 * scale).round() as u8,
//       b: (b as f32 * scale).round() as u8,
//       a: 255,
//     };
//   }
//
//   colour
// }

pub fn colour_to_style(colour: Color, theme: &ThemeColour) -> String {
  if *theme == ThemeColour::Light {
    f!("hsl({}, 100%, 30%)", colour_to_hue(colour))
  } else {
    f!("hsl({}, 100%, 80%)", colour_to_hue(colour))
  }
}

pub fn colour_to_hue(colour: Color) -> f32 {
  let Color { r, g, b, .. } = colour;

  let r = r as f32;
  let g = g as f32;
  let b = b as f32;

  let c_min = r.min(g).min(b);
  let c_max = r.max(g).max(b);
  let delta = c_max - c_min;

  let mut hue = if delta == 0. {
    0.
  } else if c_max == r {
    60. * (((g - b) / delta) % 6.)
  } else if c_max == g {
    60. * (((b - r) / delta) + 2.)
  } else {
    60. * (((r - g) / delta) + 4.)
  }
  .round();

  if hue < 0. {
    hue += 360.;
  }

  hue
}
