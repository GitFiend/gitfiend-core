use std::time::Instant;

use crate::git::git_types::Commit;
use crate::git::queries::commits_parsers::{PRETTY_FORMATTED, P_COMMITS};
use crate::git::run_git;
use crate::git::run_git::RunGitOptions;
use crate::parser::parse_all;

pub fn load_stashes(repo_path: &String) -> Option<Vec<Commit>> {
  let now = Instant::now();

  let out = run_git::run_git(RunGitOptions {
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
  });

  println!(
    "Took {}ms to request stashes from Git",
    now.elapsed().as_millis(),
  );

  let mut commits = parse_all(P_COMMITS, out?.as_str())?;

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

  Some(commits)
}

fn tidy_commit_message(message: &String) -> String {
  message
    .split(":")
    .nth(0)
    .unwrap_or("Stash")
    .replace("WIP", "Stash")
}
