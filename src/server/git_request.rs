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

#[macro_export]
macro_rules! parse_json {
  ($request: expr) => {{
    let mut content = String::new();
    $request.as_reader().read_to_string(&mut content).unwrap();
    let result = serde_json::from_str(&content);

    if result.is_ok() {
      Some(result.unwrap())
    } else {
      // println!("{}", result.err()) // TODO
      None
    }
  }};
}

#[macro_export]
macro_rules! send_response {
  ($request: expr, $result: expr) => {{
    let serialized = serde_json::to_string(&$result).unwrap();

    match $request.respond(Response::from_string(serialized)) {
      Ok(_) => {}
      Err(e) => {
        println!("{}", e);
      }
    };
  }};
}

pub fn req_commits(mut request: Request) {
  // let options = parse_json!(request);
  //
  // if options.is_some() {
  //   let ReqCommitsOptions {
  //     repo_path,
  //     num_commits,
  //   } = options.unwrap();
  //
  //   send_response!(request, load_commits_and_stashes(&repo_path, num_commits));
  // }

  match parse_json!(request) {
    Some(ReqCommitsOptions {
      repo_path,
      num_commits,
    }) => {
      send_response!(request, load_commits_and_stashes(&repo_path, num_commits));
    }
    None => {}
  };
}
