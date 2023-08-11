use crate::git::git_types::{HunkLine, HunkLineStatus, Patch, PatchType};
use crate::git::queries::hunks::load_hunks::{load_hunks, ReqHunkOptions};
use std::fmt::Display;

pub fn get_patch_as_html(options: &ReqHunkOptions) -> Result<String, String> {
  let (_, hunk_lines) = load_hunks(options)?;

  let lines = generate_lines(&hunk_lines, &options.patch);

  Ok(lines)
}

// Paginate if too large?
fn generate_lines(hunk_lines: &Vec<HunkLine>, patch: &Patch) -> String {
  let mut margin = String::new();
  let mut lines = String::new();

  let margin_width = get_margin_width(hunk_lines);

  for hunk_line in hunk_lines {
    add_margin_line(patch, hunk_line, &mut margin, margin_width);

    lines.push_str(&format!("{}\n", hunk_line.text));
  }

  format!(
    "<div class=\"margin\">{}</div><div class=\"code\">{}</div>",
    margin, lines
  )
}

fn add_margin_line(patch: &Patch, line: &HunkLine, margin: &mut String, margin_width: usize) {
  let empty_space = make_spaces(margin_width);

  match patch.patch_type {
    PatchType::A => {
      let num = format!(
        "<div>{} {:>margin_width$}</div>\n",
        empty_space,
        s(line.new_num, "+")
      );
      *margin += &num;
    }
    PatchType::D => {
      *margin += &format!(
        "<div>{:>margin_width$} {}</div>\n",
        s(line.old_num, "-"),
        empty_space
      );
    }
    _ => {
      let HunkLine { status, .. } = line;

      match status {
        HunkLineStatus::Added => {
          let num = format!(
            "<div>{} {:>margin_width$}</div>\n",
            empty_space,
            s(line.new_num, "+")
          );
          *margin += &num;
        }
        HunkLineStatus::Removed => {
          let num = format!(
            "<div>{:>margin_width$} {}</div>\n",
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
        HunkLineStatus::HeaderStart => {}
        HunkLineStatus::HeaderEnd => {}
        HunkLineStatus::Skip => {}
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

fn calc_num_chars(num: i32) -> usize {
  num.to_string().len()
}

fn pad_left(s: String, len: usize) -> String {
  format!("{:>len$}", s)
}

fn pad_right(s: String, len: usize) -> String {
  format!("{:width$}", s, width = len)
}

fn make_spaces(len: usize) -> String {
  format!("{:>width$}", "", width = len)
}

#[cfg(test)]
mod tests {
  use crate::git::queries::hunks::html_code::{calc_num_chars, make_spaces, pad_left, pad_right};

  #[test]
  fn test_pad() {
    assert_eq!("   abc", &pad_left(String::from("abc"), 6));
    assert_eq!("abc   ", &pad_right(String::from("abc"), 6));
    assert_eq!("   ", make_spaces(3));
  }

  #[test]
  fn test_num_chars() {
    // assert_eq!(1, calc_num_chars(0));
    assert_eq!(2, calc_num_chars(10));
  }
}
