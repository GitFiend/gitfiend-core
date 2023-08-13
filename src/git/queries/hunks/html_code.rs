use crate::git::git_types::{Commit, Hunk, HunkLine, HunkLineStatus, Patch, PatchType};
use crate::git::queries::hunks::load_hunks::{load_hunks, ReqHunkOptions};
use crate::git::queries::syntax_colouring::{colour_to_hue, ColourLine, ThemeColour, COLOURING};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use syntect::highlighting::{Color, Style};
use ts_rs::TS;

#[macro_export]
macro_rules! f {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        res
    }}
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqPatchCodeOptions {
  pub repo_path: String,
  pub commit: Commit,
  pub patch: Patch,
  pub theme: ThemeColour,
}

pub fn get_patch_as_html(options: &ReqPatchCodeOptions) -> Result<String, String> {
  let ReqPatchCodeOptions {
    repo_path,
    commit,
    patch,
    theme,
  } = options;

  let (hunks, hunk_lines) = load_hunks(&ReqHunkOptions {
    repo_path: repo_path.clone(),
    commit: commit.clone(),
    patch: patch.clone(),
  })?;

  let mut colouring = COLOURING.write().map_err(|e| e.to_string())?;

  let mut c = colouring.get_colour_line(theme, &patch.get_file_extension());

  let lines = generate_lines(&hunk_lines, &options.patch, &hunks, &mut c);

  Ok(lines)
}

// Paginate if too large?
fn generate_lines(
  hunk_lines: &Vec<HunkLine>,
  patch: &Patch,
  hunks: &[Hunk],
  colour: &mut ColourLine,
) -> String {
  let mut margin = String::new();
  let mut lines = String::new();

  let margin_width = get_margin_width(hunk_lines);

  for hunk_line in hunk_lines {
    add_margin_line(patch, hunk_line, &mut margin, margin_width);
    add_line(&mut lines, hunk_line.get_hunk(hunks), hunk_line, colour);
  }

  // language=HTML
  f!(
    "<div class='margin'>{}</div><div class='code'>{}</div>",
    margin,
    lines
  )
}

fn add_line(lines: &mut String, hunk: Option<&Hunk>, line: &HunkLine, colour: &mut ColourLine) {
  use HunkLineStatus::*;
  let text = if let Ok(parts) = colour.colour(&f!("{}\n", line.text)) {
    build_line(parts, &colour.colouring.theme)
  } else {
    line.text.replace('\n', "")
  };

  match line.status {
    Added => {
      // language=HTML
      *lines += &f!("<div class='added'>{}</div>", text);
    }
    Removed => {
      // language=HTML
      *lines += &f!("<div class='removed'>{}</div>", text);
    }
    Unchanged => {
      lines.push_str(&f!("{}\n", text));
    }
    HeaderStart => {
      *lines += "\n";
    }
    HeaderEnd => {
      *lines += "\n";
    }
    Skip => {
      *lines += "\n";
    }
  }
}

fn build_line(parts: Vec<(Style, &str)>, theme: &ThemeColour) -> String {
  let mut line = String::new();

  for (style, text) in parts {
    line += &f!(
      // language=HTML
      "<span style='color: {};'>{}</span>",
      colour_to_style(style.foreground, theme),
      escape_html(&text.replace('\n', ""))
    );
  }

  line
}

fn colour_to_style(colour: Color, theme: &ThemeColour) -> String {
  if *theme == ThemeColour::Light {
    f!("hsl({}, 60%, 40%)", colour_to_hue(colour))
  } else {
    f!("hsl({}, 75%, 75%)", colour_to_hue(colour))
  }
}

fn add_margin_line(patch: &Patch, line: &HunkLine, margin: &mut String, margin_width: usize) {
  let empty_space = make_spaces(margin_width);

  match patch.patch_type {
    PatchType::A => {
      // language=HTML
      let num = f!(
        "<div class='added'>{:>margin_width$}</div>",
        s(line.new_num, "+")
      );
      *margin += &num;
    }
    PatchType::D => {
      // language=HTML
      *margin += &f!(
        "<div class='removed'>{:>margin_width$}</div>",
        s(line.old_num, "-"),
      );
    }
    _ => {
      let HunkLine { status, .. } = line;

      match status {
        HunkLineStatus::Added => {
          // language=HTML
          let num = f!(
            "<div class='added'>{} {:>margin_width$}</div>",
            empty_space,
            s(line.new_num, "+")
          );
          *margin += &num;
        }
        HunkLineStatus::Removed => {
          // language=HTML
          let num = f!(
            "<div class='removed'>{:>margin_width$} {}</div>",
            s(line.old_num, "-"),
            empty_space
          );
          *margin += &num;
        }
        HunkLineStatus::Unchanged => {
          *margin += &pad_left(s(line.old_num, ""), margin_width);
          *margin += &pad_left(s(line.new_num, ""), margin_width + 1);
          *margin += "\n";
        }
        HunkLineStatus::HeaderStart => {
          *margin += "\n";
        }
        HunkLineStatus::HeaderEnd => {
          *margin += "\n";
        }
        HunkLineStatus::Skip => {
          *margin += "\n";
        }
      }
    }
  }
}

fn s<T: Display>(s: Option<T>, prefix: &str) -> String {
  s.map(|n| prefix.to_string() + &n.to_string())
    .unwrap_or(String::new())
}

// Width of a side in chars
fn get_margin_width(lines: &[HunkLine]) -> usize {
  let mut max = 0;

  for line in lines {
    if let Some(num) = line.old_num {
      let num_chars = calc_num_chars(num);
      if num_chars > max {
        max = num_chars;
      }
    }
    if let Some(num) = line.new_num {
      let num_chars = calc_num_chars(num);
      if num_chars > max {
        max = num_chars;
      }
    }
  }

  max + 1
}

fn escape_html(line: &str) -> String {
  line
    .replace('&', "&amp;")
    .replace('<', "&lt;")
    .replace('>', "&gt;")
    .replace('\"', "&quot;")
    .replace('\'', "&#39;")
}

fn calc_num_chars(num: i32) -> usize {
  num.to_string().len()
}

fn pad_left(s: String, len: usize) -> String {
  f!("{:>len$}", s)
}

fn make_spaces(len: usize) -> String {
  f!("{:>width$}", "", width = len)
}

#[cfg(test)]
mod tests {
  use crate::git::queries::hunks::html_code::{calc_num_chars, make_spaces, pad_left};

  #[test]
  fn test_pad() {
    assert_eq!("   abc", &pad_left(String::from("abc"), 6));
    assert_eq!("   ", make_spaces(3));
  }

  #[test]
  fn test_num_chars() {
    assert_eq!(1, calc_num_chars(0));
    assert_eq!(2, calc_num_chars(10));
  }
}
