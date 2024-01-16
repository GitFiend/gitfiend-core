use crate::git::run_git_action::run_git_action_with_vec;
use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GitAddOptions {
  pub repo_path: String,
  pub files: Vec<String>,
}

pub fn git_add_files(options: &GitAddOptions) -> u32 {
  let GitAddOptions { repo_path, files } = options;

  let commands: Vec<Vec<String>> = files
    .chunks(10_1000)
    .map(|chunk| {
      let mut command = vec![String::from("add")];
      command.extend(chunk.iter().map(|f| f.to_string()));
      command
    })
    .collect();

  run_git_action_with_vec(repo_path, commands)
}
