use crate::git::{run_git, RunGitOptions};
use crate::parser::standard_parsers::UNTIL_LINE_END;
use crate::parser::{parse_all, Parser};
use crate::server::git_request::ReqOptions;
use crate::{and, many, until_str, word};
use crate::{map, Input};
use std::collections::HashMap;
use tiny_http::{Request, Response};

pub fn req_config(mut request: Request) {
  let mut content = String::new();
  request.as_reader().read_to_string(&mut content).unwrap();

  let ReqOptions { repo_path } = serde_json::from_str(&content).unwrap();

  let result = load_full_config(&repo_path);

  let serialized = serde_json::to_string(&result).unwrap();

  request
    .respond(Response::from_string(serialized))
    .expect("result to be sent");
}

const P_CONFIG: Parser<HashMap<String, String>> = map!(
  many!(and!(until_str!("="), UNTIL_LINE_END)),
  |result: Vec<(String, String)>| { result.into_iter().collect() }
);

const P_REMOTE_NAME: Parser<String> = map!(
  and!(
    word!("remote."),
    until_str!("."),
    word!("url"),
    UNTIL_LINE_END
  ),
  |result: (&str, String, &str, String)| { result.1 }
);

pub fn load_full_config(repo_path: &String) -> Option<HashMap<String, String>> {
  let result = run_git(RunGitOptions {
    repo_path,
    args: ["config", "--list"],
  });

  parse_all(P_CONFIG, result?.as_str())
}

#[cfg(test)]
mod tests {
  use crate::git::queries::config::{load_full_config, P_CONFIG, P_REMOTE_NAME};
  use crate::parser::parse_all;

  #[test]
  fn load_config() {
    let result = load_full_config(&".".to_string());

    assert!(result.is_some());
    assert!(result.unwrap().len() > 0);
  }

  #[test]
  fn test_p_config() {
    let config = "credential.helper=osxkeychain
user.email=something@gmail.com
user.name=username
filter.lfs.clean=git-lfs clean -- %f
filter.lfs.smudge=git-lfs smudge -- %f
filter.lfs.process=git-lfs filter-process
filter.lfs.required=true
credential.helper=osxkeychain
core.repositoryformatversion=0
core.filemode=true
core.bare=false
core.logallrefupdates=true
core.ignorecase=true
core.precomposeunicode=true
remote.origin.url=https://gitlab.com/torquescrew/test-repo.git
remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
branch.master.remote=origin
branch.master.merge=refs/heads/master
branch.branch1.remote=origin
branch.branch1.merge=refs/heads/branch1
branch.other-branch.remote=origin
branch.other-branch.merge=refs/heads/other-branch
branch.left.remote=origin
branch.left.merge=refs/heads/left
branch.right.remote=origin
branch.right.merge=refs/heads/right
branch.a.remote=origin
branch.a.merge=refs/heads/a
remote.origin2.url=/Users/toby/Repos/test-repo-remote
remote.origin2.fetch=+refs/heads/*:refs/remotes/origin2/*
";
    let result = parse_all(P_CONFIG, config);
    assert!(result.is_some());

    let c = result.unwrap();

    assert_eq!(c.get("user.name").unwrap(), "username");
    assert_eq!(
      c.get("remote.origin2.fetch").unwrap(),
      "+refs/heads/*:refs/remotes/origin2/*"
    );
  }

  #[test]
  fn test_p_remote_name() {
    let result = parse_all(
      P_REMOTE_NAME,
      "remote.origin2.url=/Users/toby/Repos/test-repo-remote",
    );

    assert!(result.is_some());
    assert_eq!(result.unwrap(), "origin2");
  }
}
