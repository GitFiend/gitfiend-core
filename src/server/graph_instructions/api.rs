use crate::git::git_types::Commit;
use crate::git::store::{load_commits_from_store, RwStore};
use crate::server::graph_instructions::instruction_types::Instructions;
use crate::server::graph_instructions::GraphInstructions;
use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GraphInstructionOpts {
  pub repo_path: String,
  pub commit_ids: Vec<String>,
  pub all_ids: bool,
}

pub fn graph_instructions(options: &GraphInstructionOpts, store: RwStore) -> Option<Instructions> {
  let GraphInstructionOpts {
    commit_ids,
    repo_path,
    all_ids,
  } = options;

  let now = Instant::now();

  let commits: AHashMap<String, Commit> = load_commits_from_store(&repo_path, &store)?
    .into_iter()
    .map(|c| (c.id.clone(), c))
    .collect();

  let ids = if *all_ids {
    commits.keys().map(|id| id.clone()).collect::<Vec<String>>()
  } else {
    commit_ids.clone()
  };

  let mut i = GraphInstructions::new(ids.len());

  i.generate(&ids, &commits);

  println!(
    "Took {}ms for graph_instructions",
    now.elapsed().as_millis(),
  );

  Some(Instructions {
    points: i.points,
    lines: i.lines,
  })
}
