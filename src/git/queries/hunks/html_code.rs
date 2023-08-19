use crate::f;
use crate::git::git_types::{Commit, Hunk, HunkLine, HunkLineStatus, Patch};
use crate::git::queries::hunks::html_code_split::generate_lines_split;
use crate::git::queries::hunks::load_hunks::{load_hunks, load_hunks_split, ReqHunkOptions};
use crate::git::queries::syntax_colouring::{colour_to_style, ColourLine, ThemeColour, COLOURING};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use syntect::highlighting::Style;
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqPatchCodeOptions {
  pub repo_path: String,
  pub commit: Commit,
  pub patch: Patch,
  pub theme: ThemeColour,
  pub split: bool,
}

pub fn get_patch_as_html(options: &ReqPatchCodeOptions) -> Result<String, String> {
  let ReqPatchCodeOptions {
    repo_path,
    commit,
    patch,
    theme,
    split,
  } = options;

  let mut colouring = COLOURING.write().map_err(|e| e.to_string())?;
  let mut c = colouring.get_colour_line(theme, &patch.get_file_extension());

  if *split {
    let (hunks, left, right) = load_hunks_split(&ReqHunkOptions {
      repo_path: repo_path.clone(),
      commit: commit.clone(),
      patch: patch.clone(),
    })?;

    let lines = generate_lines_split(&left, &right, &hunks, &mut c);

    Ok(lines)
  } else {
    let (hunks, hunk_lines) = load_hunks(&ReqHunkOptions {
      repo_path: repo_path.clone(),
      commit: commit.clone(),
      patch: patch.clone(),
    })?;

    let lines = generate_lines(&hunk_lines, &hunks, &mut c);

    Ok(lines)
  }
}

// Paginate if too large?
fn generate_lines(hunk_lines: &Vec<HunkLine>, hunks: &[Hunk], colour: &mut ColourLine) -> String {
  let mut margin = String::new();
  let mut lines = String::new();

  let margin_width = get_margin_width(hunk_lines);

  for hunk_line in hunk_lines {
    add_margin_line(&mut margin, hunk_line, margin_width);
    add_line(&mut lines, hunk_line.get_hunk(hunks), hunk_line, colour);
  }

  // language=HTML
  f!(
    "<div class='margin'>{}</div><div class='code'>{}</div>",
    margin,
    lines
  )
}

pub fn add_line(lines: &mut String, hunk: Option<&Hunk>, line: &HunkLine, colour: &mut ColourLine) {
  use HunkLineStatus::*;
  let text = if let Ok(parts) = colour.colour(&f!("{}\n", line.text)) {
    build_line(parts, &colour.colouring.theme)
  } else {
    line.text.replace('\n', "")
  };

  match line.status {
    Added => {
      *lines += &div("added", &text);
    }
    Removed => {
      *lines += &div("removed", &text);
    }
    Unchanged => {
      *lines += &div("none", &text);
    }
    HeaderStart => {
      *lines += &div("headerStart", "");
    }
    HeaderEnd => {
      if let Some(hunk) = hunk {
        *lines += &div("headerEnd", &gen_header_ranges(hunk));
      } else {
        *lines += &div("headerEnd", "");
      }
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

pub fn add_margin_line(margin: &mut String, line: &HunkLine, margin_width: usize) {
  let empty_space = make_spaces(margin_width);

  let HunkLine { status, .. } = line;

  match status {
    HunkLineStatus::Added => {
      *margin += &div(
        "added",
        &f!(" {} {:>margin_width$} ", empty_space, s(line.new_num, "+")),
      );
    }
    HunkLineStatus::Removed => {
      *margin += &div(
        "removed",
        &f!(" {:>margin_width$} {} ", s(line.old_num, "-"), empty_space),
      );
    }
    HunkLineStatus::Unchanged => {
      *margin += &pad_left(s(line.old_num, ""), margin_width + 1);
      *margin += &pad_left(s(line.new_num, ""), margin_width + 1);
      *margin += " \n";
    }
    HunkLineStatus::HeaderStart => {
      *margin += &div("headerStart", "");
    }
    HunkLineStatus::HeaderEnd => {
      *margin += &div("headerEnd", "");
    }
    HunkLineStatus::Skip => {
      *margin += " \n";
    }
  }
}

fn gen_header_ranges(hunk: &Hunk) -> String {
  let Hunk {
    old_line_range,
    new_line_range,
    ..
  } = hunk;

  f!(
    "@@ -{},{} +{},{} @@",
    old_line_range.start,
    old_line_range.length,
    new_line_range.start,
    new_line_range.length
  )
}

fn s<T: Display>(s: Option<T>, prefix: &str) -> String {
  s.map(|n| prefix.to_string() + &n.to_string())
    .unwrap_or(String::new())
}

// Width of a side in chars
pub fn get_margin_width(lines: &[HunkLine]) -> usize {
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

pub fn div(class_name: &str, content: &str) -> String {
  // language=HTML
  f!("<div class='{}'>{}</div>", class_name, content)
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
