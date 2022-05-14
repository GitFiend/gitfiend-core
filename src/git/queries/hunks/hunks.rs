use crate::git::git_types::{Commit, Hunk, HunkLine, HunkLineStatus, Patch};
use crate::git::queries::hunks::hunk_parsers::P_HUNKS;
use crate::git::queries::COMMIT_0_ID;
use crate::git::store::Store;
use crate::git::{run_git, RunGitOptions};
use crate::parser::parse_all;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqHunkOptions {
  pub repo_path: String,
  pub commit: Commit,
  pub patch: Patch,
}

pub fn load_hunks(
  options: &ReqHunkOptions,
  store: Arc<RwLock<Store>>,
) -> Option<(Vec<Hunk>, Vec<HunkLine>)> {
  let out = run_git(RunGitOptions {
    repo_path: &options.repo_path,
    args: load_hunks_args(&options),
  })?;

  let hunks = parse_all(P_HUNKS, &out)?;
  let hunk_lines = flatten_hunks(&hunks);

  Some((hunks, hunk_lines))
}

pub fn load_hunks_args(options: &ReqHunkOptions) -> [String; 4] {
  let diff = "diff".to_string();
  let dashes = "--".to_string();

  let ReqHunkOptions { commit, patch, .. } = options;
  let old_file = patch.old_file.clone();

  let Commit {
    id,
    parent_ids,
    is_merge,
    ..
  } = commit;

  if *is_merge {
    [diff, format!("{}^@", id), dashes, old_file]
  } else if commit.parent_ids.len() > 0 {
    [diff, format!("{}..{}", parent_ids[0], id), dashes, old_file]
  } else {
    [diff, format!("{}..{}", COMMIT_0_ID, id), dashes, old_file]
  }
}

fn flatten_hunks(hunks: &Vec<Hunk>) -> Vec<HunkLine> {
  let mut lines: Vec<HunkLine> = Vec::new();

  if hunks.len() == 0 {
    return lines;
  }

  for hunk in hunks.iter() {
    lines.push(HunkLine::header_from_type(
      HunkLineStatus::HeaderStart,
      hunk.index,
    ));
    lines.push(HunkLine::header_from_type(
      HunkLineStatus::HeaderEnd,
      hunk.index,
    ));

    for line in hunk.lines.iter() {
      lines.push(line.clone());
    }
  }

  lines.push(HunkLine::header_from_type(HunkLineStatus::HeaderStart, -1));

  lines
}
