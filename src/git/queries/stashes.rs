use loggers::elapsed;

use crate::git::git_types::CommitInfo;
use crate::git::queries::commits_parsers::{PRETTY_FORMATTED, P_COMMITS};
use crate::git::run_git;
use crate::git::run_git::RunGitOptions;
use crate::git::store::RepoPath;
use crate::parser::parse_all_err;
use crate::server::request_util::R;

#[elapsed]
pub fn load_stashes(repo_path: &RepoPath) -> R<Vec<CommitInfo>> {
  let out = run_git::run_git_bstr(RunGitOptions {
    args: [
      "reflog",
      "show",
      "stash",
      "-z",
      "--decorate=full",
      PRETTY_FORMATTED,
      "--date=raw",
    ],
    repo_path,
  })?
  .stdout;

  let mut commits = parse_all_err(P_COMMITS, &out)?;

  for i in 0..commits.len() {
    let mut c = &mut commits[i];
    c.stash_id = Some(format!("refs/stash@{{{}}}", i));
    c.is_merge = false;
    c.refs.clear();

    while c.parent_ids.len() > 1 {
      c.parent_ids.pop();
    }

    c.message = tidy_commit_message(&c.message)
  }

  Ok(commits)
}

fn tidy_commit_message(message: &str) -> String {
  message
    .split(':')
    .next()
    .unwrap_or("Stash")
    .replace("WIP", "Stash")
}
