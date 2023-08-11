use crate::git::git_types::{HunkLine, HunkLineStatus, Patch, PatchType};
use crate::git::queries::hunks::load_hunks::{load_hunks, ReqHunkOptions};
use std::fmt::Display;

pub fn get_patch_as_html(options: &ReqHunkOptions) -> Result<String, String> {
  let (_, hunk_lines) = load_hunks(options)?;

  Ok(generate_lines(&hunk_lines, &options.patch))
}

// Paginate if too large?
fn generate_lines(hunk_lines: &Vec<HunkLine>, patch: &Patch) -> String {
  let mut margin = String::new();
  let mut lines = String::new();

  for hunk_line in hunk_lines {
    // margin.push_str(&format!(
    //   "<div>{}</div><div>{}</div>",
    //   s(hunk_line.old_num),
    //   s(hunk_line.new_num)
    // ));

    add_margin_line(patch, hunk_line, &mut margin);

    lines.push_str(&format!("<div>{}\n</div>", hunk_line.text));
  }

  format!("<div class=\"margin\">{}</div><div>{}</div>", margin, lines)
}

fn add_margin_line(patch: &Patch, line: &HunkLine, margin: &mut String) {
  match patch.patch_type {
    PatchType::A => {
      *margin += &format!("<div /><div>+{}</div>\n", s(line.new_num));
    }
    PatchType::D => {
      *margin += &format!("<div /><div>-{}</div>\n", s(line.old_num));
    }
    _ => {
      let HunkLine { status, .. } = line;

      match status {
        HunkLineStatus::Added => {
          *margin += &format!("<div>-{}</div>\n", s(line.old_num));
          *margin += &format!("<div>+{}</div>\n", s(line.new_num));
        }
        HunkLineStatus::Removed => {}
        HunkLineStatus::Unchanged => {}
        HunkLineStatus::HeaderStart => {}
        HunkLineStatus::HeaderEnd => {}
        HunkLineStatus::Skip => {}
      }

      //
    }
  }
}

fn s<T: Display>(s: Option<T>) -> String {
  s.map(|n| n.to_string()).unwrap_or(String::new())
}
