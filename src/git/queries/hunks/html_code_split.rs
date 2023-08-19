use crate::f;
use crate::git::git_types::{Hunk, HunkLine, HunkLineStatus};
use crate::git::queries::hunks::html_code::{add_line, add_margin_line, div, get_margin_width};
use crate::git::queries::syntax_colouring::ColourLine;

pub fn generate_lines_split(
  hl_left: &Vec<HunkLine>,
  hl_right: &Vec<HunkLine>,
  hunks: &[Hunk],
  colour: &mut ColourLine,
) -> String {
  use HunkLineStatus::*;

  let left_margin_width = get_margin_width(hl_left);
  let right_margin_width = get_margin_width(hl_right);

  let mut left_margin = String::new();
  let mut left_lines = String::new();

  let mut right_margin = String::new();
  let mut right_lines = String::new();

  for line in hl_left {
    let hunk = line.get_hunk(hunks);

    match line.status {
      Removed => {
        add_margin_line(&mut left_margin, line, left_margin_width);
        add_line(&mut left_lines, hunk, line, colour);
      }
      _ => {
        add_margin_line(&mut left_margin, line, left_margin_width);
        add_line(&mut left_lines, hunk, line, colour);
      }
    }
  }

  for line in hl_right {
    let hunk = line.get_hunk(hunks);

    match line.status {
      Added => {
        add_margin_line(&mut right_margin, line, right_margin_width);
        add_line(&mut right_lines, hunk, line, colour);
      }
      _ => {
        add_margin_line(&mut right_margin, line, right_margin_width);
        add_line(&mut right_lines, hunk, line, colour);
      }
    }
  }

  let mut left = div("margin", &left_margin);
  left += &div("code", &left_lines);

  let mut right = div("margin", &right_margin);
  right += &div("code", &right_lines);

  f!("{}{}", div("codeLeft", &left), div("codeRight", &right))
}
