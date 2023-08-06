use crate::git::git_types::HunkLine;
use crate::git::queries::hunks::load_hunks::{load_hunks, ReqHunkOptions};

pub fn get_patch_as_html(options: &ReqHunkOptions) -> Result<String, String> {
  let (_, hunk_lines) = load_hunks(options)?;

  Ok(generate_lines(&hunk_lines))
}

fn generate_lines(hunk_lines: &Vec<HunkLine>) -> String {
  let mut lines = String::new();

  for hunk_line in hunk_lines {
    lines.push_str(&hunk_line.text);
    lines.push('\n');
  }

  lines
}
