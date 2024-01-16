use crate::f;
use crate::git::git_types::{Commit, Hunk, HunkLine, HunkLineStatus, Patch, PatchType};
use crate::git::queries::hunks::hunk_parsers::P_HUNKS;
use crate::git::queries::COMMIT_0_ID;
use crate::git::request_util::R;
use crate::git::run_git;
use crate::git::run_git::{GitOut, RunGitOptions};
use crate::parser::parse_all_err;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqHunkOptions {
  pub repo_path: String,
  pub commit: Commit,
  pub patch: Patch,
}

pub fn load_hunks(options: &ReqHunkOptions) -> R<(Vec<Hunk>, Vec<HunkLine>)> {
  let GitOut { stdout, .. } = run_git::run_git_err(RunGitOptions {
    repo_path: &options.repo_path,
    args: load_hunks_args(&options.commit, &options.patch),
  })?;

  let hunks = parse_all_err(P_HUNKS, &stdout)?;
  let hunk_lines = flatten_hunks(hunks.clone());

  Ok((hunks, hunk_lines))
}

type HunkLinesSplit = (Vec<Hunk>, Vec<HunkLine>, Vec<HunkLine>);

pub fn load_hunks_split(options: &ReqHunkOptions) -> R<HunkLinesSplit> {
  let GitOut { stdout, .. } = run_git::run_git_err(RunGitOptions {
    repo_path: &options.repo_path,
    args: load_hunks_args(&options.commit, &options.patch),
  })?;

  let hunks = parse_all_err(P_HUNKS, &stdout)?;
  let (hunk_lines_left, hunk_lines_right) = flatten_hunks_split(&hunks);

  Ok((hunks, hunk_lines_left, hunk_lines_right))
}

pub fn load_hunks_args(commit: &Commit, patch: &Patch) -> Vec<String> {
  let diff = "diff".to_string();
  let no_colour = f!("--no-color");
  let dashes = "--".to_string();

  let Commit {
    id,
    parent_ids,
    is_merge,
    ..
  } = commit;

  let mut args: Vec<String> = Vec::new();

  if *is_merge {
    // args.extend_from_slice(&[diff, format!("{}^@", id)]);
    args.extend_from_slice(&[
      diff,
      no_colour,
      f!("{}...{}", parent_ids[0], parent_ids[1]),
    ]);
  } else if !commit.parent_ids.is_empty() {
    args.extend_from_slice(&[diff, no_colour, f!("{}..{}", parent_ids[0], id)]);
  } else {
    args.extend_from_slice(&[diff, no_colour, f!("{}..{}", COMMIT_0_ID, id)]);
  }

  args.push(dashes);
  args.push(patch.old_file.clone());

  if patch.patch_type == PatchType::R {
    args.push(patch.new_file.clone());
  }

  args
}

fn flatten_hunks(hunks: Vec<Hunk>) -> Vec<HunkLine> {
  let mut lines: Vec<HunkLine> = Vec::new();

  if hunks.is_empty() {
    return lines;
  }

  for hunk in hunks {
    lines.push(HunkLine::header_from_type(
      HunkLineStatus::HeaderStart,
      hunk.index,
    ));
    lines.push(HunkLine::header_from_type(
      HunkLineStatus::HeaderEnd,
      hunk.index,
    ));

    for line in hunk.lines {
      lines.push(line);
    }
  }

  lines.push(HunkLine::header_from_type(HunkLineStatus::HeaderStart, -1));

  lines
}

pub fn flatten_hunks_split(hunks: &[Hunk]) -> (Vec<HunkLine>, Vec<HunkLine>) {
  let mut left = Vec::<HunkLine>::new();
  let mut right = Vec::<HunkLine>::new();

  if hunks.is_empty() {
    return (left, right);
  }

  use HunkLineStatus::*;

  for hunk in hunks {
    left.push(HunkLine::header_from_type(HeaderStart, hunk.index));
    left.push(HunkLine::header_from_type(HeaderEnd, hunk.index));
    right.push(HunkLine::header_from_type(HeaderStart, hunk.index));
    right.push(HunkLine::header_from_type(HeaderEnd, hunk.index));

    let mut left_count = 0;
    let mut right_count = 0;

    for line in &hunk.lines {
      if line.status == Removed {
        left.push(line.clone());
        left_count += 1;
      } else if line.status == Added {
        right.push(line.clone());
        right_count += 1;
      } else {
        if right_count > left_count {
          for _ in 0..(right_count - left_count) {
            left.push(HunkLine::skip_line(hunk.index));
            left_count += 1;
          }
        }
        if left_count > right_count {
          for _ in 0..(left_count - right_count) {
            right.push(HunkLine::skip_line(hunk.index));
            right_count += 1;
          }
        }

        left.push(line.clone());
        right.push(line.clone());
      }
    }

    if right_count > left_count {
      for _ in 0..(right_count - left_count) {
        left.push(HunkLine::skip_line(hunk.index));
        left_count += 1;
      }
    }
    if left_count > right_count {
      for _ in 0..(left_count - right_count) {
        right.push(HunkLine::skip_line(hunk.index));
        right_count += 1;
      }
    }
  }

  left.push(HunkLine::header_from_type(HeaderStart, -1));
  right.push(HunkLine::header_from_type(HeaderStart, -1));

  (clear_lines_if_empty(left), clear_lines_if_empty(right))
}

// If there are no meaningful lines, just return an empty vector.
fn clear_lines_if_empty(lines: Vec<HunkLine>) -> Vec<HunkLine> {
  use HunkLineStatus::*;

  if !lines
    .iter()
    .any(|line| line.status == Added || line.status == Removed)
  {
    let unchanged = lines
      .iter()
      .filter(|line| line.status == Unchanged)
      .collect::<Vec<_>>();

    if unchanged.is_empty()
      || unchanged.len() == 1 && unchanged[0].text.contains("No newline at end of file")
    {
      return Vec::new();
    }
  }
  lines
}
