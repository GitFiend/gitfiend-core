use crate::git::git_types::{Commit, HunkLine, HunkLineStatus, WipPatch, WipPatchType};
use crate::git::store::RwStore;
use crate::git::{run_git, RunGitOptions};
use crate::many;
use crate::parser::standard_parsers::UNTIL_LINE_END_KEEP;
use crate::parser::{parse_all, Parser};
use regex::Regex;
use serde::{Deserialize, Serialize};
use similar::{ChangeTag, TextDiff};
use std::fs::read_to_string;
use std::path::Path;
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqWipHunksOptions {
  pub repo_path: String,
  pub patch: WipPatch,
  pub head_commit: Option<Commit>,
}

pub fn load_wip_hunks(options: &ReqWipHunksOptions, _: RwStore) -> Option<Vec<HunkLine>> {
  let _lines = load_wip_hunk_lines(options);

  _lines
}

pub fn load_wip_hunk_lines(options: &ReqWipHunksOptions) -> Option<Vec<HunkLine>> {
  let ReqWipHunksOptions {
    patch,
    repo_path,
    head_commit,
  } = &options;
  let WipPatch {
    new_file,
    is_image,
    patch_type,
    ..
  } = patch;

  if *is_image {
    return None;
  }

  let new_file_info = load_file(repo_path, new_file);

  if *patch_type == WipPatchType::A || head_commit.is_none() {
    return Some(calc_hunk_line_from_text("", &new_file_info?.text));
  }

  if let Some(commit) = head_commit {
    let mut old_text = load_unchanged_file(repo_path, patch, commit).unwrap_or(String::from(""));

    if *patch_type == WipPatchType::D {
      return Some(calc_hunk_line_from_text(&old_text, ""));
    }

    if let Some(new_file_info) = new_file_info {
      old_text = switch_to_line_ending(old_text, &new_file_info.line_ending);

      return Some(calc_hunk_line_from_text(&old_text, &new_file_info.text));
    }
  }

  None
}

struct FileInfo {
  text: String,
  line_ending: String,
}

fn load_file(repo_path: &String, file_path: &String) -> Option<FileInfo> {
  match read_to_string(Path::new(repo_path).join(file_path)) {
    Ok(text) => {
      let line_ending = detect_new_line(&text);

      return Some(FileInfo { text, line_ending });
    }
    Err(e) => {
      println!("{}", e)
    }
  }

  None
}

fn detect_new_line(text: &String) -> String {
  let re = Regex::new(r"\r?\n").unwrap();

  let mut crlf = 0;
  let mut lf = 0;

  for nl in re.find_iter(text) {
    if nl.as_str() == "\r\n" {
      crlf += 1;
    } else {
      lf += 1;
    }
  }

  String::from(if crlf > lf { "\r\n" } else { "\n" })
}

// TODO: This fails if it doesn't find a line ending.
const LINE_PARSER: Parser<Vec<(String, &str)>> = many!(UNTIL_LINE_END_KEEP);

fn switch_to_line_ending(text: String, line_ending: &str) -> String {
  if let Some(result) = parse_all(LINE_PARSER, &text) {
    let lines: Vec<String> = result.into_iter().map(|line| line.0).collect();

    return lines.join(line_ending);
  }

  text
}

fn calc_hunk_line_from_text(a: &str, b: &str) -> Vec<HunkLine> {
  let line_ending_re = Regex::new(r"\r?\n").unwrap();

  let diff = TextDiff::from_lines(a, b);

  let mut lines = Vec::<HunkLine>::new();

  let mut running_old_num = 0;
  let mut running_new_num = 0;

  for change in diff.iter_all_changes() {
    let mut old_num: Option<u32> = None;
    let mut new_num: Option<u32> = None;

    match change.tag() {
      ChangeTag::Insert => {
        running_new_num += 1;
        new_num = Some(running_new_num);
      }
      ChangeTag::Delete => {
        running_old_num += 1;
        old_num = Some(running_old_num);
      }
      ChangeTag::Equal => {
        running_old_num += 1;
        running_new_num += 1;
        old_num = Some(running_old_num);
        new_num = Some(running_new_num);
      }
    }

    let line_text = change.to_string();
    // TODO: This doesn't keep the line ending.
    let parts: Vec<&str> = line_ending_re.split(&line_text).collect();

    let parts2 = parse_all(UNTIL_LINE_END_KEEP, &line_text);

    lines.push(HunkLine {
      text: parts[0].to_string(),
      line_ending: parts.get(1).unwrap_or(&"").to_string(),
      status: get_status_from_change_tag(&change.tag()),
      hunk_index: -1,
      index: lines.len() as u32,
      old_num,
      new_num,
    });
  }

  lines
}

fn get_status_from_change_tag(tag: &ChangeTag) -> HunkLineStatus {
  match tag {
    ChangeTag::Insert => HunkLineStatus::Added,
    ChangeTag::Delete => HunkLineStatus::Removed,
    ChangeTag::Equal => HunkLineStatus::Unchanged,
  }
}

fn load_unchanged_file(
  repo_path: &String,
  patch: &WipPatch,
  head_commit: &Commit,
) -> Option<String> {
  run_git(RunGitOptions {
    repo_path,
    args: ["show", &format!("{}:{}", head_commit.id, &patch.old_file)],
  })
}

#[cfg(test)]
mod tests {
  use crate::git::queries::wip::wip_diff::calc_hunk_line_from_text;

  #[test]
  fn test_calc_hunk_line_from_text() {
    let text = "import {ThemeName} from '../views/theme/theming'

export const maxNumberOfCommits = 1000
export const maxNumberOfCommits = 100

export const bgSize = 500

export const font = `13px -apple-system,BlinkMacSystemFont,Segoe UI,Helvetica,Arial,sans-serif,Apple Color Emoji,Segoe UI Emoji`

export const monoFont = `13px 'Menlo', 'Ubuntu Mono', 'Consolas', monospace`

export const defaultTheme: ThemeName = 'dark'

export const defaultAnimationTime: AnimationTime = {
  short: 150,
  medium: 300,
  long: 400,
}

export const animationTimeDisabled: AnimationTime = {
  short: 0,
  medium: 0,
  long: 0,
}

export interface AnimationTime {
  short: number
  medium: number
  long: number
}
";

    let lines = calc_hunk_line_from_text("", text);

    assert_eq!(lines.len(), 30);
  }
}
