use crate::git::git_types::HunkLine;
use crate::git::queries::hunks::load_hunks::{load_hunks, ReqHunkOptions};

pub fn get_patch_as_html(options: &ReqHunkOptions) -> Result<String, String> {
  let (_, hunk_lines) = load_hunks(options)?;

  Ok(generate_lines(&hunk_lines))
}

// Paginate if too large?
fn generate_lines(hunk_lines: &Vec<HunkLine>) -> String {
  let mut margin = String::new();
  let mut lines = String::new();

  for hunk_line in hunk_lines {
    margin.push_str(&format!(
      "<div>{}</div><div>{}</div>",
      to_str(hunk_line.old_num),
      to_str(hunk_line.new_num)
    ));

    lines.push_str(&format!("<div>{}\n</div>", hunk_line.text));

    // lines.push_str(&hunk_line.text);
    // lines.push('\n');
  }

  // format!(
  //   "<div class=\"pane\"><div>{}</div><div>{}</div></div>",
  //   margin, lines
  // )

  format!("<div class=\"margin\">{}</div><div>{}</div>", margin, lines)

  // lines
}

fn to_str(num: Option<i32>) -> String {
  num.map(|n| n.to_string()).unwrap_or("".to_string())
}
