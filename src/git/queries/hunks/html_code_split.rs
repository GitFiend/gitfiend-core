use crate::f;
use crate::git::git_types::{Hunk, HunkLine, HunkLineStatus};
use crate::git::queries::hunks::html_code::{add_line, add_margin_line, div, get_margin_width};
use crate::git::queries::syntax_colouring::ColourLine;

pub fn generate_lines_split(
  hl_left: &[HunkLine],
  hl_right: &[HunkLine],
  hunks: &[Hunk],
  colour: &mut ColourLine,
) -> String {
  println!("{} {}", hl_left.len(), hl_right.len());

  let (left_margin, left_lines) = gen_side(hl_left, hunks, colour);
  let (right_margin, right_lines) = gen_side(hl_right, hunks, colour);

  let mut left = div("margin", &left_margin);
  left += &div("code", &left_lines);

  let mut right = div("margin", &right_margin);
  right += &div("code", &right_lines);

  if hl_left.is_empty() {
    return div("codeRight", &right);
  } else if hl_right.is_empty() {
    return div("codeLeft", &left);
  }

  f!("{}{}", div("codeLeft", &left), div("codeRight", &right))
}

fn gen_side(hunk_lines: &[HunkLine], hunks: &[Hunk], colour: &mut ColourLine) -> (String, String) {
  use HunkLineStatus::*;

  let margin_width = get_margin_width(hunk_lines);

  let mut margin = String::new();
  let mut lines = String::new();

  for line in hunk_lines {
    let hunk = line.get_hunk(hunks);

    match &line.status {
      Added => {
        add_margin_line(&mut margin, line, margin_width);
        add_line(&mut lines, hunk, line, colour);
      }
      Removed => {
        add_margin_line(&mut margin, line, margin_width);
        add_line(&mut lines, hunk, line, colour);
      }
      _ => {
        add_margin_line(&mut margin, line, margin_width);
        add_line(&mut lines, hunk, line, colour);
      }
    }
  }

  (margin, lines)
}
