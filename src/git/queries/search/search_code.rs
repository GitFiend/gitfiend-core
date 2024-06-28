use crate::dprintln;
use crate::git::git_settings::GIT_PATH;
use crate::git::git_types::{HunkLine, Patch};
use crate::git::queries::patches::patch_parsers::P_MANY_PATCHES_WITH_COMMIT_IDS;
use crate::git::queries::search::matching_hunk_lines::get_matching_hunk_lines;
use crate::git::queries::search::search_cancelled;
use crate::git::store::STORE;
use crate::parser::parse_all;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use ts_rs::TS;

#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CodeSearchOpts {
  pub repo_path: String,
  pub search_text: String,
  pub num_results: usize,
  pub start_commit_index: usize,
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq, TS)]
#[ts(export)]
pub struct FileMatch {
  patch: Patch,
  lines: Vec<HunkLine>,
}

// None result means either no results or cancelled.
pub fn search_commits_for_code(
  options: &CodeSearchOpts,
  search_id: u32,
) -> Option<Vec<(String, Vec<FileMatch>)>> {
  let result_text = search_code_command(options, search_id)?;

  let commit_patches = parse_all(P_MANY_PATCHES_WITH_COMMIT_IDS, &result_text)?;

  let CodeSearchOpts {
    repo_path,
    search_text,
    ..
  } = options;

  let (commits, _) = STORE.get_commits_and_refs(repo_path)?;

  Some(
    commit_patches
      .into_iter()
      .flat_map(|(id, patches)| {
        let commit = commits.iter().find(|c| c.id == id)?;

        let matches = patches
          .into_iter()
          .flat_map(|patch| {
            Some(FileMatch {
              lines: get_matching_hunk_lines(repo_path, commit, &patch, search_text)
                .ok()?,
              patch,
            })
          })
          .collect::<Vec<FileMatch>>();

        Some((commit.id.clone(), matches))
      })
      .collect::<Vec<(String, Vec<FileMatch>)>>(),
  )
}

// Just returns the raw text result from Git.
pub fn search_code_command(options: &CodeSearchOpts, search_id: u32) -> Option<String> {
  dprintln!(
    "Search for text: {}, id: {}, num: {}",
    options.search_text,
    search_id,
    options.num_results
  );

  let CodeSearchOpts {
    repo_path,
    search_text,
    num_results,
    start_commit_index,
  } = options;

  let mut cmd = Command::new(GIT_PATH.as_path())
    .args([
      "log",
      // &format!("{}..{}", last_commit_id, first_commit_id),
      &format!("--skip={}", start_commit_index),
      // &format!("-S\"{}\"", search_text),
      "-S",
      search_text,
      "--name-status",
      "--branches",
      "--remotes",
      "--pretty=format:%H,",
      &format!("-n{}", num_results),
      "-z",
    ])
    .stdout(Stdio::piped())
    .current_dir(repo_path)
    .spawn()
    .ok()?;

  while let Ok(None) = cmd.try_wait() {
    if search_cancelled(search_id) {
      dprintln!("Killing search {search_id} \"{search_text}\"");

      if let Err(_e) = cmd.kill() {
        dprintln!("{}", _e);
      }
      return None;
    }

    thread::sleep(Duration::from_millis(50));
  }

  if cmd.wait().ok()?.success() {
    let mut text = String::new();

    let len = cmd.stdout?.read_to_string(&mut text).ok()?;

    if len > 0 {
      return Some(text);
    }
  }

  None
}
