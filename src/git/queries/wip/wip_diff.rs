use std::ascii::escape_default;
use std::fs::{read, read_to_string};
use std::ops::Add;
use std::path::Path;
use std::str::from_utf8;

use serde::Deserialize;
use similar::{ChangeTag, TextDiff};
use ts_rs::TS;

use crate::git::git_types::{Commit, Hunk, HunkLine, HunkLineStatus, WipPatch, WipPatchType};
use crate::git::queries::hunks::load_hunks::flatten_hunks_split;
use crate::git::queries::syntax_colouring::COLOURING;
use crate::git::queries::wip::create_hunks::convert_lines_to_hunks;
use crate::git::run_git;
use crate::git::run_git::{run_git_bstr, RunGitOptions};
use crate::parser::standard_parsers::{LINE_END, WS_STR};
use crate::parser::{parse_all, Parser};
use crate::server::request_util::{to_r, R};
use crate::{and, or, rep_parser_sep, until_parser_keep_happy};
use bstr::{BStr, BString, ByteSlice};

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqWipHunksOptions {
  pub repo_path: String,
  pub patch: WipPatch,
  pub head_commit: Option<Commit>,
}

pub fn load_wip_hunks(options: &ReqWipHunksOptions) -> R<(Vec<Hunk>, u32)> {
  let lines = load_wip_hunk_lines(options)?;

  // try_colour(&lines, &options.patch);

  Ok(convert_lines_to_hunks(lines))
}

pub fn load_wip_hunks_split(options: &ReqWipHunksOptions) -> R<(Vec<HunkLine>, Vec<HunkLine>)> {
  let (hunks, _) = load_wip_hunks(options)?;

  Ok(flatten_hunks_split(hunks))
}

pub fn load_wip_hunks_coloured(options: &ReqWipHunksOptions) -> R<()> {
  let lines = load_wip_hunk_lines(options)?;
  // let hunks = convert_lines_to_hunks(lines);

  try_colour(&lines, &options.patch);

  Ok(())
}

fn try_colour(lines: &[HunkLine], patch: &WipPatch) {
  if let Ok(colouring) = COLOURING.read() {
    let syntax_set = &colouring.syntax_set;
    let theme_set = &colouring.theme_set;

    let file_extension = Path::new(&patch.new_file)
      .extension()
      .unwrap()
      .to_str()
      .unwrap();

    println!("extension: {}", file_extension);

    if let Some(syntax) = syntax_set.find_syntax_by_extension(file_extension) {
      let mut highlighter =
        syntect::easy::HighlightLines::new(syntax, &theme_set.themes["base16-ocean.dark"]);

      for line in lines {
        if let Ok(highlighted_line) = highlighter.highlight_line(&line.text, syntax_set) {
          for (style, text) in highlighted_line {
            print!("{:?}", style.foreground);
            print!("{}", text);
            print!("{:?}", style.foreground);
          }
          // println!("{:?}", highlighted_line);
        }
      }
    }
  }
}

pub fn load_wip_hunk_lines(options: &ReqWipHunksOptions) -> R<Vec<HunkLine>> {
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
    return Ok(Vec::new());
  }

  let new_file_info = load_file_2(repo_path, new_file)?;

  if *patch_type == WipPatchType::A || head_commit.is_none() {
    return Ok(calc_hunk_line_from_text(b"", &new_file_info.content));
  }

  if let Some(commit) = head_commit {
    let mut old_text = load_unchanged_file2(repo_path, patch, commit)?;

    if *patch_type == WipPatchType::D {
      return Ok(calc_hunk_line_from_text(&old_text, b""));
    }

    // TODO
    // old_text = switch_to_line_ending(old_text, new_file_info.line_ending);

    return Ok(calc_hunk_line_from_text(&old_text, &new_file_info.content));
  }

  Ok(Vec::new())
}

struct FileInfo {
  text: String,
  line_ending: String,
}

fn load_file(repo_path: &str, file_path: &str) -> R<FileInfo> {
  println!("{:?}", load_file_2(repo_path, file_path)?);

  let path = Path::new(repo_path).join(file_path);
  let text = read_to_string(path).map_err(|e| e.to_string())?;
  let line_ending = detect_new_line(&text);

  if !text.ends_with(&line_ending) {
    return Ok(FileInfo {
      text: text.add(&line_ending),
      line_ending,
    });
  }

  Ok(FileInfo { text, line_ending })
}

#[derive(Debug)]
struct FileInfo2 {
  content: BString,
  line_ending: &'static BStr,
}

fn show(bs: &[u8]) -> String {
  let mut visible = String::new();
  for &b in bs {
    let part: Vec<u8> = escape_default(b).collect();
    visible.push_str(from_utf8(&part).unwrap());
  }
  visible
}

fn load_file_2(repo_path: &str, file_path: &str) -> R<FileInfo2> {
  let path = Path::new(repo_path).join(file_path);
  let mut content = read(path).map_err(to_r)?;
  let line_ending = detect_new_line_2(&content);

  if !content.ends_with(line_ending) {
    content.extend(line_ending);
  }

  println!("{:?}", show(&content));

  Ok(FileInfo2 {
    content: BString::from(content),
    line_ending: line_ending.into(),
  })
}

fn detect_new_line_2(text: &[u8]) -> &'static [u8] {
  let mut n = 0;
  let mut r = 0;

  for c in text.chars() {
    match c {
      '\n' => n += 1,
      '\r' => r += 1,
      _ => {}
    }
  }

  if r > (n / 2) {
    b"\r\n"
  } else {
    b"\n"
  }
}

fn detect_new_line(text: &str) -> String {
  let mut n = 0;
  let mut r = 0;

  for c in text.chars() {
    match c {
      '\n' => n += 1,
      '\r' => r += 1,
      _ => {}
    }
  }

  String::from(if r > (n / 2) { "\r\n" } else { "\n" })
}

const LINE_PARSER: Parser<(String, &str)> =
  and!(until_parser_keep_happy!(LINE_END), or!(LINE_END, WS_STR));

const LINES_PARSER: Parser<Vec<String>> =
  rep_parser_sep!(until_parser_keep_happy!(LINE_END), or!(LINE_END, WS_STR));

/// Unifies line ending in text to be the provided. Also appends line ending to end.
// fn switch_to_line_ending(text: BString, line_ending: &BStr) -> Vec<u8> {
//   if let Some(lines) = parse_all(LINES_PARSER, &text) {
//     let joined_text = lines.join(line_ending);
//
//     return joined_text.add(line_ending);
//   }
//
//   text
// }

pub fn calc_hunk_line_from_text(a: &[u8], b: &[u8]) -> Vec<HunkLine> {
  let diff = TextDiff::from_lines(a, b);

  let mut lines = Vec::<HunkLine>::new();

  let mut running_old_num = 0;
  let mut running_new_num = 0;

  for change in diff.iter_all_changes() {
    let mut old_num: Option<i32> = None;
    let mut new_num: Option<i32> = None;

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
    let parts = parse_all(LINE_PARSER, &line_text).unwrap_or((String::from(""), ""));

    lines.push(HunkLine {
      text: parts.0,
      line_ending: parts.1.to_string(),
      status: get_status_from_change_tag(&change.tag()),
      hunk_index: -1,
      index: lines.len() as u32,
      old_num,
      new_num,
    });
  }

  lines
}

// pub fn calc_hunk_line_from_text(a: &str, b: &str) -> Vec<HunkLine> {
//   let diff = TextDiff::from_lines(a, b);
//
//   let mut lines = Vec::<HunkLine>::new();
//
//   let mut running_old_num = 0;
//   let mut running_new_num = 0;
//
//   for change in diff.iter_all_changes() {
//     let mut old_num: Option<i32> = None;
//     let mut new_num: Option<i32> = None;
//
//     match change.tag() {
//       ChangeTag::Insert => {
//         running_new_num += 1;
//         new_num = Some(running_new_num);
//       }
//       ChangeTag::Delete => {
//         running_old_num += 1;
//         old_num = Some(running_old_num);
//       }
//       ChangeTag::Equal => {
//         running_old_num += 1;
//         running_new_num += 1;
//         old_num = Some(running_old_num);
//         new_num = Some(running_new_num);
//       }
//     }
//
//     let line_text = change.to_string();
//     let parts = parse_all(LINE_PARSER, &line_text).unwrap_or((String::from(""), ""));
//
//     lines.push(HunkLine {
//       text: parts.0,
//       line_ending: parts.1.to_string(),
//       status: get_status_from_change_tag(&change.tag()),
//       hunk_index: -1,
//       index: lines.len() as u32,
//       old_num,
//       new_num,
//     });
//   }
//
//   lines
// }
//
fn get_status_from_change_tag(tag: &ChangeTag) -> HunkLineStatus {
  match tag {
    ChangeTag::Insert => HunkLineStatus::Added,
    ChangeTag::Delete => HunkLineStatus::Removed,
    ChangeTag::Equal => HunkLineStatus::Unchanged,
  }
}

fn load_unchanged_file2(repo_path: &String, patch: &WipPatch, head_commit: &Commit) -> R<BString> {
  let result = run_git_bstr(RunGitOptions {
    repo_path,
    args: ["show", &format!("{}:{}", head_commit.id, &patch.old_file)],
  })?;

  Ok(result.stdout)
}

fn load_unchanged_file(
  repo_path: &String,
  patch: &WipPatch,
  head_commit: &Commit,
) -> Option<String> {
  run_git::run_git(RunGitOptions {
    repo_path,
    args: ["show", &format!("{}:{}", head_commit.id, &patch.old_file)],
  })
}

#[cfg(test)]
mod tests {
  use crate::git::queries::wip::wip_diff::{
    calc_hunk_line_from_text, detect_new_line, LINES_PARSER,
  };
  use crate::parser::parse_all;

  #[test]
  fn test_calc_hunk_line_from_text() {
    let text = b"import {ThemeName} from '../views/theme/theming'

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

    let lines = calc_hunk_line_from_text(b"", text);

    assert_eq!(lines.len(), 30);
  }

  #[test]
  fn test_detect_new_line() {
    assert_eq!(detect_new_line("\r\na"), "\r\n");
    assert_eq!(detect_new_line("\na\nb\nc"), "\n");

    assert_eq!(detect_new_line("\r\na\r\nb\n"), "\r\n");
  }

  #[test]
  fn test_many_line_parser() {
    let res = parse_all(LINES_PARSER, "asdf\nasdf");

    assert!(res.is_some());
    assert_eq!(res.unwrap().len(), 2);

    let res = parse_all(LINES_PARSER, "asdf\nasdf\n");

    assert!(res.is_some());

    let res = parse_all(LINES_PARSER, "asdfr\nasdfr\n");

    assert!(res.is_some());

    let res = parse_all(LINES_PARSER, "asdfr");

    assert!(res.is_some());
  }
}
