use crate::git::git_types::{Commit, Hunk, HunkLine, HunkLineStatus, Patch, PatchType};
use crate::git::queries::hunks::hunk_parsers::P_HUNKS;
use crate::git::queries::COMMIT_0_ID;
use crate::git::run_git;
use crate::git::run_git::RunGitOptions;
use crate::parser::parse_all;
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

pub fn load_hunks(options: &ReqHunkOptions) -> Option<(Vec<Hunk>, Vec<HunkLine>)> {
  println!("{:?}", load_hunks_args(&options.commit, &options.patch));

  let out = run_git::run_git(RunGitOptions {
    repo_path: &options.repo_path,
    args: load_hunks_args(&options.commit, &options.patch),
  })?;

  let hunks = parse_all(P_HUNKS, &out)?;
  let hunk_lines = flatten_hunks(hunks.clone());

  Some((hunks, hunk_lines))
}

pub fn load_hunks_args(commit: &Commit, patch: &Patch) -> Vec<String> {
  let diff = "diff".to_string();
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
    args.extend_from_slice(&[diff, format!("{}...{}", parent_ids[0], parent_ids[1])]);
  } else if !commit.parent_ids.is_empty() {
    args.extend_from_slice(&[diff, format!("{}..{}", parent_ids[0], id)]);
  } else {
    args.extend_from_slice(&[diff, format!("{}..{}", COMMIT_0_ID, id)]);
  }

  args.push(dashes);
  args.push(patch.old_file.clone());

  if patch.patch_type == PatchType::R {
    args.push(patch.new_file.clone());
  }

  args
}

// pub fn load_hunks_args(commit: &Commit, patch: &Patch) -> [String; 4] {
//   let diff = "diff".to_string();
//   let dashes = "--".to_string();
//
//   let old_file = patch.old_file.clone();
//
//   let Commit {
//     id,
//     parent_ids,
//     is_merge,
//     ..
//   } = commit;
//
//   if *is_merge {
//     [diff, format!("{}^@", id), dashes, old_file]
//   } else if !commit.parent_ids.is_empty() {
//     [diff, format!("{}..{}", parent_ids[0], id), dashes, old_file]
//   } else {
//     [diff, format!("{}..{}", COMMIT_0_ID, id), dashes, old_file]
//   }
// }

pub fn flatten_hunks(hunks: Vec<Hunk>) -> Vec<HunkLine> {
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
