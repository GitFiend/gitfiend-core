use crate::git::queries::commits::load_commits_and_stashes;
use serde::{Deserialize, Serialize};
use tiny_http::{Request, Response};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqOptions {
  pub repo_path: String,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqCommitsOptions {
  repo_path: String,
  num_commits: u32,
}

pub fn req_commits(mut request: Request) {
  let mut content = String::new();
  request.as_reader().read_to_string(&mut content).unwrap();

  let ReqCommitsOptions {
    repo_path,
    num_commits,
  } = serde_json::from_str(&content).unwrap();

  let result = load_commits_and_stashes(&repo_path, num_commits);

  let serialized = serde_json::to_string(&result).unwrap();

  // TODO: We shouldn't just exit if there's an error.
  request
    .respond(Response::from_string(serialized))
    .expect("req_load_commits result to be sent");
}
