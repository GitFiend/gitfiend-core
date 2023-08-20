use crate::f;
use crate::git::git_types::{Hunk, HunkLine, HunkLineStatus};
use crate::git::queries::hunks::html_code::{
  add_line, div, get_margin_width, make_spaces, pad_left, s,
};
use crate::git::queries::syntax_colouring::ColourLine;

pub fn generate_lines_split(
  hl_left: &[HunkLine],
  hl_right: &[HunkLine],
  hunks: &[Hunk],
  colour: &mut ColourLine,
) -> String {
  println!("{} {}", hl_left.len(), hl_right.len());
  let char_width = get_longest_line(hl_left, hl_right);

  let (left_margin, left_lines) = gen_side(hl_left, hunks, colour, Side::Left, char_width);
  let (right_margin, right_lines) = gen_side(hl_right, hunks, colour, Side::Right, char_width);

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

fn gen_side(
  hunk_lines: &[HunkLine],
  hunks: &[Hunk],
  colour: &mut ColourLine,
  side: Side,
  char_width: usize,
) -> (String, String) {
  use HunkLineStatus::*;

  let margin_width = get_margin_width(hunk_lines);

  let mut margin = String::new();
  let mut lines = String::new();

  for line in hunk_lines {
    let hunk = line.get_hunk(hunks);

    match &line.status {
      Added => {
        add_margin_line(&mut margin, line, margin_width, side);
        add_line(&mut lines, hunk, line, colour);
      }
      Removed => {
        add_margin_line(&mut margin, line, margin_width, side);
        add_line(&mut lines, hunk, line, colour);
      }
      _ => {
        add_margin_line(&mut margin, line, margin_width, side);
        add_line(&mut lines, hunk, line, colour);
      }
    }
  }

  lines += &div("fillWidth", &make_spaces(char_width));
  // lines += &make_spaces(char_width);

  (margin, lines)
}

#[derive(Clone, Copy, PartialEq)]
enum Side {
  Left,
  Right,
}

fn add_margin_line(margin: &mut String, line: &HunkLine, margin_width: usize, side: Side) {
  use HunkLineStatus::*;

  let HunkLine { status, .. } = line;

  match status {
    Added => {
      *margin += &div("added", &f!(" {:>margin_width$} ", s(line.new_num, "+")));
    }
    Removed => {
      *margin += &div("removed", &f!(" {:>margin_width$} ", s(line.old_num, "-")));
    }
    Unchanged => {
      if side == Side::Left {
        *margin += &pad_left(s(line.old_num, ""), margin_width + 1);
      } else {
        *margin += &pad_left(s(line.new_num, ""), margin_width + 1);
      }
      *margin += " \n";
    }
    HeaderStart => {
      *margin += &div("headerStart", "");
    }
    HeaderEnd => {
      *margin += &div("headerEnd", "");
    }
    Skip => {
      *margin += &div("empty", "");
    }
  }
}

fn get_longest_line(left: &[HunkLine], right: &[HunkLine]) -> usize {
  left
    .iter()
    .chain(right.iter())
    .map(|hl| hl.text.len())
    .max_by(|a, b| a.cmp(b))
    .unwrap_or(0)
}
