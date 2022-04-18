use crate::load_commits;
use serde::{Deserialize, Serialize};
use tiny_http::{Request, Response};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadCommitsRequestOptions {
  repo_path: String,
  num_commits: u32,
}

pub fn req_load_commits(mut request: Request) {
  let mut content = String::new();
  request.as_reader().read_to_string(&mut content).unwrap();

  let options: LoadCommitsRequestOptions = serde_json::from_str(&content).unwrap();

  let result = load_commits(options.repo_path, options.num_commits);

  let serialized = serde_json::to_string(&result).unwrap();

  request
    .respond(Response::from_string(serialized))
    .expect("req_load_commits result to be sent");
}
