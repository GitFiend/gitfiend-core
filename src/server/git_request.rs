use serde::{Deserialize, Serialize};
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
  pub repo_path: String,
  pub num_commits: u32,
}

#[macro_export]
macro_rules! parse_json {
  ($request: expr) => {{
    let mut content = String::new();
    $request.as_reader().read_to_string(&mut content).unwrap();

    match serde_json::from_str(&content) {
      Ok(options) => options,
      Err(e) => {
        println!("{}", e);
        None
      }
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

#[macro_export]
macro_rules! handle_request {
  ($request:expr, $handler: ident) => {{
    match parse_json!($request) {
      Some(options) => {
        send_response!($request, $handler(&options));
      }
      None => {}
    };
  }};
}

// pub fn req_commits(mut request: Request) {
//   // let options = parse_json!(request);
//   //
//   // if options.is_some() {
//   //   let ReqCommitsOptions {
//   //     repo_path,
//   //     num_commits,
//   //   } = options.unwrap();
//   //
//   //   send_response!(request, load_commits_and_stashes(&repo_path, num_commits));
//   // }
//
//   match parse_json!(request) {
//     Some(options) => {
//       send_response!(request, load_commits_and_stashes(&options));
//     }
//     None => {}
//   };
// }
